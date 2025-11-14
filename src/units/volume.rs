#[derive(Debug, Copy, Clone)]
pub struct SpecificVolume {
    m3_kg: f64
}

impl SpecificVolume {
    
    pub fn as_m3_per_kg(&self) -> f64 {
        // Specific Volume Standard unit
        self.m3_kg  
    }

    pub fn from_m3_per_kg(v: f64) -> Self {
        // Get Specific Volume in standard unit
        Self {m3_kg: v}
    }

    pub fn as_density(&self) -> f64 {
        // Convert volume from density (kg/m3)
        1.0 / self.m3_kg
    }

    pub fn from_density(d: f64) -> Self {
        // Specific Volume from Density (kg/m3)
        Self {m3_kg: 1.0 / d}
    }

    pub fn as_l_per_kg(&self) -> f64 {
        // Specific volume as litre/kg
        self.m3_kg * 1000.0
    }

    pub fn from_l_per_kg(v: f64) -> Self {
        // Specific volume from l/kg.
        Self { m3_kg: v / 1000.0 }
    } 
 
}