use std::{
    fs::File,
    io::{Read, Seek},
    path::PathBuf,
};

pub(crate) struct Adc {
    raw_file: File,
    ref_voltage: f64,
    scale: f64,
}

impl Adc {
    const BASE_SYSFS_PATH: &str = "/sys/bus/iio/devices/iio:device";

    pub(crate) fn new(iio_dev: usize, channel: usize) -> std::io::Result<Self> {
        let iio_dir = PathBuf::from(format!("{}{iio_dev}", Self::BASE_SYSFS_PATH));

        if !iio_dir.is_dir() {
            return Err(std::io::Error::other("IIO device does not exist"));
        }

        let mut scale_file = File::open(iio_dir.join("in_voltage_scale"))?;
        let mut data = String::with_capacity(10);
        scale_file.read_to_string(&mut data)?;
        let scale = data.trim().parse().unwrap();

        // Maybe I can somehow get value from regulator
        let ref_voltage = 3300f64;

        let raw_file = File::open(iio_dir.join(format!("in_voltage{}_raw", channel)))?;

        Ok(Self {
            raw_file,
            ref_voltage,
            scale,
        })
    }

    fn read_raw(&mut self) -> std::io::Result<f64> {
        let mut data = String::with_capacity(10);
        self.raw_file.seek(std::io::SeekFrom::Start(0))?;
        self.raw_file.read_to_string(&mut data)?;
        Ok(data.trim().parse().unwrap())
    }

    pub(crate) fn read_scaled(&mut self) -> std::io::Result<f64> {
        let raw = self.read_raw()?;
        Ok(raw * self.scale)
    }

    pub(crate) const fn ref_voltage(&self) -> f64 {
        self.ref_voltage
    }
}
