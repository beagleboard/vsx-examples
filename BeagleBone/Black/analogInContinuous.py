#!/usr/bin/env python3
#//////////////////////////////////////
#	analogInContinuous.py
# 	Read analog data via IIO continous mode and plots it.
#//////////////////////////////////////

# From: https://stackoverflow.com/questions/20295646/python-ascii-plots-in-terminal
# https://github.com/dkogan/gnuplotlib
# https://github.com/dkogan/gnuplotlib/blob/master/guide/guide.org
# sudo apt install gnuplot  (10 minute to install)
# pip install gnuplotlib
# apt install python3-numpy
# This uses X11, so when connecting to the bone from the host use:  ssh -X bone
# I had to use sudo apt instal xauth on the bone for X to work.

# See https://elinux.org/index.php?title=EBC_Exercise_10a_Analog_In#Analog_in_-_Continuous.2C_Change_the_sample_rate
# for instructions on changing the sampling rate.  Can go up to 200KHz.

import numpy      as np
import gnuplotlib as gp
import time
# import struct

IIOPATH='/sys/bus/iio/devices/iio:device0'
IIODEV='/dev/iio:device0'
LEN = 100
SAMPLERATE=8000
AIN='2'

# Setup IIO for Continous reading
# Enable AIN
try:
    f = open(IIOPATH+'/scan_elements/in_voltage'+AIN+'_en', 'w')
    f.write('1') 
    f.close()
except:     # carry on if it's already enabled
    pass
# Set buffer length
f = open(IIOPATH+'/buffer/length', 'w')
f.write(str(2*LEN))     # I think LEN is in 16-bit values, but here we pass bytes
f.close()
# Enable continous
f = open(IIOPATH+'/buffer/enable', 'w')
f.write('1')
f.close()

fd = open(IIODEV, "r")

print('Hit ^C to stop')

x = np.linspace(0, 1000*LEN/SAMPLERATE, LEN)

try:
    while True:
        y = np.fromfile(fd, dtype='uint16', count=LEN)*1.8/4096
        # print(y)
        gp.plot(x, y,
            xlabel = 't (ms)',
            ylabel = 'volts',
            _yrange = [0, 2],
            title  = 'analogInContinuous',
            legend = np.array( ("P9.39", ), ),
            ascii=1,
            # terminal="xterm",
            # legend = np.array( ("P9.40", "P9.38"), ),
            # _with  = 'lines'
            )

except KeyboardInterrupt:
    print("\nTurning off input.")
    # Disable continous
    f = open(IIOPATH+'/buffer/enable', 'w')
    f.write('0')
    f.close()
    
    f = open(IIOPATH+'/scan_elements/in_voltage'+AIN+'_en', 'w')
    f.write('0') 
    f.close()

# // Bone  | Pocket | AIN
# // ----- | ------ | --- 
# // P9_39 | P1_19  | 0
# // P9_40 | P1_21  | 1
# // P9_37 | P1_23  | 2
# // P9_38 | P1_25  | 3
# // P9_33 | P1_27  | 4
# // P9_36 | P2_35  | 5
# // P9_35 | P1_02  | 6
