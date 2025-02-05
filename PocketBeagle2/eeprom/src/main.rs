use std::fmt::Debug;
use std::io::Read;
use std::str::from_utf8;

#[cfg(test)]
mod test;

const EEPROM: &str = "/sys/bus/i2c/devices/0-0050/eeprom";

#[repr(packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Contents {
    magic_number: [u8; 4],
    hdr1: Header,
    hdr2: Header,
    board_info: BoardInfo,
    hdr3: Header,
    ddr_info: u16,
    termination: u8,
}

impl Contents {
    fn from_bytes(buf: [u8; std::mem::size_of::<Self>()]) -> Self {
        unsafe { std::mem::transmute(buf) }
    }
}

#[repr(packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Header {
    id: u8,
    len: u16,
}

#[repr(packed)]
#[derive(Clone, Copy, PartialEq, Eq)]
struct BoardInfo {
    name: [u8; 16],
    version: [u8; 2],
    proc_number: [u8; 4],
    variant: [u8; 2],
    pcb_revision: [u8; 2],
    schematic_bom_revision: [u8; 2],
    software_revision: [u8; 2],
    vendor_id: [u8; 2],
    build_week: [u8; 2],
    build_year: [u8; 2],
    serial: [u8; 10],
}

impl BoardInfo {
    fn name(&self) -> &str {
        from_utf8(&self.name).unwrap()
    }

    fn version(&self) -> &str {
        from_utf8(&self.version).unwrap()
    }

    fn proc_number(&self) -> &str {
        from_utf8(&self.proc_number).unwrap()
    }

    fn variant(&self) -> &str {
        from_utf8(&self.variant).unwrap()
    }

    fn pcb_revision(&self) -> &str {
        from_utf8(&self.variant).unwrap()
    }

    fn schematic_bom_revision(&self) -> &str {
        from_utf8(&self.schematic_bom_revision).unwrap()
    }

    fn software_revision(&self) -> &str {
        from_utf8(&self.software_revision).unwrap()
    }

    fn vendor_id(&self) -> &str {
        from_utf8(&self.vendor_id).unwrap()
    }

    fn build_week(&self) -> &str {
        from_utf8(&self.build_week).unwrap()
    }

    fn build_year(&self) -> &str {
        from_utf8(&self.build_year).unwrap()
    }

    fn serial(&self) -> &str {
        from_utf8(&self.serial).unwrap()
    }
}

impl Debug for BoardInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut w = f.debug_struct("BoardInfo");

        w.field("name", &self.name());
        w.field("version", &self.version());
        w.field("proc_number", &self.proc_number());
        w.field("variant", &self.variant());
        w.field("pcb_revision", &self.pcb_revision());
        w.field("schematic_bom_revision", &self.schematic_bom_revision());
        w.field("software_revision", &self.software_revision());
        w.field("vendor_id", &self.vendor_id());
        w.field("build_week", &self.build_week());
        w.field("build_year", &self.build_year());
        w.field("serial", &self.serial());

        w.finish()
    }
}

fn main() {
    let mut buf = [0u8; std::mem::size_of::<Contents>()];
    let mut sysfs = std::fs::File::open(EEPROM).unwrap();

    sysfs.read_exact(&mut buf).unwrap();

    let contents = Contents::from_bytes(buf);

    println!("EEPROM Data: {:#?}", contents);
}
