from ..pin import Pin, GpioPin, PwmPin

P1_20 = Pin(gpio=GpioPin("2", 50))
P1_36 = Pin(pwm=PwmPin(0, 0))

P2_33 = Pin(gpio=GpioPin("2", 52))

EEPROM_PATH = "/sys/bus/i2c/devices/0-0050/eeprom"
