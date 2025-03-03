from pathlib import Path


class RgbLed:
    def __init__(self, base_dir: Path) -> None:
        if not base_dir.is_dir():
            raise ValueError("RGB LED does not exist")

        self.max_brightness = self.__read_max_brightness(base_dir)
        self.rgb_idx = self.__read_rgb_idx(base_dir)
        self.brightness = base_dir.joinpath("brightness")
        self.multi_intensity = base_dir.joinpath("multi_intensity")

    def set_brightness(self, b: int):
        with open(self.brightness, "w") as f:
            f.write(str(b))

    def set_color(self, color: tuple[int, int, int]):
        with open(self.multi_intensity, "w") as f:
            data = " ".join(map(str, color))
            f.write(data)

    @staticmethod
    def __read_max_brightness(base_dir: Path) -> int:
        with open(base_dir.joinpath("max_brightness"), "r") as f:
            return int(f.read())

    @staticmethod
    def __read_rgb_idx(base_dir) -> tuple[int, int, int]:
        with open(base_dir.joinpath("multi_index"), "r") as f:
            data = f.read()
            idx = data.strip().split(" ")
            res = [0, 0, 0]

            assert len(idx) == 3
            for i, v in enumerate(idx):
                if v == "red":
                    res[0] = i
                elif v == "green":
                    res[1] = i
                elif v == "blue":
                    res[2] = i
                else:
                    assert False

            return (res[0], res[1], res[2])
