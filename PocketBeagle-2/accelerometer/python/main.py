from beagle_helper import Accel
from beagle_helper.boards.pocketbeagle2 import TECHLAB_ACCELEROMETER
from time import sleep

accl = Accel(TECHLAB_ACCELEROMETER)

while True:
    print('X = ', accl.x_raw(), ', Y = ', accl.y_raw(), ', Z = ', accl.z_raw())
    sleep(1)
