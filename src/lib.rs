pub mod common;
pub mod constants;
pub mod errors;
pub mod pressure;
pub mod region1;
pub mod region2;
pub mod region3;
pub mod region4;
pub mod region5;
pub mod state;
pub mod temperature;

pub use state::Steam;

/// Saturation line functions (IF97 Region 4)
pub mod saturation {
    pub use crate::region4::{ps, ts};
}

#[cfg(test)]
mod tests {
    use crate::{pressure::Pressure, saturation, temperature::Temperature};

    #[test]
    fn test_prod_use() {
        let p = Pressure::from_bar(5.15);
        println!("Sauturation temperature at {} bar:", p.as_bar());
        let tsat = saturation::ts(p);
        match tsat {
            Ok(t) => println!("Tsat = {} °C", t.as_celsius()),
            Err(e) => println!("Error calculating Tsat: {:?}", e),
        }
    }

    #[test]
    fn test_debug_use() {
        let t = Temperature::from_celsius(282.0);
        let psat = saturation::ps(t);
        dbg!(t.as_celsius(), psat.unwrap().as_bar());
    }

    #[test]
    fn test_out_of_bounds() {
        let t = Temperature::from_celsius(1000.0);
        saturation::ps(t)
            .expect_err("psat_from_t should fail on unrealistic temperatures");
    }
}
