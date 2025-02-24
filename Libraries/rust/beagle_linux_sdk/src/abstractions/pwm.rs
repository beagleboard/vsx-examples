use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

pub(crate) struct PwmChip {
    chip_dir: PathBuf,
    export: PathBuf,
    unexport: PathBuf,
}

impl PwmChip {
    const BASE_SYSFS_PATH: &str = "/sys/class/pwm/pwmchip";

    pub(crate) fn new(n: usize) -> std::io::Result<Self> {
        let chip_dir = PathBuf::from(format!("{}{}", Self::BASE_SYSFS_PATH, n));
        let export = chip_dir.join("export");
        let unexport = chip_dir.join("unexport");

        if !chip_dir.is_dir() {
            Err(std::io::Error::other("PWM chip does not exist"))
        } else {
            Ok(Self {
                chip_dir,
                export,
                unexport,
            })
        }
    }

    /// Create a PWM channel for use.
    pub(crate) fn export(&self, channel: usize) -> std::io::Result<PwmChannel> {
        // Unexport channel if already present
        self.unexport(channel);

        let mut f = OpenOptions::new().write(true).open(&self.export).unwrap();
        f.write_all(channel.to_string().as_bytes()).unwrap();

        PwmChannel::new(self.chip_dir.clone(), channel)
    }

    /// Unexports a PWM channel. Ignore if the channle has not been exported.
    ///
    /// SAFETY: Ensure no dangling references for the channel exist.
    fn unexport(&self, channel: usize) {
        let mut f = OpenOptions::new().write(true).open(&self.unexport).unwrap();
        let _ = f.write_all(channel.to_string().as_bytes());
    }
}

pub(crate) struct PwmChannel {
    channel: usize,
    chip_dir: PathBuf,
    enable: PathBuf,
    duty_cycle: PathBuf,
    period: PathBuf,
}

impl PwmChannel {
    pub(crate) fn new(chip_dir: PathBuf, channel: usize) -> std::io::Result<Self> {
        let chan_dir = chip_dir.join(format!("pwm{}", channel));

        if !chip_dir.is_dir() {
            return Err(std::io::Error::other("PWM channel does not exist"));
        }

        let enable = chan_dir.join("enable");
        let duty_cycle = chan_dir.join("duty_cycle");
        let period = chan_dir.join("period");

        Ok(Self {
            chip_dir,
            channel,
            enable,
            duty_cycle,
            period,
        })
    }

    /// Enable PWM signal on channel
    pub(crate) fn enable(&self) -> std::io::Result<()> {
        let mut f = OpenOptions::new().write(true).open(&self.enable)?;
        f.write(b"1")?;
        Ok(())
    }

    /// Set the active time of the PWM signal. Value is in nanoseconds and must be less than or
    /// equal to the period.
    pub(crate) fn set_duty_cycle(&self, duty: usize) -> std::io::Result<()> {
        let mut f = OpenOptions::new().write(true).open(&self.duty_cycle)?;
        f.write(duty.to_string().as_bytes())?;
        Ok(())
    }

    /// Set the total period of the PWM signal. Value is in nanoseconds and is the sum of the
    /// active and inactive time of the PWM.
    pub(crate) fn set_period(&self, period: usize) -> std::io::Result<()> {
        let mut f = OpenOptions::new().write(true).open(&self.period)?;
        f.write(period.to_string().as_bytes())?;
        Ok(())
    }
}

impl Drop for PwmChannel {
    fn drop(&mut self) {
        let unexport = self.chip_dir.join("unexport");
        let mut f = OpenOptions::new().write(true).open(unexport).unwrap();
        let _ = f.write_all(self.channel.to_string().as_bytes());
    }
}
