use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

const CHAR_DEV_PATH: &str = "/dev/char/";
const INPUT_SYSFS: &str = "/sys/class/input/";

pub struct CharDev(File);

impl CharDev {
    fn input_from_dev(dev: &str) -> io::Result<Self> {
        let devp = Path::new(CHAR_DEV_PATH).join(dev);

        // Not sure why but the device is not a File. So cannot use [Path::is_file()]
        if devp.exists() {
            File::open(devp).map(Self)
        } else {
            Err(io::Error::other("Device does not exist"))
        }
    }

    pub fn open_input_with_name(name: &str) -> io::Result<Self> {
        let p = Path::new(INPUT_SYSFS);

        assert!(p.is_dir());
        for inp in std::fs::read_dir(p).unwrap() {
            let mut temp = String::with_capacity(10);
            let dev_p = inp.unwrap().path();

            let name_p = dev_p.join("device").join("name");

            // Some devices do not have `name` sysfs entry
            if !name_p.exists() {
                continue;
            }

            let mut name_f = File::open(&name_p).unwrap();
            name_f.read_to_string(&mut temp).unwrap();

            if temp.trim() == name {
                let mut temp = String::with_capacity(10);
                let mut devf = File::open(dev_p.join("dev")).unwrap();

                devf.read_to_string(&mut temp).unwrap();

                return Self::input_from_dev(temp.trim());
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to find any input named {name}",
        ))
    }
}

impl io::Read for CharDev {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}
