#[derive(Debug)]
pub enum SteamError {
    OutOfRange { what: &'static str, value: f64 },
    InvalidInput { what: &'static str},
    NoConvergence { what: &'static str},
    RegionUnknown { what: &'static str},
}