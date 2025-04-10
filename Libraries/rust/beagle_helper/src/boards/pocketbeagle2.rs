use crate::pin::{GpioPin, Pin, PwmPin};

pub const P1_20: Pin = Pin::new(Some(GpioPin::new(2, 50)), None);
pub const P1_36: Pin = Pin::new(None, Some(PwmPin::new(0, 0)));

pub const P2_30: Pin = Pin::new(None, Some(PwmPin::new(2, 0)));
pub const P2_33: Pin = Pin::new(Some(GpioPin::new(2, 52)), None);

pub mod techlab {
    use crate::pin::Pin;

    pub const BUZZER: Pin = super::P2_30;

    pub const LED: &str = "/sys/devices/platform/techlab-led/leds/multi-led/";
}
