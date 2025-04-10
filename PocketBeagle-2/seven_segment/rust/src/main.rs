use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use beagle_helper::sysfs::Device;

pub const SEVEN_SEGMENT_LEFT: &str = "/sys/devices/platform/seven-segments-left/linedisp.1/";
pub const SEVEN_SEGMENT_RIGHT: &str = "/sys/devices/platform/seven-segments-right/linedisp.0/";

fn main() {
    let segment_left = Device::with_path(SEVEN_SEGMENT_LEFT).unwrap();
    let segment_right = Device::with_path(SEVEN_SEGMENT_RIGHT).unwrap();

    let mut left_msg = segment_left.sysfs_w("message").unwrap();
    let mut right_msg = segment_right.sysfs_w("message").unwrap();

    segment_left
        .sysfs_w("scroll_step_ms")
        .unwrap()
        .write_string(1000)
        .unwrap();
    segment_right
        .sysfs_w("scroll_step_ms")
        .unwrap()
        .write_string(1000)
        .unwrap();

    println!("Countdown");
    left_msg.write_all(b"10000000000").unwrap();
    right_msg.write_all(b"09876543210").unwrap();

    sleep(Duration::from_secs(11));

    left_msg.write_all(b" ").unwrap();
    right_msg.write_all(b" ").unwrap();
}
