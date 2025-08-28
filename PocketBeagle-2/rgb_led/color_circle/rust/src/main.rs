//! This example cycles through the base colors in the Color circle.

use std::{io::Write, thread::sleep, time::Duration};

use beagle_helper::sysfs::Device;

const LED: &str = "/sys/devices/platform/techlab-led/leds/multi-led/";
const DELAY: Duration = Duration::from_secs(1);

fn main() {
    let led = Device::with_path(LED).unwrap();
    let mut multi_intensity = led.sysfs_w("multi_intensity").unwrap();

    let max_brightness = led
        .sysfs_r("max_brightness")
        .unwrap()
        .read_string()
        .unwrap();
    // Set brightness to max from the start.
    led.sysfs_w("brightness")
        .unwrap()
        .write_all(max_brightness.as_bytes())
        .unwrap();

    loop {
        multi_intensity.write_all(b"255 0 0").unwrap();
        sleep(DELAY);

        multi_intensity.write_all(b"0 255 0").unwrap();
        sleep(DELAY);

        multi_intensity.write_all(b"0 0 255").unwrap();
        sleep(DELAY);
    }
}
