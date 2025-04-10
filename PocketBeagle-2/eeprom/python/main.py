from pprint import pprint
from sysfs import Device, Entry
from pathlib import Path
from dataclasses import dataclass
from struct import Struct

EEPROM = Path("/sys/bus/i2c/devices/0-0050/")


@dataclass
class Header:
    hdr_id: int
    length: int

    @staticmethod
    def data_format() -> str:
        return "BH"


@dataclass
class BoardInfo:
    name: str
    version: str
    proc_number: str
    variant: str
    pcb_revision: str
    schematic_bom_revision: str
    software_revision: str
    vendor_id: str
    build_week: str
    build_year: str
    serial: str

    @staticmethod
    def data_format() -> str:
        return "16s2s4s2s2s2s2s2s2s2s10s"


@dataclass
class BeagleEeprom:
    magic_number: tuple[int, int, int, int]
    hdr1: Header
    hdr2: Header
    board_info: BoardInfo
    hdr3: Header
    ddr_info: int
    termination: int

    @classmethod
    def read(cls, e: Entry):
        hdr_format = Header.data_format()
        board_info_format = BoardInfo.data_format()

        # Use little-endian
        data_format = Struct(
            "<BBBB" + hdr_format + hdr_format + board_info_format + hdr_format + "HB"
        )
        data = e.read_binary(data_format.size)
        parsed = data_format.unpack(data)

        magic_num = tuple(parsed[:4])
        hdr1 = Header(parsed[4], parsed[5])
        hdr2 = Header(parsed[6], parsed[7])
        board_info = BoardInfo(
            parsed[8].decode("ascii"),
            parsed[9].decode("ascii"),
            parsed[10].decode("ascii"),
            parsed[11].decode("ascii"),
            parsed[12].decode("ascii"),
            parsed[13].decode("ascii"),
            parsed[14].decode("ascii"),
            parsed[15].decode("ascii"),
            parsed[16].decode("ascii"),
            parsed[17].decode("ascii"),
            parsed[18].decode("ascii"),
        )
        hdr3 = Header(parsed[19], parsed[20])

        return cls(magic_num, hdr1, hdr2, board_info, hdr3, parsed[21], parsed[22])


eeprom_dev = Device(path=EEPROM)
eeprom = eeprom_dev.sysfs("eeprom")

contents = BeagleEeprom.read(eeprom)
pprint(contents)
