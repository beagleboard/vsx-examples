from pathlib import Path


BASE_SYSFS_PATH = "/sys/bus/iio/devices/iio:device"


class Adc:
    def __init__(self, iio_dev: int, channel: int) -> None:
        iio_dir = Path(f"{BASE_SYSFS_PATH}{iio_dev}")
        if not iio_dir.is_dir():
            raise ValueError("PWM channel does not exist")

        with open(iio_dir.joinpath("in_voltage_scale"), "r") as scale_file:
            data = scale_file.read()
            self.scale = float(data)

        self.ref_v = 3300
        self.raw_volatage_file = iio_dir.joinpath(f"in_voltage{channel}_raw")

    def read_raw(self) -> float:
        with open(self.raw_volatage_file, "r") as f:
            data = f.read()
            return float(data)

    def read_scaled(self) -> float:
        return self.read_raw() * self.scale

    def ref_voltage(self) -> float:
        return self.ref_v
