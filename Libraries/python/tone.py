from dataclasses import dataclass


@dataclass()
class Tone:
    freq: float

    def octave(self, n: int):
        return Tone(self.freq * pow(2, n))


NOTE_C = Tone(16.35)
NOTE_C_SHARP = Tone(17.32)
NOTE_D = Tone(18.35)
NOTE_D_SHARP = Tone(19.45)
NOTE_E = Tone(20.60)
NOTE_F = Tone(21.83)
NOTE_F_SHARP = Tone(23.12)
NOTE_G = Tone(24.50)
NOTE_G_SHARP = Tone(25.96)
NOTE_A = Tone(27.50)
NOTE_A_SHARP = Tone(29.14)
NOTE_B = Tone(30.87)
