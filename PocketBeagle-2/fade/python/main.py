import time
from beagle_linux_sdk import PwmLed
from beagle_linux_sdk.boards.pocketbeagle2 import P1_36

PERIOD = 255
DELAY = 0.05

led = PwmLed(P1_36)

while True:
    for i in range(5, PERIOD, 5):
        led.start(i, PERIOD)
        time.sleep(DELAY)

    for i in range(PERIOD, -1, -5):
        led.start(i, PERIOD)
        time.sleep(DELAY)
