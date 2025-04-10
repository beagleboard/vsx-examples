from pathlib import Path

CHAR_DEV_PATH = Path('/dev/char/')
INPUT_SYSFS = Path("/sys/class/input/")

class CharDev:
    def __init__(self, dev: str) -> None:
        self.path = CHAR_DEV_PATH.joinpath(dev)

        if not self.path.exists():
            raise ValueError("Device does not exist")


    @classmethod
    def input_device_by_name(cls, name: str):
        assert INPUT_SYSFS.is_dir()

        for dev in INPUT_SYSFS.iterdir():
            name_p = dev.joinpath('device/name')

            if not name_p.is_file():
                continue

            with open(name_p, 'r') as f:
                temp = f.read().strip()

                if temp != name:
                    continue

            dev_p = dev.joinpath('dev')
            with open(dev_p, 'r') as f:
                temp = f.read().strip()
                return cls(temp)

        raise ValueError("Device not found")

    def read_binary(self, size: int):
        with open(self.path, 'rb') as f:
            return f.read(size)
