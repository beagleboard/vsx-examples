from sysfs import Device
from pathlib import Path
from time import sleep

LED = Path("/sys/devices/platform/techlab-led/leds/rgb:/")
DELAY = 0.05

led = Device(path=LED)

brightness = led.sysfs('brightness')

max_brightness = led.sysfs('max_brightness').read_int()

# Set intensity to a single color
led.sysfs('multi_intensity').write('255 0 0')

while True:
    for i in range(5, max_brightness, 5):
        brightness.write(i)
        sleep(DELAY)

    for i in range(max_brightness, -1, -5):
        brightness.write(i)
        sleep(DELAY)
