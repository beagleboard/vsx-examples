from time import sleep
from sysfs import Device

# Reading values from ADC directly
DEV_NAME = "ad7291"

THRESHOLD = 2000

adc = Device(name=DEV_NAME)

scale = adc.sysfs("in_voltage_scale").read_float()
ldr_raw = adc.sysfs("in_voltage0_raw")

while True:
    scaled = ldr_raw.read_float() * scale

    if scaled > THRESHOLD:
        print("Light")
    else:
        print("Dark")

    sleep(0.5)
