"""
This example demonstrates reading GPIO Buttons using the GPIO Keys kernel driver
"""

from chardev import CharDev

BUTTONS_NAME = "buttons"
RIGHT_CODE = 106


btn = CharDev.input_device_by_name(BUTTONS_NAME)

print("Waiting for Input")
while True:
    evt = btn.read_evt()

    # We use value to only print on pressed (not released)
    if evt.code == RIGHT_CODE and evt.value == 1:
        print("Right")
