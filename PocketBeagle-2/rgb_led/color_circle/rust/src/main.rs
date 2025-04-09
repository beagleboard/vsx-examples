//! This example cycles through all the base colors in the Color circle.

use std::{thread::sleep, time::Duration};

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
        .write(max_brightness)
        .unwrap();

    loop {
        multi_intensity.write("255 0 0").unwrap();
        sleep(DELAY);

        multi_intensity.write("255 255 0").unwrap();
        sleep(DELAY);

        multi_intensity.write("0 255 0").unwrap();
        sleep(DELAY);

        multi_intensity.write("0 255 255").unwrap();
        sleep(DELAY);

        multi_intensity.write("0 0 255").unwrap();
        sleep(DELAY);

        multi_intensity.write("255 0 255").unwrap();
        sleep(DELAY);

        multi_intensity.write("255 255 255").unwrap();
        sleep(DELAY);

        multi_intensity.write("0 0 0").unwrap();
        sleep(DELAY);
    }
}
