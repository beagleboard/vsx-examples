use std::{thread::sleep, time::Duration};

use linux_embedded_hal::{
    gpio_cdev::{self, Line, LineRequestFlags},
    CdevPin,
};

/// PIN name
const PIN: &str = "P1_20";

/// Find line by PIN Name.
fn find_line_by_name(name: &str) -> Line {
    let chips = gpio_cdev::chips()
        .expect("Failed to get line names")
        .filter_map(|x| x.ok());

    for chip in chips {
        for line in chip.lines() {
            match line.info() {
                Ok(info) if info.name() == Some(name) => return line,
                _ => {}
            }
        }
    }

    panic!("PIN {} not found", name);
}

fn main() {
    let line = find_line_by_name(PIN);
    let line_handle = line.request(LineRequestFlags::OUTPUT, 0, "blinky").unwrap();
    let pin = CdevPin::new(line_handle).unwrap();

    loop {
        println!("ON");
        pin.set_value(1).unwrap();
        sleep(Duration::from_secs(1));

        println!("OFF");
        pin.set_value(0).unwrap();
        sleep(Duration::from_secs(1));
    }
}
