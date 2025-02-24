#[cfg(feature = "button")]
pub mod button;
#[cfg(feature = "led")]
pub mod led;
#[cfg(feature = "pwm_led")]
pub mod pwm_led;
#[cfg(feature = "beagle_eeprom")]
pub mod beagle_eeprom;

#[cfg(feature = "button")]
pub use button::Button;
#[cfg(feature = "led")]
pub use led::Led;
#[cfg(feature = "pwm_led")]
pub use pwm_led::PwmLed;
#[cfg(feature = "beagle_eeprom")]
pub use beagle_eeprom::*;
