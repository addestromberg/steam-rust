#[derive(Debug, Copy, Clone)]
pub struct Pressure {
    mpa: f64,
}

impl Pressure {
    pub fn from_mpa(p: f64) -> Self {
        Self { mpa: p }
    }

    pub fn from_bar(p: f64) -> Self {
        Self { mpa: p / 10.0 }
    }

    pub fn from_pa(p: f64) -> Self {
        Self { mpa: p / 1_000_000.0 }
    }

    pub fn as_mpa(&self) -> f64 { self.mpa }

    pub fn as_bar(&self) -> f64 { self.mpa * 10.0 }

    pub fn as_pa(&self) -> f64 { self.mpa * 1_000_000.0 }
}
