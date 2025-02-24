from beagle_linux_sdk import Button
from beagle_linux_sdk.boards.pocketbeagle2 import P2_33

btn = Button(P2_33)

print("Waiting for Button Press")
btn.wait_for_press()
print("Button Pressed")
