use crate::abstractions::pwm::PwmChannel;

const NS_IN_SEC: f64 = 1_000_000_000.0;

#[derive(Clone, Copy)]
pub struct Tone {
    period: usize,
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

    const fn duty_cycle_ns(&self) -> usize {
        self.period_ns() >> 1
    }

    const fn period_ns(&self) -> usize {
        self.period
    }

    pub const fn octave(&self, n: usize) -> Self {
        assert!(n <= 8);
        Self {
            period: self.period >> n,
        }
    }

    pub const fn from_frequency(freq: f64) -> Self {
        let period = NS_IN_SEC / freq;

        Self {
            period: period as usize,
        }
    }
}

pub struct TonalBuzzer(PwmChannel);

impl TonalBuzzer {
    pub fn new(pin: crate::pin::Pin) -> std::io::Result<Self> {
        pin.pwm_output().map(Self)
    }

    pub fn play(&self, tone: Option<Tone>) -> std::io::Result<()> {
        if let Some(t) = tone {
            self.0.set_period(t.period_ns())?;
            self.0.set_duty_cycle(t.duty_cycle_ns())?;
            self.0.enable()
        } else {
            self.stop()
        }
    }

    pub fn stop(&self) -> std::io::Result<()> {
        self.0.disable()
    }
}
