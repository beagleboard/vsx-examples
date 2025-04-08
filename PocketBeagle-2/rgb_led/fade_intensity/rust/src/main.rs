//! This example demonstrates fading a single color in and out in an RGB LED using the intensity
//! setting.

use std::{thread::sleep, time::Duration};

use beagle_helper::sysfs::Device;

const LED: &str = "/sys/devices/platform/techlab-led/leds/multi-led/";
const DELAY: Duration = Duration::from_millis(50);

const MAX_INTENSITY: usize = 255;

fn main() {
    let led = Device::with_path(LED).unwrap();

    let mut multi_intensity = led.sysfs_w("multi_intensity").unwrap();

    let max_brightness = led
        .sysfs_r("max_brightness")
        .unwrap()
        .read_string()
        .unwrap();
    // Set brightness to max from the start
    led.sysfs_w("brightness")
        .unwrap()
        .write(max_brightness)
        .unwrap();

    // Set intensity to a single color
    led.sysfs_w("multi_intensity")
        .unwrap()
        .write("255 0 0")
        .unwrap();

    loop {
        for i in (5..(MAX_INTENSITY + 1)).step_by(5) {
            multi_intensity.write(format!("{i} 0 0")).unwrap();
            sleep(DELAY);
        }

        for i in (0..(MAX_INTENSITY - 4)).step_by(5).rev() {
            multi_intensity.write(format!("{i} 0 0")).unwrap();
            sleep(DELAY);
        }
    }
}
