use std::thread::sleep;
use std::time::Duration;

use beagle_linux_sdk::boards::pocketbeagle2::P1_20;
use beagle_linux_sdk::Led;

fn main() {
    let led = Led::new(P1_20).expect("Failed to open LED PIN");

    loop {
        println!("ON");
        led.on().unwrap();
        sleep(Duration::from_secs(1));

        println!("OFF");
        led.off().unwrap();
        sleep(Duration::from_secs(1));
    }
}
