import gpiod

from .abstractions.pwm import PwmChip, PwmChannel
from time import sleep


class GpioPin:
    def __init__(self, chipset: str, offset: int) -> None:
        self.chipset = chipset
        self.offset = offset

    def gpio_input(self):
        chip = gpiod.Chip(self.chipset)
        led = chip.get_line(self.offset)
        led.request(consumer="", type=gpiod.LINE_REQ_EV_BOTH_EDGES)

        return led


class PwmPin:
    def __init__(self, chipset: int, channel: int) -> None:
        self.chipset = chipset
        self.channel = channel

    def pwm_output(self) -> PwmChannel:
        chip = PwmChip(self.chipset)
        return chip.export(self.channel)


class Pin:
    def __init__(self, gpio: GpioPin | None = None, pwm: PwmPin | None = None) -> None:
        self.gpio = gpio
        self.pwm = pwm

    def gpio_input(self):
        if self.gpio == None:
            raise ValueError("Pin cannot be used as GPIO")

        return self.gpio.gpio_input()

    def pwm_output(self) -> PwmChannel:
        if self.pwm == None:
            raise ValueError("Pin cannot be used as PWM")

        chan = self.pwm.pwm_output()
        
        # Linux group permissions need a bit to work on the newly exported pwm.
        # Removing this delay can lead to Permission denied.
        sleep(0.1)

        return chan
