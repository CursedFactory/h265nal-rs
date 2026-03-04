use crate::constants::{
    H265NAL_STATUS_DUMP_UNAVAILABLE, H265NAL_STATUS_INVALID_ARGUMENT, H265NAL_STATUS_OK,
    H265NAL_STATUS_PARSE_FAILURE,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidArgument,
    ParseFailure,
    DumpUnavailable,
    UnknownStatus(i32),
}

impl Error {
    fn from_status(status: i32) -> Self {
        match status {
            H265NAL_STATUS_INVALID_ARGUMENT => Self::InvalidArgument,
            H265NAL_STATUS_PARSE_FAILURE => Self::ParseFailure,
            H265NAL_STATUS_DUMP_UNAVAILABLE => Self::DumpUnavailable,
            other => Self::UnknownStatus(other),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArgument => write!(f, "invalid argument"),
            Self::ParseFailure => write!(f, "parse failure"),
            Self::DumpUnavailable => write!(f, "dump unavailable in this build"),
            Self::UnknownStatus(status) => write!(f, "unknown status code {status}"),
        }
    }
}

impl std::error::Error for Error {}

pub(crate) fn status_to_result(status: i32) -> Result<(), Error> {
    if status == H265NAL_STATUS_OK {
        Ok(())
    } else {
        Err(Error::from_status(status))
    }
}
