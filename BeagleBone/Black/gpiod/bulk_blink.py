#!/usr/bin/env python3
# //////////////////////////////////////
# 	bulk_blink.py
#  Toggles given pins as fast as it can. 
# 	Given pins must be on the same gpiochip
# 	P9_14 is line 18 P9_16 is line 19.
#   Run gpioinfo to see which pins are where.
# 	Wiring:	Attach an oscilloscope to P9_14 and P9_16  to see the squarewave or 
#          uncomment the sleep and attach an LED.
# 	Setup:	sudo apt update; pip install gpiod
#           Run: gpioinfo | grep -i -e chip -e P9_14 to find chip and line numbers
# 	See:	https://git.kernel.org/pub/scm/libs/libgpiod/libgpiod.git/tree/bindings/python/examples
# //////////////////////////////////////
import sys
import time
import gpiod

try:
    if len(sys.argv) > 2:
        LED_CHIP = sys.argv[1]
        LED_LINE_OFFSETS = []
        for i in range(len(sys.argv) - 2):
            LED_LINE_OFFSETS.append(int(sys.argv[i + 2]))
    else:
        raise Exception()
# pylint: disable=broad-except
except Exception:
    print(
        "Usage:"
        + "    python3 -m gpiod.test.bulk_blink <chip> <line offset1>"
        + " [<line offset2> ...]"
    )
    sys.exit()

chip = gpiod.Chip(LED_CHIP)
lines = chip.get_lines(LED_LINE_OFFSETS)

lines.request(consumer='Bulk Blink', type=gpiod.LINE_REQ_DIR_OUT)

off = [0] * len(LED_LINE_OFFSETS)
on  = [1] * len(LED_LINE_OFFSETS)

while True:
    lines.set_values(off)
    time.sleep(0.25)
    lines.set_values(on)
    time.sleep(0.25)
