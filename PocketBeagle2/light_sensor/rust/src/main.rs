use beagle_linux_sdk::boards::pocketbeagle2::P1_19;
use beagle_linux_sdk::LightSensor;

fn main() {
    let mut ldr = LightSensor::new(P1_19, None).unwrap();

    loop {
        println!("Value: {}", ldr.value().unwrap());
    }
}
