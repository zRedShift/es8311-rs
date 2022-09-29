#[derive(Debug)]
pub enum Error<E> {
    BusError(E),
    InvalidConfiguration,
    WrongChip,
}

#[cfg(feature = "std")]
impl<E: std::fmt::Display> std::fmt::Display for Error<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::BusError(err) => write!(f, "Bus Error: {err}"),
            Self::InvalidConfiguration => write!(f, "Invalid clock configuration"),
            Self::WrongChip => write!(f, "Wrong Chip"),
        }
    }
}

#[cfg(feature = "std")]
impl<E: std::error::Error> std::error::Error for Error<E> {}
