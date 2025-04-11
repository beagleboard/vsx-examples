//! This example demonstrates reading button inputs using GPIO Keys driver.

use beagle_helper::chardev::CharDev;
use std::{ffi, fs::OpenOptions, io::Read};

const BUTTONS_NAME: &str = "buttons";

#[repr(C)]
struct Timeval {
    tv_sec: i64,
    tv_usec: i64,
}

/// Defined in [Event Interface](https://docs.kernel.org/input/input.html#event-interface)
#[repr(C)]
struct InputEvent {
    time: Timeval,
    r#type: ffi::c_ushort,
    code: ffi::c_ushort,
    value: ffi::c_int,
}

impl InputEvent {
    fn read(dev: &mut CharDev) -> std::io::Result<Self> {
        let mut data = [0u8; std::mem::size_of::<Self>()];
        dev.read_exact(&mut data)?;

        // Reinterprets the bits from CharDev as [InputEvent]
        Ok(unsafe { std::mem::transmute(data) })
    }
}

fn main() {
    let mut evt =
        CharDev::open_input_with_name(BUTTONS_NAME, OpenOptions::new().read(true)).unwrap();

    loop {
        let inp = InputEvent::read(&mut evt).unwrap();

        match (inp.code, inp.value) {
            // We use value to only print on pressed (not released)
            (106, 1) => println!("Right"),
            _ => {}
        }
    }
}
