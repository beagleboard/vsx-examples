use std::thread::sleep;
use std::time::Duration;

use beagle_helper::boards::pocketbeagle2::USR_LED4;
use beagle_helper::Led;

fn main() {
    let mut led = Led::new(USR_LED4).expect("Failed to open LED PIN");

    loop {
        println!("ON");
        led.on().unwrap();
        sleep(Duration::from_secs(1));

        println!("OFF");
        led.off().unwrap();
        sleep(Duration::from_secs(1));
    }
}
