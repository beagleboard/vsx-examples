use std::{thread::sleep, time::Duration};

use beagle_helper::{boards::pocketbeagle2::techlab, Accel};

fn main() {
    let accl = Accel::new(techlab::ACCELEROMETER).unwrap();

    loop {
        println!(
            "X = {}, Y = {}, Z = {}",
            accl.x_raw().unwrap(),
            accl.y_raw().unwrap(),
            accl.z_raw().unwrap()
        );

        sleep(Duration::from_secs(1));
    }
}
