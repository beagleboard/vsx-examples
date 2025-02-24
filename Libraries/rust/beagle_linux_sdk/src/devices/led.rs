pub struct Led(gpiod::Lines<gpiod::Output>);

impl Led {
    pub fn new(pin: crate::pin::Pin) -> std::io::Result<Self> {
        pin.gpio_output().map(Self)
    }

    pub fn on(&self) -> std::io::Result<()> {
        self.0.set_values([true])
    }

    pub fn off(&self) -> std::io::Result<()> {
        self.0.set_values([false])
    }
}
