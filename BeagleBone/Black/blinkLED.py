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
# import Adafruit_BBIO.GPIO as GPIO
import time
import os

# out = "P9_14"     Look up using show-pins
gpio = "50"
 
# GPIO.setup(out, GPIO.OUT)
PATH='/sys/class/gpio/'
# export gpio pin if not already
if (not os.path.exists(PATH+"gpio"+gpio)):
    f = open(PATH+"export", "w")
    f.write(gpio)
    f.close()
# Set direction to "out"
f = open(PATH+"gpio"+gpio+"/direction", "w")
f.write("out")
f.close()
 
f = open(PATH+"gpio"+gpio+"/value", "w")
while True:
    # GPIO.output(out, GPIO.HIGH)
    f.seek(0)
    f.write("1")
    time.sleep(0.5)
    # GPIO.output(out, GPIO.LOW)
    f.seek(0)
    f.write("0")
    time.sleep(0.5)
f.close()