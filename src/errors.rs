#[derive(Debug)]
pub enum SteamError {
    OutOfRange,
    InvalidInput,
    NoConvergence,
    RegionUnknown,
}