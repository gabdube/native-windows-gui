use nwg::NwgError;
use std::{fmt, ffi::OsString};

pub enum CreateProjectError {
    BadPath(OsString),
    Internal(NwgError)
}

impl fmt::Display for CreateProjectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CreateProjectError::BadPath(path) => {
                write!(f, "The selected path is not a valid utf-8 string: {:?}", path)
            },
            CreateProjectError::Internal(e) => {
                write!(f, "A system error ocurred while reading the path: {:?}", e)
            },
        }
    }
}
