import gpiod

from .abstractions.adc import Adc
from .abstractions.pwm import PwmChip, PwmChannel
from time import sleep
from dataclasses import dataclass


class GpioPin:
    def __init__(self, chipset: str, offset: int) -> None:
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


class PwmPin:
    def __init__(self, chipset: int, channel: int) -> None:
        self.chipset = chipset
        self.channel = channel

    def pwm_output(self) -> PwmChannel:
        chip = PwmChip(self.chipset)
        return chip.export(self.channel)


@dataclass
class AdcPin:
    iio_device: int
    channel: int

    def adc_input(self) -> Adc:
        return Adc(self.iio_device, self.channel)


class Pin:
    def __init__(self, gpio: GpioPin | None = None, pwm: PwmPin | None = None, adc: AdcPin | None = None) -> None:
        self.gpio = gpio
        self.pwm = pwm
        self.adc = adc

    def gpio_output(self):
        if self.gpio == None:
            raise ValueError("Pin cannot be used as GPIO")

        return self.gpio.gpio_output()

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

    def adc_input(self) -> Adc:
        if self.adc == None:
            raise ValueError("Pin cannot be used as ADC")

        return self.adc.adc_input()
