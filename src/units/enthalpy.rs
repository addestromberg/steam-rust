

pub struct Enthalpy {
    kj_kg: f64,
}

impl Enthalpy {
    pub fn as_kj_per_kg(&self) -> f64 {
        // Enthalpy standard unit
        self.kj_kg
    }

    pub fn from_kj_per_kg(h: f64) -> Self {
        // Get Enthalpy in standard unit
        Self { kj_kg: h }
    }

    pub fn as_j_per_kg(&self) -> f64 {
        // Enthalpy in J/kg
        self.kj_kg * 1000.0
    }

    pub fn from_j_per_kg(h: f64) -> Self {
        // Enthalpy from J/kg
        Self { kj_kg: h / 1000.0 }
    }
}