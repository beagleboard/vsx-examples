import time
from beagle_helper import RgbLed
from beagle_helper.boards.pocketbeagle2 import TECHLAB_LED


def wheel(pos: int) -> tuple[int, int, int]:
    if pos < 85:
        return (255 - pos * 3, 0, pos * 3)
    elif pos < 170:
        pos -= 85
        return (0, pos * 3, 255 - pos * 3)
    else:
        pos -= 170
        return (pos * 3, 255 - pos * 3, 0)


UPDATE_INTERVAL = 0.01
BRIGHTNESS = 255

led = RgbLed(TECHLAB_LED)
led.set_brightness(BRIGHTNESS)

while True:
    for i in range(0, 256):
        led.set_color(wheel(i))
        time.sleep(UPDATE_INTERVAL)
