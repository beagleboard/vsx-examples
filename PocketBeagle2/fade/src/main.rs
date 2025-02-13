use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone)]
struct Chip(PathBuf);

impl Chip {
    const BASE_SYSFS_PATH: &str = "/sys/class/pwm/pwmchip";

    fn new(num: usize) -> Self {
        let p = PathBuf::from(format!("{}{}", Self::BASE_SYSFS_PATH, num));
        assert!(p.is_dir());

        Self(p)
    }

    /// Create a PWM channel for use.
    fn export(&self, channel: usize, force: bool) -> Channel {
        if force {
            unsafe {
                self.unexport(channel);
            }
        }

        let export = self.0.join("export");
        let mut f = OpenOptions::new().write(true).open(export).unwrap();
        f.write_all(channel.to_string().as_bytes()).unwrap();

        Channel::new(self.clone(), channel)
    }

    /// Unexports a PWM channel.
    ///
    /// SAFETY: Ensure no dangling references for the channel exist.
    unsafe fn unexport(&self, channel: usize) {
        let unexport = self.0.join("unexport");
        let mut f = OpenOptions::new().write(true).open(unexport).unwrap();
        let _ = f.write_all(channel.to_string().as_bytes());
    }
}

/// Structure to manage PWM Channel.
///
/// PWM is cleaned up after drop.
struct Channel {
    chip: Chip,
    channel: usize,
}

impl Channel {
    fn new(chip: Chip, channel: usize) -> Self {
        let channel = Self { chip, channel };

        assert!(channel.sysfs("").is_dir());

        channel
    }

    /// Enable PWM signal on channel
    fn enable(&self) {
        self.write("enable", b"1");
    }

    /// Disable PWM signal on channel
    fn disable(&self) {
        self.write("enable", b"0");
    }

    /// Set the active time of the PWM signal. Value is in nanoseconds and must be less than or
    /// equal to the period.
    fn set_duty_cycle(&self, duty: usize) {
        self.write("duty_cycle", duty.to_string().as_bytes());
    }

    /// Set the total period of the PWM signal. Value is in nanoseconds and is the sum of the
    /// active and inactive time of the PWM.
    fn set_period(&self, period: usize) {
        self.write("period", period.to_string().as_bytes());
    }

    fn write(&self, file: &str, data: &[u8]) {
        let f_path = self.sysfs(file);
        let mut f = OpenOptions::new().write(true).open(f_path).unwrap();

        f.write_all(data).unwrap();
    }

    fn sysfs(&self, file: &str) -> PathBuf {
        self.chip.0.join(format!("pwm{}", self.channel)).join(file)
    }
}

impl Drop for Channel {
    fn drop(&mut self) {
        self.disable();
        unsafe {
            self.chip.unexport(self.channel);
        }
    }
}

fn main() {
    const PERIOD: usize = 255;
    const DELAY: Duration = Duration::from_millis(50);

    let chip = Chip::new(0);
    let channel = chip.export(0, true);

    // Linux group permissions need a bit to work on the newly exported pwm. Removing this delay
    // can lead to Permission denied.
    sleep(DELAY);

    channel.set_period(PERIOD);

    loop {
        for i in (5..(PERIOD + 1)).step_by(5) {
            channel.set_duty_cycle(i);
            channel.enable();
            sleep(DELAY);
        }

        for i in (0..(PERIOD - 4)).step_by(5).rev() {
            channel.set_duty_cycle(i);
            channel.enable();
            sleep(DELAY);
        }
    }
}
