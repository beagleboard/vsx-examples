use std::thread::sleep;
use std::time::Duration;

/// PIN name
const PIN: &str = "P1_20";

/// Find line by PIN Name.
fn find_line_by_name(name: &str) -> (gpiod::Chip, u32) {
    let chips = gpiod::Chip::list_devices().expect("Failed to get line names");

    for chip_path in chips {
        let chip = gpiod::Chip::new(chip_path).unwrap();
        let num_lines = chip.num_lines();

        for line in 0..num_lines {
            match chip.line_info(line) {
                Ok(info) if info.name == name => return (chip, line),

                _ => {}
            }
        }
    }

    panic!("PIN {} not found", name);
}

fn main() {
    let (chip, offset) = find_line_by_name(PIN);
    let opts = gpiod::Options::output([offset]).consumer("blinky");
    let pin = chip.request_lines(opts).unwrap();

    loop {
        println!("ON");
        pin.set_values([true]).unwrap();
        sleep(Duration::from_secs(1));

        println!("OFF");
        pin.set_values([false]).unwrap();
        sleep(Duration::from_secs(1));
    }
}
