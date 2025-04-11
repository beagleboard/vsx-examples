"""
Ported from Arduino Example:

https://github.com/robsoncouto/arduino-songs/blob/master/harrypotter/harrypotter.ino
"""

import time
import melody
from chardev import CharDev, InputKey


OFF_NOTE = InputKey.with_freq(0)

buzzer = CharDev.input_device_by_name("pwm-beeper")

print("Start Playing")
for t, d in melody.MELODY:
    note = InputKey.with_freq(int(t.freq)) if t else OFF_NOTE

    note_dur = melody.WHOLE_NOTE / abs(d)
    if d < 0:
        note_dur *= 1.5

    buzzer.write_evt(note)
    time.sleep(note_dur * 0.9 / 1000)

    buzzer.write_evt(OFF_NOTE)
    time.sleep(note_dur * 0.1 / 1000)
