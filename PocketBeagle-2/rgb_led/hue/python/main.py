"""
This example generates different color hues on the RGB LED
"""

from sysfs import Device
from pathlib import Path
from time import sleep


def wheel(pos: int) -> tuple[int, int, int]:
    if pos < 85:
        return (255 - pos * 3, 0, pos * 3)
    elif pos < 170:
        pos -= 85
        return (0, pos * 3, 255 - pos * 3)
    else:
        pos -= 170
        return (pos * 3, 255 - pos * 3, 0)


LED = Path("/sys/devices/platform/techlab-led/leds/multi-led/")
DELAY = 0.01

led = Device(path=LED)

max_brightness = led.sysfs("max_brightness").read_str()
# Set brightness to max from the start
led.sysfs("brightness").write(max_brightness)

multi_intensity = led.sysfs("multi_intensity")

while True:
    for i in range(0, 256):
        val = ' '.join(map(str, wheel(i)))
        multi_intensity.write(val)
        sleep(DELAY)
