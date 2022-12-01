#!/usr/bin/env bash
#//////////////////////////////////////
#	blinkInternalLED.sh
#	Blinks a USR LED
#	Wiring:	
#	Setup:	
#	See:	
#//////////////////////////////////////

LED="3"
 
LEDPATH='/sys/class/leds/beaglebone:green:usr'
 
while true ; do
    echo "1" > ${LEDPATH}${LED}/brightness
    sleep 0.5
    echo "0" > ${LEDPATH}${LED}/brightness
    sleep 0.5
done