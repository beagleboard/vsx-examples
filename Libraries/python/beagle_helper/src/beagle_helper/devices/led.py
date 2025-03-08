from pathlib import Path
from ..pin import Pin


class Led:
    def __init__(self, pin: Pin | Path) -> None:
        if isinstance(pin, Pin):
            self.inner = _GpioLed(pin)
        elif isinstance(pin, Path):
            self.inner = _SysfsLed(pin)
        else:
            raise ValueError("Invalid pin")

    def on(self):
        self.inner.on()

    def off(self):
        self.inner.off()


class _SysfsLed:
    def __init__(self, base_dir: Path) -> None:
        if not base_dir.is_dir():
            raise ValueError("LED does not exist")

        self.max_brightness = self.__read_max_brightness(base_dir)
        self.brightness = base_dir.joinpath("brightness")

    def on(self):
        with open(self.brightness, "w") as f:
            f.write(self.max_brightness)

    def off(self):
        with open(self.brightness, "w") as f:
            f.write("0")

    @staticmethod
    def __read_max_brightness(base_dir: Path) -> str:
        with open(base_dir.joinpath("max_brightness"), "r") as f:
            return f.read()


class _GpioLed:
    def __init__(self, pin: Pin) -> None:
        self.pin = pin.gpio_output()

    def on(self):
        self.pin.set_value(1)

    def off(self):
        self.pin.set_value(0)
