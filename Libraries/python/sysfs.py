from pathlib import Path


CHAR_DEV_PATH = Path("/sys/dev/char/")


class Entry:
    def __init__(self, path: Path) -> None:
        assert path.is_file()
        self.path = path

    def read_float(self) -> float:
        with open(self.path, "r") as f:
            return float(f.read())


class Device:
    def __init__(self, name: str | None = None, path: Path | None = None) -> None:
        if path:
            if path.is_dir():
                self.path = path
                return
            else:
                raise ValueError("Invalid path")

        if name:
            assert CHAR_DEV_PATH.is_dir()
            for dev in CHAR_DEV_PATH.iterdir():
                name_p = dev.joinpath("name")
                if not name_p.is_file():
                    continue

                with open(name_p, "r") as f:
                    val = f.read().strip()
                    if val == name:
                        self.path = dev
                        return

            raise ValueError(f"Device with {name} not found")

        raise ValueError("Either name or path should be provided")

    def sysfs(self, fname: str) -> Entry:
        return Entry(self.path.joinpath(fname))
