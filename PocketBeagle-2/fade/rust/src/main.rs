use std::thread::sleep;
use std::time::Duration;

use beagle_helper::boards::pocketbeagle2::P1_36;
use beagle_helper::PwmLed;

fn main() {
    const PERIOD: usize = 255;
    const DELAY: Duration = Duration::from_millis(50);

    let led = PwmLed::new(P1_36).unwrap();

    loop {
        for i in (5..(PERIOD + 1)).step_by(5) {
            led.start(i, PERIOD).unwrap();
            sleep(DELAY);
        }

        for i in (0..(PERIOD - 4)).step_by(5).rev() {
            led.start(i, PERIOD).unwrap();
            sleep(DELAY);
        }
    }
}
