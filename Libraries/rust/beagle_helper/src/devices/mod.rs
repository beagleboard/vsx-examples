#[cfg(feature = "beagle_eeprom")]
pub mod beagle_eeprom;
#[cfg(feature = "tonal_buzzer")]
pub mod tonal_buzzer;

#[cfg(feature = "beagle_eeprom")]
pub use beagle_eeprom::*;
#[cfg(feature = "tonal_buzzer")]
pub use tonal_buzzer::TonalBuzzer;
