use crate::pin::{AdcPin, GpioPin, Pin, PwmPin};

pub const P1_19: Pin = Pin::new(None, None, Some(AdcPin::new(0, 0)));
pub const P1_20: Pin = Pin::new(Some(GpioPin::new(2, 50)), None, None);
pub const P1_36: Pin = Pin::new(None, Some(PwmPin::new(0, 0)), None);

pub const P2_30: Pin = Pin::new(None, Some(PwmPin::new(2, 0)), None);
pub const P2_33: Pin = Pin::new(Some(GpioPin::new(2, 52)), None, None);

pub const EEPROM_PATH: &str = "/sys/bus/i2c/devices/0-0050/eeprom";
pub const USR_LED1: &str = "/sys/class/leds/beaglebone:green:usr1";
pub const USR_LED2: &str = "/sys/class/leds/beaglebone:green:usr2";
pub const USR_LED3: &str = "/sys/class/leds/beaglebone:green:usr3";
pub const USR_LED4: &str = "/sys/class/leds/beaglebone:green:usr4";

pub mod techlab {
    use crate::pin::Pin;

    pub const SEVEN_SEGMENT_LEFT: &str = "/sys/devices/platform/seven-segments-left/linedisp.1/";
    pub const SEVEN_SEGMENT_RIGHT: &str = "/sys/devices/platform/seven-segments-right/linedisp.0/";

    pub const BUZZER: Pin = super::P2_30;

    pub const LED: &str = "/sys/devices/platform/techlab-led/leds/multi-led/";

    pub const ACCELEROMETER: &str = "/sys/bus/iio/devices/iio:device1";
}
