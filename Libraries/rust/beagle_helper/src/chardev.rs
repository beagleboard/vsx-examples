use std::{
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

const CHAR_DEV_PATH: &str = "/dev/char/";
const INPUT_SYSFS: &str = "/sys/class/input/";

pub struct CharDev(File);

impl CharDev {
    fn input_from_dev(dev: &str, opts: &OpenOptions) -> io::Result<Self> {
        let devp = Path::new(CHAR_DEV_PATH).join(dev);

        // Not sure why but the device is not a File. So cannot use [Path::is_file()]
        if devp.exists() {
            opts.open(devp).map(Self)
        } else {
            Err(io::Error::other("Device does not exist"))
        }
    }

    pub fn open_input_with_name(name: &str, opts: &OpenOptions) -> io::Result<Self> {
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

                return Self::input_from_dev(temp.trim(), opts);
            }
        }

        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Failed to find any input named {name}",
        ))
    }

    pub fn write_evt(&mut self, evt: InputEvent) -> io::Result<()> {
        let bin: [u8; std::mem::size_of::<InputEvent>()] = unsafe { std::mem::transmute(evt) };
        self.0.write_all(&bin)
    }
}

impl io::Read for CharDev {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Timeval {
    pub tv_sec: i64,
    pub tv_usec: i64,
}

/// Defined in [Event Interface](https://docs.kernel.org/input/input.html#event-interface)
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct InputEvent {
    pub time: Timeval,
    pub r#type: std::ffi::c_ushort,
    pub code: std::ffi::c_ushort,
    pub value: std::ffi::c_int,
}

impl InputEvent {
    pub const fn with_frequency(freq: std::ffi::c_int) -> Self {
        Self {
            r#type: INPUT_TYPE_EV_SND,
            code: INPUT_CODE_SND_TONE,
            value: freq,
            time: Timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
        }
    }
}

const INPUT_TYPE_EV_SND: std::ffi::c_ushort = 0x12;
const INPUT_CODE_SND_TONE: std::ffi::c_ushort = 0x02;
