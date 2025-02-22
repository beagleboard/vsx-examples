import gpiod


class Pin:
    def __init__(self, chipset: str | None = None, offset: int | None = None) -> None:
        self.chipset = chipset
        self.offset = offset

    def gpio_output(self):
        if self.chipset == None or self.offset == None:
            raise ValueError("Pin cannot be used as GPIO")
        chip = gpiod.Chip(self.chipset)
        led = chip.get_line(self.offset)
        led.request(consumer="", type=gpiod.LINE_REQ_DIR_OUT)

        return led

    def gpio_input(self):
        if self.chipset == None or self.offset == None:
            raise ValueError("Pin cannot be used as GPIO")
        chip = gpiod.Chip(self.chipset)
        led = chip.get_line(self.offset)
        led.request(consumer="", type=gpiod.LINE_REQ_EV_BOTH_EDGES)

        return led
