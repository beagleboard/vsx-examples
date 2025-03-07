use std::thread::sleep;
use std::time::Duration;

use beagle_linux_sdk::boards::pocketbeagle2::USER_LED4;
use beagle_linux_sdk::Led;

fn main() {
    let mut led = Led::new(USER_LED4).expect("Failed to open LED PIN");

    loop {
        println!("ON");
        led.on().unwrap();
        sleep(Duration::from_secs(1));

        println!("OFF");
        led.off().unwrap();
        sleep(Duration::from_secs(1));
    }
}
