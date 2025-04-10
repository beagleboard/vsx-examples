"""
This example demonstrates reading GPIO Buttons using the GPIO Keys kernel driver
"""

from chardev import CharDev
from struct import Struct

BUTTONS_NAME = "buttons"
RIGHT_CODE = 106


def read_input_key(dev: CharDev) -> tuple[int, int]:
    data_format = Struct("ddHHi")
    data = dev.read_binary(data_format.size)
    parsed = data_format.unpack(data)

    return parsed[3], parsed[4]


btn = CharDev.input_device_by_name(BUTTONS_NAME)

print("Waiting for Input")
while True:
    code, val = read_input_key(btn)

    # We use value to only print on pressed (not released)
    if code == RIGHT_CODE and val == 1:
        print("Right")
