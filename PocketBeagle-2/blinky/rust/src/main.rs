use std::thread::sleep;
use std::time::Duration;

use beagle_helper::sysfs;

const LED: &str = "/sys/class/leds/beaglebone:green:usr4";

fn main() {
    let led = sysfs::Device::with_path(LED).unwrap();

    // Technically, max_brightness will be an unsigned integer value. However, since we never
    // actually parse it, and writing to the file needs conversion back to string anyway, it is
    // better to just keep it as string
    let max_brightness = led
        .sysfs_r("max_brightness")
        .unwrap()
        .read_string()
        .unwrap();
    let mut brightness = led.sysfs_w("brightness").unwrap();

    loop {
        println!("ON");
        brightness.write_string(&max_brightness).unwrap();
        sleep(Duration::from_secs(1));

        println!("OFF");
        brightness.write_string("0").unwrap();
        sleep(Duration::from_secs(1));
    }
}
