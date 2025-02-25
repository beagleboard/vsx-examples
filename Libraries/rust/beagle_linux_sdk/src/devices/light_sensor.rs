use std::fs::File;

use crate::pin::Pin;

pub struct LightSensor(File);

impl LightSensor {
    pub fn new(pin: Pin) -> Self {
        todo!()
    }
}
