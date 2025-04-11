#[derive(Clone, Copy)]
pub struct Tone {
    freq: f64,
}

impl Tone {
    pub const C: Self = Self::from_frequency(16.35);
    pub const C_SHARP: Self = Self::from_frequency(17.32);
    pub const D: Self = Self::from_frequency(18.35);
    pub const D_SHARP: Self = Self::from_frequency(19.45);
    pub const E: Self = Self::from_frequency(20.60);
    pub const F: Self = Self::from_frequency(21.83);
    pub const F_SHARP: Self = Self::from_frequency(23.12);
    pub const G: Self = Self::from_frequency(24.50);
    pub const G_SHARP: Self = Self::from_frequency(25.96);
    pub const A: Self = Self::from_frequency(27.50);
    pub const A_SHARP: Self = Self::from_frequency(29.14);
    pub const B: Self = Self::from_frequency(30.87);

    pub const fn freq(&self) -> f64 {
        self.freq
    }

    pub const fn octave(&self, n: usize) -> Self {
        assert!(n <= 8);
        let pow = 1 << n;

        Self::from_frequency(self.freq * (pow as f64))
    }

    pub const fn from_frequency(freq: f64) -> Self {
        Self { freq }
    }
}
