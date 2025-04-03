import time
from beagle_helper import Led
from beagle_helper.boards.pocketbeagle2 import USR_LED4

led = Led(USR_LED4)

while True:
    print("ON")
    led.on()
    time.sleep(1)

    print("OFF")
    led.off()
    time.sleep(1)
