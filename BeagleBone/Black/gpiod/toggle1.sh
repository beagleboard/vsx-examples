#!/bin/bash
# gpio chip=1  line 18=P9_14  line19=P9_16, run: gpioinfo | grep -i -e chip -e P9_14
# Stays in given state for given number of microseconds

while true; do 
    gpioset --mode=time --usec=100000 1 18=0 19=1
    gpioset --mode=time --usec=100000 1 18=1 19=0
done