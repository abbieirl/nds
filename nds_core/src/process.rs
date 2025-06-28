//! A module for working with processes.

pub trait Termination {
    fn status(&self) -> i32;
}

impl Termination for () {
    fn status(&self) -> i32 {
        Status::Success as i32
    }
}

impl<T: Termination, E> Termination for Result<T, E> {
    fn status(&self) -> i32 {
        match self {
            Ok(t) => t.status(),
            Err(_) => Status::Failure.into(),
        }
    }
}

#[repr(i32)]
pub enum Status {
    Success = 0,
    Failure = 1,
}

impl From<Status> for i32 {
    fn from(status: Status) -> Self {
        status as i32
    }
}

#[cfg(feature = "panic")]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    use crate::console::Console;
    use core::fmt::Write;

    let mut console = Console::demo();
    console.clear();

    writeln!(console, "\x1b[0;0H\x1b[31mPanic!\x1b[39;0m").ok();

    if let Some(location) = info.location() {
        writeln!(
            console,
            "\x1b[34m{}:{}\x1b[39;0m",
            location.file(),
            location.line()
        )
        .ok();
    }

    writeln!(console, "{}", info.message()).ok();

    loop {
        crate::interrupt::swi_wait_for_vblank();
    }
}
