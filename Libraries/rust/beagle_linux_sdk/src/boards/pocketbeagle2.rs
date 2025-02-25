use crate::pin::{AdcPin, GpioPin, Pin, PwmPin};

pub const P1_19: Pin = Pin::new(None, None, Some(AdcPin::new(0, 0)));
pub const P1_20: Pin = Pin::new(Some(GpioPin::new(2, 50)), None, None);
pub const P1_36: Pin = Pin::new(None, Some(PwmPin::new(0, 0)), None);

pub const P2_33: Pin = Pin::new(Some(GpioPin::new(2, 52)), None, None);

pub const EEPROM_PATH: &str = "/sys/bus/i2c/devices/0-0050/eeprom";
