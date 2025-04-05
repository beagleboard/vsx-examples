use std::thread::sleep;
use std::time::Duration;

use beagle_helper::sysfs;

/// Reading values from ADC directly
const DEV_NAME: &str = "ad7291";

const THRESHOLD: f64 = 2000.0;

fn main() {
    let adc = sysfs::Device::with_dev_name(DEV_NAME).unwrap();

    let mut ldr_raw = adc.sysfs_r("in_voltage0_raw").unwrap();
    let scale = adc.sysfs_r("in_voltage_scale").unwrap().read_f64().unwrap();

    loop {
        let scaled = ldr_raw.read_f64().unwrap() * scale;

        if scaled > THRESHOLD {
            println!("Light");
        } else {
            println!("Dark");
        }

        sleep(Duration::from_millis(500));
    }
}
