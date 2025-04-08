#![allow(dead_code)]

pub(crate) struct GpioPin {
    chip: usize,
    offset: u32,
}

impl GpioPin {
    pub(crate) const fn new(chip: usize, offset: u32) -> Self {
        Self { chip, offset }
    }

    #[cfg(feature = "led")]
    pub(crate) fn gpio_output(&self) -> std::io::Result<gpiod::Lines<gpiod::Output>> {
        let chip = gpiod::Chip::new(self.chip)?;
        let opts = gpiod::Options::output([self.offset]);
        chip.request_lines(opts)
    }

    #[cfg(feature = "button")]
    pub(crate) fn gpio_input(&self) -> std::io::Result<gpiod::Lines<gpiod::Input>> {
        let chip = gpiod::Chip::new(self.chip)?;
        let opts = gpiod::Options::input([self.offset]).edge(gpiod::EdgeDetect::Both);
        chip.request_lines(opts)
    }
}

pub(crate) struct PwmPin {
    chip: usize,
    channel: usize,
}

impl PwmPin {
    pub(crate) const fn new(chip: usize, channel: usize) -> Self {
        Self { chip, channel }
    }

    #[cfg(feature = "pwm")]
    pub(crate) fn pwm_output(&self) -> std::io::Result<crate::abstractions::pwm::PwmChannel> {
        let chip = crate::abstractions::pwm::PwmChip::new(self.chip)?;
        let chan = chip.export(self.channel)?;

        // Linux group permissions need a bit to work on the newly exported pwm. Removing this delay
        // can lead to Permission denied.
        std::thread::sleep(std::time::Duration::from_millis(100));

        Ok(chan)
    }
}

pub(crate) struct AdcPin {
    iio_device: usize,
    channel: usize,
}

impl AdcPin {
    pub(crate) const fn new(iio_device: usize, channel: usize) -> Self {
        Self {
            iio_device,
            channel,
        }
    }
}

pub struct Pin {
    gpio: Option<GpioPin>,
    pwm: Option<PwmPin>,
}

impl Pin {
    pub(crate) const fn new(gpio: Option<GpioPin>, pwm: Option<PwmPin>) -> Self {
        Self { gpio, pwm }
    }

    #[cfg(feature = "led")]
    pub(crate) fn gpio_output(&self) -> std::io::Result<gpiod::Lines<gpiod::Output>> {
        match &self.gpio {
            Some(x) => x.gpio_output(),
            None => Err(std::io::Error::other(
                "Pin does not support GPIO Output functionality",
            )),
        }
    }

    #[cfg(feature = "button")]
    pub(crate) fn gpio_input(&self) -> std::io::Result<gpiod::Lines<gpiod::Input>> {
        match &self.gpio {
            Some(x) => x.gpio_input(),
            None => Err(std::io::Error::other(
                "Pin does not support GPIO Input functionality",
            )),
        }
    }

    #[cfg(feature = "pwm")]
    pub(crate) fn pwm_output(&self) -> std::io::Result<crate::abstractions::pwm::PwmChannel> {
        match &self.pwm {
            Some(x) => x.pwm_output(),
            None => Err(std::io::Error::other(
                "Pin does not support PWM output functionality",
            )),
        }
    }
}
