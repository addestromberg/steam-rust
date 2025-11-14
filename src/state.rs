use crate::errors::*;


pub struct Steam {
    pub p: f64,         // Pressure in Pa
    pub t: f64,         // Temperature in K
    pub h: f64,         // Enthalpy in kJ/kg
    pub s: f64,         // Entropy in kJ/kg.K
    pub rho: f64,       // Density in kg/m3
    pub x: Option<f64>, // Quality (0 to 1) if in two-phase region
}

impl Steam {
    
    pub fn from_p_t(_p: f64, _t: f64) -> Result<Self, SteamError> {
        // Determine region based on p and t
        unimplemented!()
    }
    
    pub fn from_p_h(_p: f64, _h: f64) -> Result<Self, SteamError> {
        // Determine region based on p and h
        unimplemented!()
    }

    pub fn from_p_s(_p: f64, _s: f64) -> Result<Self, SteamError> {
        // Determine region based on p and s
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn from_p_t_test() {
        unimplemented!();
    }
}