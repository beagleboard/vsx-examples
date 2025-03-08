from time import sleep
from beagle_helper import SevenSegment
from beagle_helper.boards.pocketbeagle2 import TECHLAB_SEVEN_SEGMENT_LEFT, TECHLAB_SEVEN_SEGMENT_RIGHT

segment_left = SevenSegment(TECHLAB_SEVEN_SEGMENT_LEFT)
segment_right = SevenSegment(TECHLAB_SEVEN_SEGMENT_RIGHT)

print("Countdown Automatic on Right")
segment_right.set_step(1000)
segment_right.set_message("9876543210")

print("Countdown Manual on Left")
for i in range(9, -1, -1):
    segment_left.set_message(str(i))
    sleep(1)

segment_left.set_message(" ")
segment_right.set_message(" ")
