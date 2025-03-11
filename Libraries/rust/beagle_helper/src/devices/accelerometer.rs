use std::{
    fs::File,
    io::{self, Read, Seek},
    path::Path,
};

pub struct Accel {
    x_raw: File,
    y_raw: File,
    z_raw: File,
}

impl Accel {
    pub fn new(p: &str) -> io::Result<Self> {
        let base_dir = Path::new(p);

        if !base_dir.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotADirectory,
                "Base path should be directory",
            ));
        }

        let x_raw = File::open(base_dir.join("in_accel_x_raw"))?;
        let y_raw = File::open(base_dir.join("in_accel_y_raw"))?;
        let z_raw = File::open(base_dir.join("in_accel_z_raw"))?;

        Ok(Self {
            x_raw,
            y_raw,
            z_raw,
        })
    }

    pub fn x_raw(&self) -> io::Result<i64> {
        Self::read_sysfs(&self.x_raw)
    }

    pub fn y_raw(&self) -> io::Result<i64> {
        Self::read_sysfs(&self.y_raw)
    }

    pub fn z_raw(&self) -> io::Result<i64> {
        Self::read_sysfs(&self.z_raw)
    }

    fn read_sysfs(mut f: &File) -> io::Result<i64> {
        let mut data = String::with_capacity(10);
        f.seek(io::SeekFrom::Start(0))?;
        f.read_to_string(&mut data)?;
        Ok(data.trim().parse().unwrap())
    }
}
