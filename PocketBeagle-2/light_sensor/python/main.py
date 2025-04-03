from time import sleep
from beagle_helper import LightSensor
from beagle_helper.boards.pocketbeagle2 import P1_19

ldr = LightSensor(P1_19)

while True:
    if ldr.value():
        print("Light")
    else:
        print("Dark")
    sleep(0.5)
