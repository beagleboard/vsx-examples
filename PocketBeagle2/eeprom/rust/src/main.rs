use beagle_linux_sdk::boards::pocketbeagle2::EEPROM_PATH;
use beagle_linux_sdk::BeagleEeprom;

fn main() {
    let contents = BeagleEeprom::read(EEPROM_PATH).unwrap();
    println!("{:#?}", contents);
}
