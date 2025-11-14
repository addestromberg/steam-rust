#[derive(Debug, Copy, Clone)]
pub struct Temperature {
    kelvin: f64,
}

impl Temperature {
    pub fn from_kelvin(k: f64) -> Self {
        Self { kelvin: k }
    }

    pub fn from_celsius(c: f64) -> Self {
        Self { kelvin: c + 273.15 }
    }

    pub fn from_fahrenheit(f: f64) -> Self {
        Self { kelvin: (f - 32.0) * 5.0 / 9.0 + 273.15 }
    }

    pub fn as_kelvin(&self) -> f64 {
        self.kelvin
    }

    pub fn as_celsius(&self) -> f64 {
        self.kelvin - 273.15
    }

    pub fn as_fahrenheit(&self) -> f64 {
        (self.kelvin - 273.15) * 9.0 / 5.0 + 32.0
    }
}