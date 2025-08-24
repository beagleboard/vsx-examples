//! This example generates different color hue on the RGB LED.

use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use beagle_helper::sysfs::Device;

const LED: &str = "/sys/devices/platform/techlab-led/leds/multi-led/";
const DELAY: Duration = Duration::from_millis(10);

/// Helper function to convert hue to RGB
const fn wheel(pos: usize) -> (usize, usize, usize) {
    if pos < 85 {
        (255 - pos * 3, 0, pos * 3)
    } else if pos < 170 {
        let temp = pos - 85;
        (0, temp * 3, 255 - temp * 3)
    } else {
        let temp = pos - 170;
        (temp * 3, 255 - temp * 3, 0)
    }
}

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
        for i in 0..256 {
            let hue = wheel(i);
            multi_intensity
                .write_all(format!("{} {} {}", hue.0, hue.1, hue.2).as_bytes())
                .unwrap();
            sleep(DELAY);
        }
    }
}
