from dataclasses import dataclass
from ..pin import Pin
from typing import Self


NS_IN_SEC = 1000000000


@dataclass()
class Tone:
    period: int

    @classmethod
    def from_freq(cls, freq: float) -> Self:
        period = int(NS_IN_SEC / freq)
        return cls(period)

    def period_ns(self) -> int:
        return self.period

    def duty_cycle_ns(self) -> int:
        return int(self.period_ns() / 2)

    def octave(self, n: int):
        return Tone(self.period >> n)


class TonalBuzzer:
    def __init__(self, pin: Pin) -> None:
        self.pwm = pin.pwm_output()

    def play(self, tone: Tone | None):
        if tone:
            self.pwm.set_period(tone.period_ns())
            self.pwm.set_duty_cycle(tone.duty_cycle_ns())
            self.pwm.enable()
        else:
            self.stop()

    def stop(self):
        self.pwm.disable()


NOTE_C = Tone.from_freq(16.35)
NOTE_C_SHARP = Tone.from_freq(17.32)
NOTE_D = Tone.from_freq(18.35)
NOTE_D_SHARP = Tone.from_freq(19.45)
NOTE_E = Tone.from_freq(20.60)
NOTE_F = Tone.from_freq(21.83)
NOTE_F_SHARP = Tone.from_freq(23.12)
NOTE_G = Tone.from_freq(24.50)
NOTE_G_SHARP = Tone.from_freq(25.96)
NOTE_A = Tone.from_freq(27.50)
NOTE_A_SHARP = Tone.from_freq(29.14)
NOTE_B = Tone.from_freq(30.87)
