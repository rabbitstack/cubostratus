//! Encapsulates error types.

use std::result;
use std::fmt;

pub type Result<T> = result::Result<T, Error>;

/// Possible errors produced by syscall collector.
pub enum Error {
    RingBufferMapping,
    TooManyCollectors,
    DeviceError
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::RingBufferMapping => write!(f, "Unable to map ring buffer device"),
            Error::TooManyCollectors => write!(f, "Too many collectors attached to device"),
            Error::DeviceError => write!(f, "Insufficient privileges to open device or device not loaded")
        }
    }
}