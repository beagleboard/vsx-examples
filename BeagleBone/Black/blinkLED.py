#!/usr/bin/python
#//////////////////////////////////////
#	blink.py
#	Blinks one LED wired to P9_14.
#	Wiring:	P9_14 connects to the plus lead of an LED.  The negative lead of the
#			LED goes to a 220 Ohm resistor.  The other lead of the resistor goes
#			to ground.
#	Setup:	
#	See:	
#//////////////////////////////////////
import time
import os

# Look up P9.14 using show-pins.  gpio1.18 maps to 50
pin = "50"
 
GPIOPATH='/sys/class/gpio/'
# Make sure pin is exported
if (not os.path.exists(GPIOPATH+"gpio"+pin)):
    f = open(GPIOPATH+"export", "w")
    f.write(pin)
    f.close()

# Make it an output pin
f = open(GPIOPATH+"gpio"+pin+"/direction", "w")
f.write("out")
f.close()
 
f = open(GPIOPATH+"gpio"+pin+"/value", "w")
# Blink
while True:
    f.seek(0)
    f.write("1")
    time.sleep(0.5)

    f.seek(0)
    f.write("0")
    time.sleep(0.5)
f.close()