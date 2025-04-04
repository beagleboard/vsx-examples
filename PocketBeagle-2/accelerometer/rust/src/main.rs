use std::{thread::sleep, time::Duration};

use beagle_helper::sysfs;

const DEV_NAME: &str = "mma8453";

fn main() {
    let accl = sysfs::Device::with_dev_name(DEV_NAME).unwrap();

    let scale = accl.sysfs_r("in_accel_scale").unwrap().read_f64().unwrap();
    let mut x_raw = accl.sysfs_r("in_accel_x_raw").unwrap();
    let mut y_raw = accl.sysfs_r("in_accel_y_raw").unwrap();
    let mut z_raw = accl.sysfs_r("in_accel_z_raw").unwrap();

    loop {
        println!(
            "Acceleration along X = {:.2} ms^2, Y = {:.2} ms^2, Z = {:.2} ms^2",
            x_raw.read_f64().unwrap() * scale,
            y_raw.read_f64().unwrap() * scale,
            z_raw.read_f64().unwrap() * scale,
        );

        sleep(Duration::from_secs(1));
    }
}
