use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

pub struct Led(LedInner);

impl Led {
    pub fn new(x: impl TryInto<Self, Error = std::io::Error>) -> std::io::Result<Self> {
        x.try_into()
    }

    pub fn on(&mut self) -> std::io::Result<()> {
        self.0.on()
    }

    pub fn off(&mut self) -> std::io::Result<()> {
        self.0.off()
    }
}

enum LedInner {
    Gpio(GpiodLed),
    Sysfs(SysfsLed),
}

impl LedInner {
    pub fn on(&mut self) -> std::io::Result<()> {
        match self {
            Self::Gpio(gpiod_led) => gpiod_led.on(),
            Self::Sysfs(sysfs_led) => sysfs_led.on(),
        }
    }

    pub fn off(&mut self) -> std::io::Result<()> {
        match self {
            Self::Gpio(gpiod_led) => gpiod_led.off(),
            Self::Sysfs(sysfs_led) => sysfs_led.off(),
        }
    }
}

impl TryFrom<crate::pin::Pin> for Led {
    type Error = std::io::Error;

    fn try_from(value: crate::pin::Pin) -> Result<Self, Self::Error> {
        GpiodLed::new(value).map(LedInner::Gpio).map(Self)
    }
}

impl TryFrom<&str> for Led {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SysfsLed::new(value).map(LedInner::Sysfs).map(Self)
    }
}

struct GpiodLed(gpiod::Lines<gpiod::Output>);

impl GpiodLed {
    pub fn new(pin: crate::pin::Pin) -> std::io::Result<Self> {
        pin.gpio_output().map(Self)
    }

    pub fn on(&self) -> std::io::Result<()> {
        self.0.set_values([true])
    }

    pub fn off(&self) -> std::io::Result<()> {
        self.0.set_values([false])
    }
}

struct SysfsLed {
    max_brightness: String,
    brightness: File,
}

impl SysfsLed {
    pub fn new(dir: &str) -> std::io::Result<Self> {
        let base_dir = Path::new(dir);

        if !base_dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotADirectory,
                "Base path should be directory",
            ));
        }

        let max_brightness = Self::read_max_brightness(base_dir)?;
        let brightness = OpenOptions::new()
            .write(true)
            .open(base_dir.join("brightness"))?;

        Ok(Self {
            max_brightness,
            brightness,
        })
    }

    pub fn on(&mut self) -> std::io::Result<()> {
        self.brightness.write_all(self.max_brightness.as_bytes())
    }

    pub fn off(&mut self) -> std::io::Result<()> {
        self.brightness.write_all(b"0")
    }

    fn read_max_brightness(base_dir: &Path) -> std::io::Result<String> {
        let mut data = String::with_capacity(10);
        let mut file = File::open(base_dir.join("max_brightness"))?;
        file.read_to_string(&mut data)?;
        Ok(data)
    }
}
