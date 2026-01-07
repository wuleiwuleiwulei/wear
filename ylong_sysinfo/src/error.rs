use std::fmt::{Debug, Display, Formatter};
use std::io::Error as OsError;

/// `Error` structure of ylong_sysinfo.
#[derive(Debug)]
pub enum Error {
    Os(OsError),
    Internal(InnerError),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Os(err) => write!(f, "{err}"),
            Error::Internal(err) => {
                let str: &str = err.into();
                write!(f, "{str}")
            }
        }
    }
}

/// `InnerError` structure of ylong_sysinfo.
#[derive(Debug)]
pub enum InnerError {
    CreateQueryFailed,
    AddCounterFailed,
    UpdateQueryFailed,
    CannotConvert,
    GetVolumeInfoFailed,
    GetDiskFreeSpaceFailed,
    NotEnoughMemory,
    GetAdaptersAddressesFailed,
    GetIPAddressFailed,
    GetMemoryFailed,
    GetProcessesFailed,
    NotFound,
    Overflow,
    ParseIntError,
    LenSizeError,
    ReadError(String),
}

impl<'a> From<&'a InnerError> for &'a str {
    fn from(error: &'a InnerError) -> Self {
        match error {
            InnerError::CreateQueryFailed => "Creates query failed",
            InnerError::AddCounterFailed => "Adds counter failed",
            InnerError::UpdateQueryFailed => "Updates query failed",
            InnerError::CannotConvert => "Cannot convert number to DriveType",
            InnerError::GetVolumeInfoFailed => "Gets volume info failed",
            InnerError::GetDiskFreeSpaceFailed => "Gets disk free space failed",
            InnerError::NotEnoughMemory => "Not enough memory",
            InnerError::GetAdaptersAddressesFailed => "Gets adapters addresses failed",
            InnerError::GetIPAddressFailed => "Gets ip address failed",
            InnerError::GetMemoryFailed => "Gets memory status failed",
            InnerError::GetProcessesFailed => "Gets processed information failed",
            InnerError::NotFound => "NotFound",
            InnerError::Overflow => "Overflow",
            InnerError::ParseIntError => "ParseIntError",
            InnerError::LenSizeError => "Parts len less than 2",
            InnerError::ReadError(content) => content,
        }
    }
}

impl std::error::Error for Error {}

#[cfg(target_os = "linux")]
pub(crate) fn read_error(name: &str, part: &str, err: impl std::error::Error) -> Error {
    Error::Internal(InnerError::ReadError(format!(
        "Get {name} {part} fail, error is {err:?}."
    )))
}

#[cfg(target_os = "linux")]
pub(crate) fn find_error(name: &[&str], file: &str) -> Error {
    Error::Internal(InnerError::ReadError(format!(
        "{name:?} is not found in {file}."
    )))
}
