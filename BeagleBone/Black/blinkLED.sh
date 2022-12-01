#!/usr/bin/env bash
# //////////////////////////////////////
#	blink.sh
#	Blinks one LED wired to P9_14.
#	Wiring:	P9_14 connects to the plus lead of an LED.  The negative lead of the
#			LED goes to a 220 Ohm resistor.  The other lead of the resistor goes
#			to ground.
#	Setup:	
#	See:	
#//////////////////////////////////////

GPIOPATH='/sys/class/gpio/'
# Look up P9.14 using show-pins.  gpio1.18 maps to 50
pin=50

# Make sure pin is exported
if [ ! -d "${GPIOPATH}gpio${pin}" ]; then
    echo "${GPIOPATH}gpio$pin"  Not found
    echo $pin > ${GPIOPATH}/export
fi
# Make it an output pin
echo "out" > ${GPIOPATH}gpio${pin}/direction

# Blink
while true ; do
    echo "1" > ${GPIOPATH}gpio${pin}/value
    sleep 0.5
    echo "0" > ${GPIOPATH}gpio${pin}/value
    sleep 0.5
done