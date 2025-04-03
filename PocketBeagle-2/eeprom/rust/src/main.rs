use beagle_helper::boards::pocketbeagle2::EEPROM_PATH;
use beagle_helper::BeagleEeprom;

fn main() {
    let contents = BeagleEeprom::read(EEPROM_PATH).unwrap();
    println!("{:#?}", contents);
}
