//! Abstractions to help working with Linux devices using sysfs entries.

use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Seek, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

const CHAR_DEV_PATH: &str = "/sys/dev/char/";

/// A structure to represent a Linux kernel device
pub struct Device(PathBuf);

impl Device {
    /// Create a new [Device] from the path to the base directory.
    pub fn with_path<T: Into<PathBuf>>(path: T) -> io::Result<Self> {
        let p = path.into();
        if !p.is_dir() {
            Err(io::Error::other("Path should be a directory"))
        } else {
            Ok(Self(p))
        }
    }

    /// Creates a new [Device] from the first Linux device found with the name.
    ///
    /// For more control, use [Self::with_path]
    pub fn with_dev_name(name: &str) -> io::Result<Self> {
        let mut temp = String::with_capacity(10);
        let p = Path::new(CHAR_DEV_PATH);

        assert!(p.is_dir());
        for dev in std::fs::read_dir(p).unwrap() {
            let dev_p = dev.unwrap().path();
            let name_p = dev_p.join("name");

            // Some devices do not have `name` sysfs entry
            if !name_p.exists() {
                continue;
            }

            let mut name_f = File::open(&name_p).unwrap();
            name_f.read_to_string(&mut temp).unwrap();

            if temp.trim() == name {
                return Ok(Self(dev_p.to_path_buf()));
            }

            temp.clear();
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to find any device named {name}",
        ))
    }

    /// Open a sysfs file in read-only mode
    pub fn sysfs_r(&self, file_name: &str) -> io::Result<Entry> {
        self.sysfs(file_name, OpenOptions::new().read(true))
    }

    /// Open a sysfs file in write-only mode
    pub fn sysfs_w(&self, file_name: &str) -> io::Result<Entry> {
        self.sysfs(file_name, OpenOptions::new().write(true))
    }

    fn sysfs(&self, file_name: &str, opts: &mut OpenOptions) -> io::Result<Entry> {
        let fpath = self.0.join(file_name);

        if fpath.is_file() {
            opts.open(fpath).map(Entry)
        } else {
            Err(io::Error::other("Failed to find sysfs entry"))
        }
    }
}

/// A structure to represet a sysfs entry for a device
pub struct Entry(File);

impl Entry {
    pub fn read_f64(&mut self) -> io::Result<f64> {
        self.read()
    }

    /// Read a single line
    pub fn read_string(&mut self) -> io::Result<String> {
        let mut data = String::with_capacity(10);
        self.0.seek(io::SeekFrom::Start(0))?;
        self.0.read_to_string(&mut data)?;

        Ok(data)
    }

    pub fn write_string(&mut self, data: &str) -> io::Result<()> {
        self.0.write_all(data.as_bytes())
    }

    pub fn write<T: ToString>(&mut self, data: T) -> io::Result<()> {
        self.0.write_all(data.to_string().as_bytes())
    }

    fn read<T>(&mut self) -> io::Result<T>
    where
        T: FromStr,
    {
        self.read_string()?
            .trim()
            .parse()
            .map_err(|_| io::Error::other(format!("Failed to parse sysfs value")))
    }
}
