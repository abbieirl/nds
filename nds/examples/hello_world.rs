#![no_std]
#![no_main]

use anyhow::Result;
use core::fmt::Write;
use nds::input::Keys;
use nds::io::Console;

#[nds::entry]
fn main() -> Result<()> {
    let mut console = Console::demo();

    writeln!(console, "\x1b[2J")?;
    writeln!(console, "\x1b[8;10HHello World!")?;
    writeln!(console, "\x1b[8ALine 0")?;
    writeln!(console, "\x1b[28DColumn 0")?;
    writeln!(console, "\x1b[19BLine 19")?;
    writeln!(console, "\x1b[5CColumn 20")?;
    writeln!(console, "\x1b[14;4H")?;

    for (c, i) in (b'A'..=b'Z').zip(30..) {
        let color = 30 + (i % 8);
        let intensity = i / 8;
        write!(console, "\x1b[{};{}m{}", color, intensity, c as char)?;
    }

    writeln!(console, "\x1b[39;0m")?;
    writeln!(console, "\x1b[23;0HPress START to exit to loader")?;

    loop {
        nds::interrupt::swi_wait_for_vblank();
        Keys::scan();

        if Keys::down().contains(Keys::START) {
            break;
        }
    }

    Ok(())
}
