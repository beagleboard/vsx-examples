import time
from beagle_linux_sdk import Led
from beagle_linux_sdk.boards.pocketbeagle2 import P1_20

led = Led(P1_20)

while True:
    print("ON")
    led.on()
    time.sleep(1)
    print("OFF")
    led.off()
    time.sleep(1)
