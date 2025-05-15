import time
from sysfs import Device
from pathlib import Path

LED = Path("/sys/class/leds/beaglebone:green:usr4")
number_of_blinks = 5
led = Device(path=LED)

# Technically, max_brightness will be an unsigned integer value. However, since we never
# actually parse it, and writing to the file needs conversion back to string anyway, it is
# better to just keep it as string
max_brightness = led.sysfs("max_brightness").read_str()

brightness = led.sysfs("brightness")

for blinks in range(number_of_blinks):
    print("ON")
    brightness.write_str(max_brightness)
    time.sleep(1) # 1 second on

    print("OFF")
    brightness.write_str("0")
    time.sleep(1) # 1 second off
