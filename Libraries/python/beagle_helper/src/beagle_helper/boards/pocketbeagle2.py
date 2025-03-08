from pathlib import Path
from ..pin import AdcPin, Pin, GpioPin, PwmPin

P1_19 = Pin(adc=AdcPin(0, 0))
P1_20 = Pin(gpio=GpioPin("2", 50))
P1_36 = Pin(pwm=PwmPin(0, 0))

P2_30 = Pin(pwm=PwmPin(2, 0))
P2_33 = Pin(gpio=GpioPin("2", 52))

USR_LED1 = Path("/sys/class/leds/beaglebone:green:usr1")
USR_LED2 = Path("/sys/class/leds/beaglebone:green:usr2")
USR_LED3 = Path("/sys/class/leds/beaglebone:green:usr3")
USR_LED4 = Path("/sys/class/leds/beaglebone:green:usr4")

EEPROM_PATH = "/sys/bus/i2c/devices/0-0050/eeprom"

TECHLAB_SEVEN_SEGMENT_LEFT = Path("/sys/devices/platform/seven-segments-left/linedisp.1/")
TECHLAB_SEVEN_SEGMENT_RIGHT = Path("/sys/devices/platform/seven-segments-right/linedisp.0/")
TECHLAB_LED = Path("/sys/devices/platform/techlab-led/leds/multi-led/")
