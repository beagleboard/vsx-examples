use std::{thread::sleep, time::Duration};

use beagle_linux_sdk::{
    boards::pocketbeagle2::techlab::{SEVEN_SEGMENT_LEFT, SEVEN_SEGMENT_RIGHT},
    SevenSegment,
};

fn main() {
    let mut segment_left = SevenSegment::new(SEVEN_SEGMENT_LEFT).unwrap();
    let mut segment_right = SevenSegment::new(SEVEN_SEGMENT_RIGHT).unwrap();

    println!("Countdown Automatic on Right");
    segment_right.set_step(Duration::from_secs(1)).unwrap();
    segment_right.set_message("9876543210").unwrap();

    println!("Countdown Manual on Left");
    for i in (0..10).rev() {
        segment_left.set_message(i.to_string().as_str()).unwrap();
        sleep(Duration::from_secs(1));
    }

    segment_right.set_message(" ").unwrap();
    segment_left.set_message(" ").unwrap();
}
