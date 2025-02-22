pub struct Pin {
    chip: usize,
    offset: u32,
}

impl Pin {
    pub(crate) const fn new(chip: usize, offset: u32) -> Self {
        Self { chip, offset }
    }

    #[cfg(feature = "led")]
    pub(crate) fn gpio_output(&self) -> std::io::Result<gpiod::Lines<gpiod::Output>> {
        let chip = gpiod::Chip::new(self.chip)?;
        let opts = gpiod::Options::output([self.offset]);
        chip.request_lines(opts)
    }

    #[cfg(feature = "button")]
    pub(crate) fn gpio_input(&self) -> std::io::Result<gpiod::Lines<gpiod::Input>> {
        let chip = gpiod::Chip::new(self.chip)?;
        let opts = gpiod::Options::input([self.offset]).edge(gpiod::EdgeDetect::Both);
        chip.request_lines(opts)
    }
}
