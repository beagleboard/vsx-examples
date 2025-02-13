use std::thread::sleep;
use std::time::Duration;

use linux_embedded_hal::gpio_cdev;

/// PIN name
const PIN: &str = "P1_20";

/// Find line by PIN Name.
fn find_line_by_name(name: &str) -> gpio_cdev::Line {
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
    let line_handle = line
        .request(gpio_cdev::LineRequestFlags::OUTPUT, 0, "blinky")
        .unwrap();

    loop {
        println!("ON");
        line_handle.set_value(1).unwrap();
        sleep(Duration::from_secs(1));

        println!("OFF");
        line_handle.set_value(0).unwrap();
        sleep(Duration::from_secs(1));
    }
}
