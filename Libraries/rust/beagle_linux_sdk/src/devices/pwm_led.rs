use crate::abstractions::pwm::PwmChannel;

pub struct PwmLed(PwmChannel);

impl PwmLed {
    pub fn new(pin: crate::pin::Pin) -> std::io::Result<Self> {
        pin.pwm_output().map(Self)
    }

    pub fn start(&self, duty_cycle: usize, period: usize) -> std::io::Result<()> {
        self.0.set_period(period)?;
        self.0.set_duty_cycle(duty_cycle)?;
        self.0.enable()
    }
}
