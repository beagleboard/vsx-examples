#[cfg(feature = "beagle_eeprom")]
pub mod beagle_eeprom;
#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "tonal_buzzer")]
pub mod tonal_buzzer;

#[cfg(feature = "beagle_eeprom")]
pub use beagle_eeprom::*;
#[cfg(feature = "button")]
pub use button::Button;
#[cfg(feature = "tonal_buzzer")]
pub use tonal_buzzer::TonalBuzzer;
