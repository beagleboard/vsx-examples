use crate::{abstractions::adc::Adc, pin::Pin};

pub struct LightSensor {
    adc: Adc,
    threshold: f64,
}

impl LightSensor {
    pub fn new(pin: Pin, threshold: Option<f64>) -> std::io::Result<Self> {
        let adc = pin.adc_input()?;
        Ok(Self {
            adc,
            threshold: threshold.unwrap_or(0.1),
        })
    }

    pub fn value(&mut self) -> std::io::Result<bool> {
        let ref_voltage = self.adc.ref_voltage();
        let scaled = self.adc.read_scaled()?;

        Ok((scaled / ref_voltage) > self.threshold)
    }

    pub fn wait_for_dark(&mut self) {
        loop {
            if let Ok(false) = self.value() {
                return;
            }
        }
    }

    pub fn wait_for_light(&mut self) {
        loop {
            if let Ok(true) = self.value() {
                return;
            }
        }
    }
}
