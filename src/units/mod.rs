#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pressure(f64); // Pa

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Temperature(f64); // Kelvin

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpecificVolume(f64); // m3/kg

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpecificEnthalpy(f64); // J/kg

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpecificEntropy(f64); // J/(kg*K)

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Density(f64); // kg/m3

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpecificInternalEnergy(f64); // J/kg

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpecificHeatCapacity(f64); // cp/cv J/(kg*K)

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DynamicViscosity(f64); // Pa*s

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalConductivity(f64); // W/(m*K)

impl Pressure {
    pub fn into_inner(self) -> f64 { self.0 }

    pub fn from_pa(value: f64) -> Self { Pressure(value) }
    pub fn from_mpa(value: f64) -> Self { Pressure(value * 1_000_000.0) }
    pub fn as_pa(self) -> f64 { self.0 }
    pub fn as_mpa(self) -> f64 { self.0 / 1_000_000.0 }

    pub fn from_si(value: f64) -> Self { Self::from_pa(value) }
    pub fn as_si(self) -> f64 { self.as_pa() }
}

impl Temperature {
    pub fn into_inner(self) -> f64 { self.0 }

    pub fn from_kelvin(value: f64) -> Self { Temperature(value) }
    pub fn from_celsius(value: f64) -> Self { Temperature(value + 273.15) }
    pub fn from_fahrenheit(value: f64) -> Self {
        Temperature((value - 32.0) * 5.0 / 9.0 + 273.15)
    }
    pub fn as_kelvin(self) -> f64 { self.0 }
    pub fn as_celsius(self) -> f64 { self.0 - 273.15 }
    pub fn as_fahrenheit(self) -> f64 {
        (self.0 - 273.15) * 9.0 / 5.0 + 32.0
    }

    pub fn from_si(value: f64) -> Self { Self::from_kelvin(value) }
    pub fn as_si(self) -> f64 { self.as_kelvin() }
}

impl SpecificVolume {
    pub fn into_inner(self) -> f64 { self.0 }

    pub fn from_m3_per_kg(value: f64) -> Self { SpecificVolume(value) }
    pub fn as_m3_per_kg(self) -> f64 { self.0 }

    pub fn from_si(value: f64) -> Self { Self::from_m3_per_kg(value) }
    pub fn as_si(self) -> f64 { self.as_m3_per_kg() }
}

impl SpecificEnthalpy {
    pub fn into_inner(self) -> f64 { self.0 }

    pub fn from_j_per_kg(value: f64) -> Self { SpecificEnthalpy(value) }
    pub fn from_kj_per_kg(value: f64) -> Self { SpecificEnthalpy(value * 1000.0) }
    pub fn as_j_per_kg(self) -> f64 { self.0 }
    pub fn as_kj_per_kg(self) -> f64 { self.0 / 1000.0 }

    pub fn from_si(value: f64) -> Self { Self::from_j_per_kg(value) }
    pub fn as_si(self) -> f64 { self.as_j_per_kg() }
}

impl SpecificEntropy {
    pub fn into_inner(self) -> f64 { self.0 }

    pub fn from_j_per_kg_k(value: f64) -> Self { SpecificEntropy(value) }
    pub fn from_kj_per_kg_k(value: f64) -> Self { SpecificEntropy(value  * 1000.0) }
    pub fn as_j_per_kg_k(self) -> f64 { self.0}
    pub fn as_kj_per_kg_k(self) -> f64 { self.0 / 1000.0 }


    pub fn from_si(value: f64) -> Self { Self::from_j_per_kg_k(value) }
    pub fn as_si(self) -> f64 { self.as_j_per_kg_k() }
}

impl Density {
    pub fn into_inner(self) -> f64 { self.0 }

    pub fn from_kg_per_m3(value: f64) -> Self { Density(value) }
    pub fn as_kg_per_m3(self) -> f64 { self.0 }

    pub fn from_si(value: f64) -> Self { Self::from_kg_per_m3(value) }
    pub fn as_si(self) -> f64 { self.as_kg_per_m3() }
}

impl SpecificInternalEnergy {
    pub fn into_inner(self) -> f64 { self.0 }

    pub fn from_j_per_kg(value: f64) -> Self { SpecificInternalEnergy(value) }
    pub fn as_j_per_kg(self) -> f64 { self.0 }

