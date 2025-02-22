import gpiod
from .pin import Pin


class Button:
    def __init__(self, pin: Pin) -> None:
        self.pin = pin.gpio_input()

    def wait_for_press(self):
        while True:
            if self.pin.event_wait():
                evt = self.pin.event_read()
                if evt.type == gpiod.LineEvent.FALLING_EDGE:
                    return

    def wait_for_release(self):
        while True:
            if self.pin.event_wait():
                evt = self.pin.event_read()
                if evt.type == gpiod.LineEvent.RISING_EDGE:
                    return
