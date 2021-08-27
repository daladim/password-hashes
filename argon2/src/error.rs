//! Error type

use core::fmt;

#[cfg(feature = "password-hash")]
use password_hash::errors::InvalidValue;

/// Result with argon2's [`Error`] type.
pub type Result<T> = core::result::Result<T, Error>;

/// Error type.
// TODO(tarcieri): consolidate/replace with `password_hash::Error`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// Associated data is too long.
    AdTooLong,

    /// Algorithm identifier invalid.
    AlgorithmInvalid,

    /// Memory cost is too small.
    MemoryTooLittle,

    /// Memory cost is too large.
    MemoryTooMuch,

    /// Output is too short.
    OutputTooShort,

    /// Output is too long.
    OutputTooLong,

    /// Password is too long.
    PwdTooLong,

    /// Salt is too short.
    SaltTooShort,

    /// Salt is too long.
    SaltTooLong,

    /// Secret is too long.
    SecretTooLong,

    /// Not enough threads.
    ThreadsTooFew,

    /// Too many threads.
    ThreadsTooMany,

    /// Time cost is too small.
    TimeTooSmall,

    /// Invalid version
    VersionInvalid,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Error::AdTooLong => "associated data is too long",
            Error::AlgorithmInvalid => "algorithm identifier invalid",
            Error::MemoryTooLittle => "memory cost is too small",
            Error::MemoryTooMuch => "memory cost is too large",
            Error::OutputTooShort => "output is too short",
            Error::OutputTooLong => "output is too long",
            Error::PwdTooLong => "password is too long",
            Error::SaltTooShort => "salt is too short",
            Error::SaltTooLong => "salt is too long",
            Error::SecretTooLong => "secret is too long",
            Error::ThreadsTooFew => "not enough threads",
            Error::ThreadsTooMany => "too many threads",
            Error::TimeTooSmall => "time cost is too small",
            Error::VersionInvalid => "invalid version",
        })
    }
}

#[cfg(feature = "password-hash")]
#[cfg_attr(docsrs, doc(cfg(feature = "password-hash")))]
impl From<Error> for password_hash::Error {
    fn from(err: Error) -> password_hash::Error {
        match err {
            Error::AdTooLong => password_hash::Error::ParamValueInvalid(InvalidValue::TooLong),
            Error::AlgorithmInvalid => password_hash::Error::Algorithm,
            Error::MemoryTooLittle => {
                password_hash::Error::ParamValueInvalid(InvalidValue::TooShort)
            }
            Error::MemoryTooMuch => password_hash::Error::ParamValueInvalid(InvalidValue::TooLong),
            Error::PwdTooLong => password_hash::Error::Password,
            Error::OutputTooShort => password_hash::Error::OutputTooShort,
            Error::OutputTooLong => password_hash::Error::OutputTooLong,
            Error::SaltTooShort => password_hash::Error::SaltInvalid(InvalidValue::TooShort),
            Error::SaltTooLong => password_hash::Error::SaltInvalid(InvalidValue::TooLong),
            Error::SecretTooLong => password_hash::Error::ParamValueInvalid(InvalidValue::TooLong),
            Error::ThreadsTooFew => password_hash::Error::ParamValueInvalid(InvalidValue::TooShort),
            Error::ThreadsTooMany => password_hash::Error::ParamValueInvalid(InvalidValue::TooLong),
            Error::TimeTooSmall => password_hash::Error::ParamValueInvalid(InvalidValue::TooShort),
            Error::VersionInvalid => password_hash::Error::Version,
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
