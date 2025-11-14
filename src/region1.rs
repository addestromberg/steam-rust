use crate::constants::{REGION_1_TABLE, R};
use crate::errors::SteamError;
use crate::saturation;
use crate::units::{Pressure, SpecificVolume, Temperature};

// ---------------------------------------------------------
// ErrorChecks
// ---------------------------------------------------------



// ---------------------------------------------------------
// Reduced variables
// ---------------------------------------------------------

fn pi(pressure: Pressure) -> f64 {
    pressure.as_mpa() / 16.53
}

fn tau(temperature: Temperature) -> f64 {
    1386.0 / temperature.as_kelvin()
}

// ---------------------------------------------------------
// Fundamental equations γ, γπ, γτ
// (all take reduced variables directly: pi, tau)
// ---------------------------------------------------------

fn gamma(pi: f64, tau: f64) -> f64 {
    let a = 7.1 - pi;
    let b = tau - 1.222;

    let mut sum = 0.0;

    for coeff in REGION_1_TABLE.iter().skip(1) {
        sum += coeff.n * a.powi(coeff.i) * b.powi(coeff.j);
    }

    sum
}

fn gamma_pi(pi: f64, tau: f64) -> f64 {
    let a = 7.1 - pi;
    let b = tau - 1.222;

    let mut sum = 0.0;

    for coeff in REGION_1_TABLE.iter().skip(1) {
        if coeff.i == 0 {
            continue; // derivative = 0
        }

        sum += -(coeff.i as f64) *
                coeff.n *
                a.powi(coeff.i - 1) *
                b.powi(coeff.j);
    }

    sum
}

fn gamma_tau(pi: f64, tau: f64) -> f64 {
    let a = 7.1 - pi;
    let b = tau - 1.222;

    let mut sum = 0.0;

    for coeff in REGION_1_TABLE.iter().skip(1) {
        if coeff.j == 0 {
            continue; // derivative = 0
        }

        sum += (coeff.j as f64) *
               coeff.n *
               a.powi(coeff.i) *
               b.powi(coeff.j - 1);
    }

    sum
}

// ---------------------------------------------------------
// Public API: Specific Volume
// ---------------------------------------------------------

pub fn volume(pressure: Pressure, temperature: Temperature) -> Result<SpecificVolume, SteamError> {
    // Get specific volume for given pressure and temperature in Region 1
    
    // Check if inputs are within valid ranges for Region 1
    if temperature.as_kelvin() < 273.15 {
        return Err(SteamError::OutOfRange { what: "Crossing lower boundary for region 1", value: temperature.as_kelvin() });
    }
    // Check that we aren't crossing saturation curve
    if temperature.as_kelvin() > saturation::ts(pressure)?.as_kelvin() {
        return Err(SteamError::RegionUnknown { what: "Crossed saturation line."} );
    }

    // Check pressure is lower than saturation line.
    if pressure.as_mpa() < saturation::ps(temperature)?.as_mpa() {
        return Err(SteamError::RegionUnknown { what: "Crossed saturation line." });
    }
    
    let t = temperature.as_kelvin();
    let p = pressure.as_mpa();

    // Compute reduced variables once
    let pi_val = pi(pressure);
    let tau_val = tau(temperature);

    // Compute derivative γπ(π, τ)
    let gpi = gamma_pi(pi_val, tau_val);

    // IF97 Region 1 volume equation
    let v = (R * t / p) * pi_val * gpi;

    // println!("DEBUG p_mpa    = {}", p);
    // println!("DEBUG t_kelvin = {}", t);
    // println!("DEBUG pi       = {}", pi_val);
    // println!("DEBUG tau      = {}", tau_val);
    // println!("DEBUG g_pi     = {}", gpi);
    // println!("DEBUG R        = {}", R);

    // Return calclated specific volume
    Ok(SpecificVolume::from_m3_per_kg(v))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_volume() {
        let t: Temperature = Temperature::from_celsius(20.0);
        let p = Pressure::from_bar(1.0);
        let v = volume(p, t);
        match v {
            Ok(sv) => {
                let v_calc = sv.as_m3_per_kg();
                let v_expected = 0.0010018; // Approximate expected value
                let diff = (v_calc - v_expected).abs();
                println!("Calculated volume: {}, Expected volume: {}", v_calc, v_expected);
                assert!(diff < 0.01, "Calculated volume {} differs from expected {}", v_calc, v_expected);
            },
            Err(e) => panic!("Error calculating volume: {:?}", e),
        }       
        
    }
}
