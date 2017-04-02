//! Encapsulates error types.

use std::result;
use std::fmt;

pub type Result<T> = result::Result<T, Error>;

/// Possible errors produced by syscall collector.
#[derive(Deserialize, Debug)]
pub enum Error {
    RingBufferMapping,
    TooManyCollectors,
    DeviceError,
    UnknownConfigPathError,
    ConfigParseError(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RingBufferMapping => write!(f, "Unable to map ring buffer device"),
            Error::TooManyCollectors => write!(f, "Too many collectors attached to device"),
            Error::DeviceError => write!(f, "Insufficient privileges to open device \
                                        or device not loaded"),
            Error::UnknownConfigPathError => write!(f, "Unable to resolve \
                                                configuration file path"),
            Error::ConfigParseError(ref e) => write!(f, "Invalid configuration descriptor. \
                                               Reason: {}", e)
        }
    }
}
