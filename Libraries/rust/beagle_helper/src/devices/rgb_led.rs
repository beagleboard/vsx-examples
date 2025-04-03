use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, Write},
    path::Path,
};

pub struct RgbLed {
    max_brightness: usize,
    rgb_idx: (usize, usize, usize),
    brightness: File,
    multi_intensity: File,
}

impl RgbLed {
    pub fn new(base_dir: &str) -> std::io::Result<Self> {
        let base_dir = Path::new(base_dir);

        if !base_dir.is_dir() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotADirectory,
                "Base path should be directory",
            ));
        }

        let max_brightness = Self::read_max_brightness(base_dir)?;
        let rgb_idx = Self::read_rgb_idx(base_dir)?;
        let brightness = OpenOptions::new()
            .write(true)
            .open(base_dir.join("brightness"))?;
        let multi_intensity = OpenOptions::new()
            .write(true)
            .open(base_dir.join("multi_intensity"))?;

        Ok(Self {
            max_brightness,
            rgb_idx,
            brightness,
            multi_intensity,
        })
    }

    fn read_max_brightness(base_dir: &Path) -> std::io::Result<usize> {
        let mut data = String::with_capacity(10);
        let mut file = File::open(base_dir.join("max_brightness"))?;
        file.read_to_string(&mut data)?;
        Ok(data.trim().parse().unwrap())
    }

    fn read_rgb_idx(base_dir: &Path) -> std::io::Result<(usize, usize, usize)> {
        let mut data = String::with_capacity(16);
        let mut file = File::open(base_dir.join("multi_index"))?;
        file.read_to_string(&mut data)?;

        let iter = data.trim().split_whitespace();
        let mut res = (0, 0, 0);

        for (idx, item) in iter.enumerate() {
            match item {
                "red" => res.0 = idx,
                "green" => res.1 = idx,
                "blue" => res.2 = idx,
                _ => return Err(std::io::Error::other("Invalud color")),
            }
        }

        Ok(res)
    }

    pub fn set_color(&mut self, rgb: (usize, usize, usize)) -> std::io::Result<()> {
        let mut res = [0, 0, 0];

        res[self.rgb_idx.0] = rgb.0;
        res[self.rgb_idx.1] = rgb.1;
        res[self.rgb_idx.2] = rgb.2;

        self.multi_intensity.seek(std::io::SeekFrom::Start(0))?;
        let data = format!("{} {} {}", res[0], res[1], res[2]);
        self.multi_intensity.write_all(data.as_bytes())?;

        Ok(())
    }

    pub fn set_brightness(&mut self, brightness: usize) -> std::io::Result<()> {
        if brightness > self.max_brightness {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid brightness",
            ));
        }

        self.brightness.seek(std::io::SeekFrom::Start(0))?;
        self.brightness
            .write_all(brightness.to_string().as_bytes())?;

        Ok(())
    }

    pub const fn max_brightness(&self) -> usize {
        self.max_brightness
    }
}
