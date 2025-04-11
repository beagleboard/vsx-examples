from pathlib import Path
from struct import Struct
from dataclasses import dataclass

CHAR_DEV_PATH = Path("/dev/char/")
INPUT_SYSFS = Path("/sys/class/input/")


@dataclass
class Timeval:
    tv_sec: int
    tv_usec: int


@dataclass
class InputKey:
    timeval: Timeval
    inp_type: int
    code: int
    value: int

    @staticmethod
    def data_format() -> Struct:
        return Struct("ddHHi")

    @classmethod
    def from_binary(cls, data: bytes):
        parsed = cls.data_format().unpack(data)

        return cls(Timeval(parsed[0], parsed[1]), parsed[2], parsed[3], parsed[4])

    def to_binary(self) -> bytes:
        return self.data_format().pack(
            self.timeval.tv_sec,
            self.timeval.tv_usec,
            self.inp_type,
            self.code,
            self.value,
        )

    @classmethod
    def with_freq(cls, freq: int):
        return cls(Timeval(0, 0), 0x12, 0x02, freq)


class CharDev:
    def __init__(self, dev: str) -> None:
        self.path = CHAR_DEV_PATH.joinpath(dev)

        if not self.path.exists():
            raise ValueError("Device does not exist")

    @classmethod
    def input_device_by_name(cls, name: str):
        assert INPUT_SYSFS.is_dir()

        for dev in INPUT_SYSFS.iterdir():
            name_p = dev.joinpath("device/name")

            if not name_p.is_file():
                continue

            with open(name_p, "r") as f:
                temp = f.read().strip()

                if temp != name:
                    continue

            dev_p = dev.joinpath("dev")
            with open(dev_p, "r") as f:
                temp = f.read().strip()
                return cls(temp)

        raise ValueError("Device not found")

    def read_evt(self) -> InputKey:
        with open(self.path, "rb") as f:
            data =  f.read(InputKey.data_format().size)
            return InputKey.from_binary(data)

    def write_evt(self, data: InputKey):
        with open(self.path, "wb") as f:
            f.write(data.to_binary())
