use beagle_linux_sdk::boards::pocketbeagle2::P2_33;
use beagle_linux_sdk::Button;

fn main() {
    let mut btn = Button::new(P2_33).expect("Failed to open button");

    println!("Waiting for button press");
    btn.wait_for_press().unwrap();
    println!("Button Pressed");
}
