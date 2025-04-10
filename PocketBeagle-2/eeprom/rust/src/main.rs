use std::{io::Read, str::from_utf8};

use beagle_helper::sysfs::{Device, Entry};

#[repr(C, packed)]
#[derive(Debug)]
pub struct BeagleEeprom {
    pub magic_number: [u8; 4],
    pub hdr1: Header,
    pub hdr2: Header,
    pub board_info: BoardInfo,
    pub hdr3: Header,
    pub ddr_info: u16,
    pub termination: u8,
}

impl BeagleEeprom {
    fn read(sys: &mut Entry) -> std::io::Result<Self> {
        let mut data = [0u8; std::mem::size_of::<Self>()];
        sys.read_exact(&mut data)?;
        Ok(unsafe { std::mem::transmute(data) })
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub id: u8,
    pub len: u16,
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct BoardInfo {
    pub name: [u8; 16],
    pub version: [u8; 2],
    pub proc_number: [u8; 4],
    pub variant: [u8; 2],
    pub pcb_revision: [u8; 2],
    pub schematic_bom_revision: [u8; 2],
    pub software_revision: [u8; 2],
    pub vendor_id: [u8; 2],
    pub build_week: [u8; 2],
    pub build_year: [u8; 2],
    pub serial: [u8; 10],
}

impl std::fmt::Debug for BoardInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut w = f.debug_struct("BoardInfo");

        w.field("name", &from_utf8(&self.name).unwrap());
        w.field("version", &from_utf8(&self.version).unwrap());
        w.field("proc_number", &from_utf8(&self.proc_number).unwrap());
        w.field("variant", &from_utf8(&self.variant).unwrap());
        w.field("pcb_revision", &from_utf8(&self.pcb_revision).unwrap());
        w.field(
            "schematic_bom_revision",
            &from_utf8(&self.schematic_bom_revision).unwrap(),
        );
        w.field(
            "software_revision",
            &from_utf8(&self.software_revision).unwrap(),
        );
        w.field("vendor_id", &from_utf8(&self.vendor_id).unwrap());
        w.field("build_week", &from_utf8(&self.build_week).unwrap());
        w.field("build_year", &from_utf8(&self.build_year).unwrap());
        w.field("serial", &from_utf8(&self.serial).unwrap());

        w.finish()
    }
}

pub const EEPROM: &str = "/sys/bus/i2c/devices/0-0050";

fn main() {
    let eeprom_dev = Device::with_path(EEPROM).unwrap();
    let mut eeprom = eeprom_dev.sysfs_r("eeprom").unwrap();

    let contents = BeagleEeprom::read(&mut eeprom).unwrap();

    println!("{:#?}", contents);
}
