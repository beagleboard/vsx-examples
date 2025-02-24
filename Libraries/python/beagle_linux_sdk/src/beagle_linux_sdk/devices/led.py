from ..pin import Pin


class Led:
    def __init__(self, pin: Pin) -> None:
        self.pin = pin.gpio_output()

    def on(self):
        self.pin.set_value(1)

    def off(self):
        self.pin.set_value(0)
