"""
Ported from Arduino Example:

https://github.com/robsoncouto/arduino-songs/blob/master/harrypotter/harrypotter.ino
"""

import time
from beagle_linux_sdk import TonalBuzzer
from beagle_linux_sdk.devices.tonal_buzzer import (
    NOTE_C,
    NOTE_C_SHARP,
    NOTE_D,
    NOTE_D_SHARP,
    NOTE_E,
    NOTE_F,
    NOTE_G,
    NOTE_G_SHARP,
    NOTE_A,
    NOTE_A_SHARP,
    NOTE_B,
)
from beagle_linux_sdk.boards.pocketbeagle2 import P2_30


# Hedwig's theme fromn the Harry Potter Movies
# Score from https://musescore.com/user/3811306/scores/4906610
MELODY = [
    # Opening
    (None, 2),
    (NOTE_D.octave(4), 4),
    (NOTE_G.octave(4), -4),
    (NOTE_A_SHARP.octave(4), 8),
    (NOTE_A.octave(4), 4),
    (NOTE_G.octave(4), 2),
    (NOTE_D.octave(5), 4),
    (NOTE_C.octave(5), -2),
    (NOTE_A.octave(4), -2),
    (NOTE_G.octave(4), -4),
    (NOTE_A_SHARP.octave(4), 8),
    (NOTE_A.octave(4), 4),
    (NOTE_F.octave(4), 2),
    (NOTE_G_SHARP.octave(4), 4),
    (NOTE_D.octave(4), -1),
    (NOTE_D.octave(4), 4),
    # Part 2
    (NOTE_G.octave(4), -4),
    (NOTE_A_SHARP.octave(4), 8),
    (NOTE_A.octave(4), 4),
    (NOTE_G.octave(4), 2),
    (NOTE_D.octave(5), 4),
    (NOTE_F.octave(5), 2),
    (NOTE_E.octave(5), 4),
    (NOTE_D_SHARP.octave(5), 2),
    (NOTE_B.octave(4), 4),
    (NOTE_D_SHARP.octave(5), -4),
    (NOTE_D.octave(5), 8),
    (NOTE_C_SHARP.octave(5), 4),
    (NOTE_C_SHARP.octave(4), 2),
    (NOTE_B.octave(4), 4),
    (NOTE_G.octave(4), -1),
    (NOTE_A_SHARP.octave(4), 4),
    # Part 3
    (NOTE_D.octave(5), 2),
    (NOTE_A_SHARP.octave(4), 4),
    (NOTE_D.octave(5), 2),
    (NOTE_A_SHARP.octave(4), 4),
    (NOTE_D_SHARP.octave(5), 2),
    (NOTE_D.octave(5), 4),
    (NOTE_C_SHARP.octave(5), 2),
    (NOTE_A.octave(4), 4),
    (NOTE_A_SHARP.octave(4), -4),
    (NOTE_D.octave(5), 8),
    (NOTE_C_SHARP.octave(5), 4),
    (NOTE_C_SHARP.octave(4), 2),
    (NOTE_D.octave(4), 4),
    (NOTE_D.octave(5), -1),
    (None, 4),
    (NOTE_A_SHARP.octave(4), 4),
    # Part 4
    (NOTE_D.octave(5), 2),
    (NOTE_A_SHARP.octave(4), 4),
    (NOTE_D.octave(5), 2),
    (NOTE_A_SHARP.octave(4), 4),
    (NOTE_F.octave(5), 2),
    (NOTE_E.octave(5), 4),
    (NOTE_D_SHARP.octave(5), 2),
    (NOTE_B.octave(4), 4),
    (NOTE_D_SHARP.octave(5), -4),
    (NOTE_D.octave(5), 8),
    (NOTE_C_SHARP.octave(5), 4),
    (NOTE_C_SHARP.octave(4), 2),
    (NOTE_A_SHARP.octave(4), 4),
    (NOTE_G.octave(4), -1),
]

TEMPO = 144
WHOLE_NOTE = (60000 * 4) / TEMPO

buzzer = TonalBuzzer(P2_30)
print("Start Playing")

for t, d in MELODY:
    note_dur = WHOLE_NOTE / abs(d)

    if d < 0:
        note_dur *= 1.5

    buzzer.play(t)
    time.sleep(note_dur * 0.9 / 1000)
    buzzer.stop()
    time.sleep(note_dur * 0.1 / 1000)

buzzer.stop()
