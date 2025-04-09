"""
This example cycles through all the base colors in the Color circle.
"""

from sysfs import Device
from pathlib import Path
from time import sleep

LED = Path("/sys/devices/platform/techlab-led/leds/multi-led/")
DELAY = 1

led = Device(path=LED)

max_brightness = led.sysfs("max_brightness").read_str()
# Set brightness to max from the start
led.sysfs("brightness").write(max_brightness)

multi_intensity = led.sysfs("multi_intensity")

while True:
    multi_intensity.write('255 0 0')
    sleep(DELAY)

    multi_intensity.write('255 255 0')
    sleep(DELAY)

    multi_intensity.write('0 255 0')
    sleep(DELAY)

    multi_intensity.write('0 255 255')
    sleep(DELAY)

    multi_intensity.write('0 0 255')
    sleep(DELAY)

    multi_intensity.write('255 0 255')
    sleep(DELAY)

    multi_intensity.write('255 255 255')
    sleep(DELAY)

    multi_intensity.write('0 0 0')
    sleep(DELAY)
