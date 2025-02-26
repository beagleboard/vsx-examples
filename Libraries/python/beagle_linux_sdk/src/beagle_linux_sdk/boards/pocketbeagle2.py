from ..pin import AdcPin, Pin, GpioPin, PwmPin

P1_19 = Pin(adc=AdcPin(0, 0))
P1_20 = Pin(gpio=GpioPin("2", 50))
P1_36 = Pin(pwm=PwmPin(0, 0))

P2_33 = Pin(gpio=GpioPin("2", 52))

EEPROM_PATH = "/sys/bus/i2c/devices/0-0050/eeprom"
