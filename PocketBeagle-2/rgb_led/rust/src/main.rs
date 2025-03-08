use std::thread::sleep;
use std::time::Duration;

use beagle_helper::boards::pocketbeagle2::techlab;
use beagle_helper::RgbLed;

const UPDATE_INTERVAL: Duration = Duration::from_millis(10);
const BRIGHTNESS: usize = 255;

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
    let mut led = RgbLed::new(techlab::LED).unwrap();
    led.set_brightness(BRIGHTNESS).unwrap();

    loop {
        for i in 0..256 {
            led.set_color(wheel(i)).unwrap();
            sleep(UPDATE_INTERVAL);
        }
    }
}
