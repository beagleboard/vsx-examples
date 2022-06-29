#!/usr/bin/env python3
# ////////////////////////////////////////
# //	seqLED.py
# //	Blinks the USR LEDs in sequence.
# //	Wiring:
# //	Setup:
# //	See:
# ////////////////////////////////////////
# //	Tested: may: 2022.06.29 - BBB - 5.10.109-ti-r45

import time
import os

LEDs=4
LEDPATH='/sys/class/leds/beaglebone:green:usr'

# Turn off triggers
for i in range(LEDs):
    print(LEDPATH+str(i)+"/trigger")
    f = open(LEDPATH+str(i)+"/trigger", "w")
    f.write("none")
    f.close()

# Open a file for each LED
f = []
for i in range(LEDs):
    f.append(open(LEDPATH+str(i)+"/brightness", "w"))

# Sequence
while True:
    for i in range(LEDs):
        f[i].seek(0)
        f[i].write("1")
        time.sleep(0.25)
    for i in range(LEDs):
        f[i].seek(0)
        f[i].write("0")
        time.sleep(0.25)
