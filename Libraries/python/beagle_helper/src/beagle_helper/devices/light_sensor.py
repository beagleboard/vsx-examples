from ..pin import Pin

class LightSensor:
    def __init__(self, pin: Pin, threashold: float | None = None) -> None:
        self.adc = pin.adc_input()
        if threashold:
            self.threashold = threashold
        else:
            self.threashold = 0.1


    def value(self) -> bool:
        ref_v = self.adc.ref_voltage()
        scaled = self.adc.read_scaled()

        return (scaled / ref_v) > self.threashold

    def wait_for_dark(self):
        while self.value():
            pass

    def wait_for_light(self):
        while not self.value():
            pass