    pub fn from_si(value: f64) -> Self { Self::from_j_per_kg(value) }
    pub fn as_si(self) -> f64 { self.as_j_per_kg() }
}

impl SpecificHeatCapacity {
    pub fn into_inner(self) -> f64 { self.0 }

    pub fn from_j_per_kg_k(value: f64) -> Self { SpecificHeatCapacity(value) }
    pub fn as_j_per_kg_k(self) -> f64 { self.0 }

    pub fn from_si(value: f64) -> Self { Self::from_j_per_kg_k(value) }
    pub fn as_si(self) -> f64 { self.as_j_per_kg_k() }
}

impl DynamicViscosity {
    pub fn into_inner(self) -> f64 { self.0 }

    pub fn from_pa_s(value: f64) -> Self { DynamicViscosity(value) }
    pub fn as_pa_s(self) -> f64 { self.0 }

    pub fn from_si(value: f64) -> Self { Self::from_pa_s(value) }
    pub fn as_si(self) -> f64 { self.as_pa_s() }
}

impl ThermalConductivity {
    pub fn into_inner(self) -> f64 { self.0 }

    pub fn from_w_per_m_k(value: f64) -> Self { ThermalConductivity(value) }
    pub fn as_w_per_m_k(self) -> f64 { self.0 }

    pub fn from_si(value: f64) -> Self { Self::from_w_per_m_k(value) }
    pub fn as_si(self) -> f64 { self.as_w_per_m_k() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pressure_conversions() {
        let p1 = Pressure::from_pa(1000.0);
        let p2 = Pressure::from_mpa(0.001);
        assert_eq!(p1, p2);
        assert_eq!(p1.as_pa(), 1000.0);
        assert_eq!(p1.as_mpa(), 0.001);
    }

    #[test]
    fn temperature_conversion() {
        let t1 = Temperature::from_celsius(0.0);
        let t2 = Temperature::from_kelvin(273.15);
        let t3 = Temperature::from_fahrenheit(32.0);
        assert_eq!(t1, t2);
        assert_eq!(t1, t3);
        assert_eq!(t1.as_celsius(), 0.0);
        assert_eq!(t1.as_kelvin(), 273.15);
        assert_eq!(t1.as_fahrenheit(), 32.0);
    }

    #[test]
    fn specific_volume_conversion() {
        let v = SpecificVolume::from_m3_per_kg(0.001);
        assert_eq!(v.as_m3_per_kg(), 0.001);
    }
    
    #[test]
    fn specific_enthalpy_conversion() {
        let h1 = SpecificEnthalpy::from_j_per_kg(1000.0);
        let h2 = SpecificEnthalpy::from_kj_per_kg(1.0);
        assert_eq!(h1, h2);
        assert_eq!(h1.as_j_per_kg(), 1000.0);
        assert_eq!(h1.as_kj_per_kg(), 1.0);
    }
    
    #[test]
    fn specific_entropy_conversion() {
        let s1 = SpecificEntropy::from_j_per_kg_k(1000.0);
        let s2 = SpecificEntropy::from_kj_per_kg_k(1.0);
        assert_eq!(s1, s2);
        assert_eq!(s1.as_j_per_kg_k(), 1000.0);
        assert_eq!(s1.as_kj_per_kg_k(), 1.0);
    }

    #[test]
    fn density_conversion() {
        let d = Density::from_kg_per_m3(1000.0);
        assert_eq!(d.into_inner(), 1000.0);
    }

    #[test]
    fn specific_internal_energy_conversion() {
        let u = SpecificInternalEnergy::from_j_per_kg(500.0);
        assert_eq!(u.into_inner(), 500.0);
    }
    
    #[test]
    fn specific_heat_capacity_conversion() {
        let c = SpecificHeatCapacity::from_j_per_kg_k(4200.0);
        assert_eq!(c.into_inner(), 4200.0);
    }

    #[test]
    fn dynamic_viscosity_conversion() {
        let mu = DynamicViscosity::from_pa_s(0.001);
        assert_eq!(mu.into_inner(), 0.001);
    }

    #[test]
    fn thermal_conductivity_conversion() {
        let k = ThermalConductivity::from_w_per_m_k(0.6);
        assert_eq!(k.into_inner(), 0.6);
    }
}
