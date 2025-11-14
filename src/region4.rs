use crate::constants::REGION_4_TABLE;
use crate::errors::SteamError;
use crate::units::{Pressure, Temperature};

//////////////// HELPERS ////////////////////////
fn solve_beta_from_pressure(pressure: Pressure) -> f64 {
    // Solve for beta from pressure in Region 4
    // Beräkna beta enligt Eq. (29a):
    // beta = (p/1 MPa)^0.25
    let ps = pressure.as_mpa();
    (ps / 1.0).powf(0.25)
}

fn solve_theta_from_temperature(temperature: Temperature) -> f64 {
    // Solve for theta from temperature in Region 4
    // Beräkna theta enligt Eq. (29b):
    // theta = (T/1 K + n9 / (T/1 K) - n10)
    let ts = temperature.as_kelvin();
    let n = REGION_4_TABLE;

    ts + (n[9] / (ts - n[10]))
}

//////////////// PUBLIC FUNCTIONS ////////////////////////
pub fn ts(pressure: Pressure) -> Result<Temperature, SteamError> {
    // Calculate saturation temperature from pressure in Region 4

    // Check pressure range
    // 611.213 Pa <= p <= 22.064 MPa
    if pressure.as_pa() < 611.213 || pressure.as_mpa() > 22.064 {
        return Err(SteamError::OutOfRange {
            what: "Pressure",
            value: pressure.as_mpa(),
        });
    }

    let n = REGION_4_TABLE;
    let b = solve_beta_from_pressure(pressure);

    // Lös sektion 8.2 Saturation-Temperature Equation (Backward Eq.) sid 35
    // Ts = Saturated Steam Temperature
    // Ts/1 = (n10+D-[((n10+D)^2) - 4*(n9 + n10 * D)]^1/2) / 2 (Master)
    // där D = 2G / (-F-(F^2-4EG))^1/2
    // där E = B² + n3 * B + n6
    // där F = n1*B² + n4*B + n7
    // där G = n2*B² + n5*B + n8

    // Lös baklänges
    let g = n[2] * b.powi(2) + n[5] * b + n[8];
    let f = n[1] * b.powi(2) + n[4] * b + n[7];
    let e = b.powi(2) + n[3] * b + n[6];

    let d = 2.0 * g / (-f - (f.powi(2) - 4.0 * e * g).sqrt());

    // Nu när vi har D och poly functionerna kan vi lösa för Ts
    let ts = (n[10] + d - ((n[10] + d).powi(2) - 4.0 * (n[9] + n[10] * d)).sqrt()) / 2.0;
    if ts > 273.15 && ts < 647.096 {
        return Ok(Temperature::from_kelvin(ts));
    } else {
        return Err(SteamError::OutOfRange {
            what: "Temperature",
            value: ts,
        });
    }
}

pub fn ps(temperature: Temperature) -> Result<Pressure, SteamError> {
    // Calculate saturation pressure from temperature in Region 4

    // Kolla T inom giltigt intervall
    if temperature.as_kelvin() < 273.15 || temperature.as_kelvin() > 647.096 {
        return Err(SteamError::OutOfRange {
            what: "Temperature",
            value: temperature.as_kelvin(),
        });
    }

    let theta = solve_theta_from_temperature(temperature);
    let n = REGION_4_TABLE;
    // Lös sektion 8.1 Saturation-Pressure Equation (Basic Eq.) sid 33
    // Ps = Saturated Steam Pressure
    // T* är 1 Kelvin
    // P* är 1 MPa
    // Ps/P* = ( 2c / -b + (b^2 - 4ac) )^4 (Master)

    // där theta = Ts/1 + n9 / (Ts/1) - n10
    // där a = theta^2 + n1*theta + n2
    // där b = n3*theta^2 + n4*theta + n5
    // där c = n6*theta^2 + n7*theta + n8
    let a = theta.powi(2) + n[1] * theta + n[2];
    let b = n[3] * theta.powi(2) + n[4] * theta + n[5];
    let c = n[6] * theta.powi(2) + n[7] * theta + n[8];

    let ps = (2.0 * c / (-b + (b.powi(2) - 4.0 * a * c).sqrt())).powi(4);

    if ps > 0.000611213 && ps < 22.064 {
        return Ok(Pressure::from_mpa(ps));
    } else {
        return Err(SteamError::OutOfRange {
            what: "Pressure",
            value: ps,
        });
    }
}

///////////////// TESTS ////////////////////////
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tsat_at_x_bar() {
        let t1 = ts(Pressure::from_bar(1f64));
        let t67 = ts(Pressure::from_bar(67.0f64));
        match t1 {
            Ok(temp) => {
                // Physical expected ~99.61°C = 372.76 K
                println!("T at 1 bar: {} C", temp.as_celsius());
                assert!(
                    (temp.as_kelvin() - 372.76).abs() < 0.5,
                    "T = {} C",
                    temp.as_celsius()
                );
            }
            Err(e) => panic!("Error: {:?}", e),
        }

        match t67 {
            Ok(temp) => {
                // Physical expected ~282.83°C = 556.98 K
                println!("T at 67 bar: {} C", temp.as_celsius());
            }
            Err(e) => panic!("Error: {:?}", e),
        }

        // Physical expected ~99.97°C = 373.15 K
        assert!(t1.is_ok(), "Error: {:?}", t1.err());
    }

    #[test]
    fn test_psat_at_x_celsius() {
        let t1 = ps(Temperature::from_celsius(99.605));
        match t1 {
            Ok(press) => {
                // Physical expected ~1 bar
                println!("P at 99.605 C: {} bar", press.as_bar());
                assert!(
                    (press.as_bar() - 1.0).abs() < 0.01,
                    "P = {} bar",
                    press.as_bar()
                );
            }
            Err(e) => panic!("Error: {:?}", e),
        }
    }
}
