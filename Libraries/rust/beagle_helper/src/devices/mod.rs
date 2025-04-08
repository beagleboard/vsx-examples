#[cfg(feature = "beagle_eeprom")]
pub mod beagle_eeprom;
#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "rgb_led")]
pub mod rgb_led;
#[cfg(feature = "tonal_buzzer")]
pub mod tonal_buzzer;

#[cfg(feature = "beagle_eeprom")]
pub use beagle_eeprom::*;
#[cfg(feature = "button")]
pub use button::Button;
#[cfg(feature = "rgb_led")]
pub use rgb_led::RgbLed;
#[cfg(feature = "tonal_buzzer")]
pub use tonal_buzzer::TonalBuzzer;
