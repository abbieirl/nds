#![no_std]
#![no_main]

use anyhow::Result;
use nds::sys::background::{Size, Type};
use nds::sys::console::Console;
use nds::sys::input::Keys;
use nds::sys::video::Mode;

#[nds::entry]
fn main() -> Result<()> {
    nds::sys::video::set_mode(Mode::Zero2D);
    nds::sys::video::set_mode_sub(Mode::Zero2D);

    let console_top = Console::top(3, Type::Text4Bpp, Size::T256x256);
    let console_bottom = Console::bottom(3, Type::Text4Bpp, Size::T256x256);

    loop {
        nds::sys::interrupt::swi_wait_for_vblank();
        Keys::scan();

        if Keys::down().contains(Keys::START) {
            break;
        }
    }

    Ok(())
}
