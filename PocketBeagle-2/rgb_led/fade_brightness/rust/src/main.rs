//! This example demonstrates fading a single color in and out in an RGB LED using the brightness
//! setting.

use std::{io::Write, thread::sleep, time::Duration};

use beagle_helper::sysfs::Device;

const LED: &str = "/sys/devices/platform/techlab-led/leds/rgb:/";
const DELAY: Duration = Duration::from_millis(50);

fn main() {
    let led = Device::with_path(LED).unwrap();

    let max_brightness = led.sysfs_r("max_brightness").unwrap().read_usize().unwrap();

    // Set intensity to a single color
    led.sysfs_w("multi_intensity")
        .unwrap()
        .write_all(b"255 0 0")
        .unwrap();

    let mut brightness = led.sysfs_w("brightness").unwrap();

    loop {
        for i in (5..(max_brightness + 1)).step_by(5) {
            brightness.write_string(i).unwrap();
            sleep(DELAY);
        }

        for i in (0..(max_brightness - 4)).step_by(5).rev() {
            brightness.write_string(i).unwrap();
            sleep(DELAY);
        }
    }
}
