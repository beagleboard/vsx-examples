from dataclasses import dataclass
from io import BufferedReader
from pathlib import Path
from typing import Self
import struct


@dataclass
class Header:
    hdr_id: int
    length: int

    @staticmethod
    def read_id(f: BufferedReader) -> int:
        data_format = struct.Struct("B")
        data = f.read(data_format.size)
        return data_format.unpack(data)[0]

    @staticmethod
    def read_len(f: BufferedReader) -> int:
        data_format = struct.Struct("H")
        data = f.read(data_format.size)
        return data_format.unpack(data)[0]


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
    def __read_str(f: BufferedReader, n: int) -> str:
        data_format = struct.Struct(f"{n}s")
        data = f.read(data_format.size)
        return data_format.unpack(data)[0].decode("utf-8")

    @staticmethod
    def read_name(f: BufferedReader) -> str:
        return BoardInfo.__read_str(f, 16)

    @staticmethod
    def read_version(f: BufferedReader) -> str:
        return BoardInfo.__read_str(f, 2)

    @staticmethod
    def read_proc_number(f: BufferedReader) -> str:
        return BoardInfo.__read_str(f, 4)

    @staticmethod
    def read_variant(f: BufferedReader) -> str:
        return BoardInfo.__read_str(f, 2)

    @staticmethod
    def read_pcb_revision(f: BufferedReader) -> str:
        return BoardInfo.__read_str(f, 2)

    @staticmethod
    def read_schematic_bom_revision(f: BufferedReader) -> str:
        return BoardInfo.__read_str(f, 2)

    @staticmethod
    def read_software_revision(f: BufferedReader) -> str:
        return BoardInfo.__read_str(f, 2)

    @staticmethod
    def read_vendor_id(f: BufferedReader) -> str:
        return BoardInfo.__read_str(f, 2)

    @staticmethod
    def read_build_week(f: BufferedReader) -> str:
        return BoardInfo.__read_str(f, 2)

    @staticmethod
    def read_build_year(f: BufferedReader) -> str:
        return BoardInfo.__read_str(f, 2)

    @staticmethod
    def read_serial(f: BufferedReader) -> str:
        return BoardInfo.__read_str(f, 10)


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
    def read(cls, p: Path) -> Self:
        with open(p, "rb") as file:
            magic_number = BeagleEeprom.__read_magic_number(file)
            hdr1 = BeagleEeprom.__read_header(file)
            hdr2 = BeagleEeprom.__read_header(file)
            board_info = BeagleEeprom.__read_board_info(file)
            hdr3 = BeagleEeprom.__read_header(file)
            ddr_info = BeagleEeprom.__read_ddr_info(file)
            termination = BeagleEeprom.__read_termination(file)

            return cls(
                magic_number=magic_number,
                hdr1=hdr1,
                hdr2=hdr2,
                board_info=board_info,
                hdr3=hdr3,
                ddr_info=ddr_info,
                termination=termination,
            )

    @staticmethod
    def __read_magic_number(f: BufferedReader) -> tuple[int, int, int, int]:
        magic_number_format = struct.Struct("BBBB")
        data = f.read(magic_number_format.size)
        return magic_number_format.unpack(data)

    @staticmethod
    def __read_header(f: BufferedReader) -> Header:
        hdr_id = Header.read_id(f)
        length = Header.read_len(f)
        return Header(hdr_id, length)

    @staticmethod
    def __read_board_info(f: BufferedReader):
        name = BoardInfo.read_name(f)
        version = BoardInfo.read_version(f)
        proc_number = BoardInfo.read_proc_number(f)
        variant = BoardInfo.read_variant(f)
        pcb_revision = BoardInfo.read_pcb_revision(f)
        schematic_bom_revision = BoardInfo.read_schematic_bom_revision(f)
        software_revision = BoardInfo.read_software_revision(f)
        vendor_id = BoardInfo.read_vendor_id(f)
        build_week = BoardInfo.read_build_week(f)
        build_year = BoardInfo.read_build_year(f)
        serial = BoardInfo.read_serial(f)

        return BoardInfo(
            name=name,
            version=version,
            proc_number=proc_number,
            variant=variant,
            pcb_revision=pcb_revision,
            schematic_bom_revision=schematic_bom_revision,
            software_revision=software_revision,
            vendor_id=vendor_id,
            build_week=build_week,
            build_year=build_year,
            serial=serial,
        )

    @staticmethod
    def __read_ddr_info(f: BufferedReader) -> int:
        data_format = struct.Struct("H")
        data = f.read(data_format.size)
        return data_format.unpack(data)[0]

    @staticmethod
    def __read_termination(f: BufferedReader) -> int:
        data_format = struct.Struct("B")
        data = f.read(data_format.size)
        return data_format.unpack(data)[0]
