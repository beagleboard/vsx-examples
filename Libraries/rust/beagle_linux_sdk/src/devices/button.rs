pub struct Button(gpiod::Lines<gpiod::Input>);

impl Button {
    pub fn new(pin: crate::pin::Pin) -> std::io::Result<Self> {
        pin.gpio_input().map(Self)
    }

    pub fn wait_for_press(&mut self) -> std::io::Result<()> {
        loop {
            let evt = self.0.read_event()?;
            if evt.edge == gpiod::Edge::Falling {
                return Ok(());
            }
        }
    }

    pub fn wait_for_release(&mut self) -> std::io::Result<()> {
        loop {
            let evt = self.0.read_event()?;
            if evt.edge == gpiod::Edge::Rising {
                return Ok(());
            }
        }
    }
}
