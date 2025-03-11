from pathlib import Path


class Accel:
    def __init__(self, base_dir: Path) -> None:
        if not base_dir.is_dir():
            raise ValueError("Accelerometer path does not exist")

        self.x_raw_path = base_dir.joinpath("in_accel_x_raw")
        self.y_raw_path = base_dir.joinpath("in_accel_y_raw")
        self.z_raw_path = base_dir.joinpath("in_accel_z_raw")

    @staticmethod
    def _read_int(p: Path) -> int:
        with open(p, 'r') as f:
            data = f.read()
            return int(data)

    def x_raw(self) -> int:
        return Accel._read_int(self.x_raw_path)

    def y_raw(self) -> int:
        return Accel._read_int(self.y_raw_path)

    def z_raw(self) -> int:
        return Accel._read_int(self.z_raw_path)
