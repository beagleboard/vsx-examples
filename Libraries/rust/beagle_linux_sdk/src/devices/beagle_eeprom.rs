//! All boards from BeagleBoard.org have an EEPROM which contains information regarding the exact
//! revision of the board. This helps with debugging hardware issues and differences between
//! specific revisions of the same board.

use std::fmt::Debug;
use std::io::Read;
use std::str::from_utf8;

#[repr(packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    fn from_bytes(buf: [u8; std::mem::size_of::<Self>()]) -> Self {
        unsafe { std::mem::transmute(buf) }
    }

    pub fn read(path: &str) -> std::io::Result<Self> {
        let mut buf = [0u8; std::mem::size_of::<BeagleEeprom>()];
        let mut sysfs = std::fs::File::open(path)?;

        sysfs.read_exact(&mut buf)?;

        Ok(Self::from_bytes(buf))
    }
}

#[repr(packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub id: u8,
    pub len: u16,
}

#[repr(packed)]
#[derive(Clone, Copy, PartialEq, Eq)]
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

impl BoardInfo {
    pub fn name(&self) -> &str {
        from_utf8(&self.name).unwrap()
    }

    pub fn version(&self) -> &str {
        from_utf8(&self.version).unwrap()
    }

    pub fn proc_number(&self) -> &str {
        from_utf8(&self.proc_number).unwrap()
    }

    pub fn variant(&self) -> &str {
        from_utf8(&self.variant).unwrap()
    }

    pub fn pcb_revision(&self) -> &str {
        from_utf8(&self.variant).unwrap()
    }

    pub fn schematic_bom_revision(&self) -> &str {
        from_utf8(&self.schematic_bom_revision).unwrap()
    }

    pub fn software_revision(&self) -> &str {
        from_utf8(&self.software_revision).unwrap()
    }

    pub fn vendor_id(&self) -> &str {
        from_utf8(&self.vendor_id).unwrap()
    }

    pub fn build_week(&self) -> &str {
        from_utf8(&self.build_week).unwrap()
    }

    pub fn build_year(&self) -> &str {
        from_utf8(&self.build_year).unwrap()
    }

    pub fn serial(&self) -> &str {
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
