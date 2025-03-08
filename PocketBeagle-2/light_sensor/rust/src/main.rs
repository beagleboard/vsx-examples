use std::thread::sleep;
use std::time::Duration;

use beagle_helper::boards::pocketbeagle2::P1_19;
use beagle_helper::LightSensor;

fn main() {
    let mut ldr = LightSensor::new(P1_19, None).unwrap();

    loop {
        if ldr.value().unwrap() {
            println!("Light");
        } else {
            println!("Dark");
        }

        sleep(Duration::from_millis(500));
    }
}
