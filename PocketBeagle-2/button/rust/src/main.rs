//! This example demonstrates reading button inputs using GPIO Keys driver.

use beagle_helper::chardev::CharDev;
use std::fs::OpenOptions;

const BUTTONS_NAME: &str = "buttons";

fn main() {
    let mut evt =
        CharDev::open_input_with_name(BUTTONS_NAME, OpenOptions::new().read(true)).unwrap();

    loop {
        let inp = evt.read_evt().unwrap();
        match (inp.code, inp.value) {
            // We use value to only print on pressed (not released)
            (106, 1) => println!("Right"),
            _ => {}
        }
    }
}
