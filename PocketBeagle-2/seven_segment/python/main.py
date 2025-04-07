from time import sleep
from sysfs import Device
from pathlib import Path

TECHLAB_SEVEN_SEGMENT_LEFT = Path("/sys/devices/platform/seven-segments-left/linedisp.1/")
TECHLAB_SEVEN_SEGMENT_RIGHT = Path("/sys/devices/platform/seven-segments-right/linedisp.0/")

segment_left = Device(path=TECHLAB_SEVEN_SEGMENT_LEFT)
segment_right = Device(path=TECHLAB_SEVEN_SEGMENT_RIGHT)

left_msg = segment_left.sysfs('message')
right_msg = segment_right.sysfs('message')

segment_left.sysfs('scroll_step_ms').write(1000)
segment_right.sysfs('scroll_step_ms').write(1000)

print("Countdown")
left_msg.write('10000000000')
right_msg.write('09876543210')

sleep(11)

left_msg.write(' ')
right_msg.write(' ')
