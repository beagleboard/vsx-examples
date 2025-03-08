from ..pin import Pin

class PwmLed:
    def __init__(self, pin: Pin) -> None:
        self.pwm = pin.pwm_output()

    def start(self, duty_cycle: int, period: int = 255):
        self.pwm.set_period(period)
        self.pwm.set_duty_cycle(duty_cycle)
        self.pwm.enable()
