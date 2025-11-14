use crate::constants::REGION4_N;
use crate::errors::SteamError;
use crate::pressure::Pressure;
use crate::temperature::Temperature;

fn solve_beta_from_pressure(pressure: Pressure) -> f64 {
    // Beräkna beta enligt Eq. (29a):
    // beta = (p/1 MPa)^0.25
    let ps = pressure.as_mpa();
    (ps / 1.0).powf(0.25)
}

pub fn tsat_from_p(pressure: Pressure) -> Result<Temperature, SteamError> {
    // Check pressure range
    // 611.213 Pa <= p <= 22.064 MPa
    if pressure.as_pa() < 611.213 || pressure.as_mpa() > 22.064 {
        return Err(SteamError::OutOfRange);
    }

    let n = REGION4_N;
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
        return Err(SteamError::OutOfRange);
    }
}

pub fn psat_from_t(_temperature: Temperature) -> Pressure {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tsat_at_x_bar() {
        let t1 = tsat_from_p(Pressure::from_bar(1f64));
        let t67 = tsat_from_p(Pressure::from_bar(67.0f64));
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
}
