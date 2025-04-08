from sysfs import Device
from pathlib import Path
from time import sleep

LED = Path("/sys/devices/platform/techlab-led/leds/multi-led/")
DELAY = 0.05
MAX_INTENSITY = 255

led = Device(path=LED)

max_brightness = led.sysfs("max_brightness").read_str()
# Set brightness to max from the start
led.sysfs("brightness").write(max_brightness)

multi_intensity = led.sysfs("multi_intensity")

while True:
    for i in range(5, MAX_INTENSITY, 5):
        multi_intensity.write(f"{i} 0 0")
        sleep(DELAY)

    for i in range(MAX_INTENSITY, -1, -5):
        multi_intensity.write(f"{i} 0 0")
        sleep(DELAY)
