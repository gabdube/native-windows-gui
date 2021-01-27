use nwg::NwgError;
use std::ffi::OsString;

pub enum CreateProjectError {
    BadPath(OsString),
    Internal(NwgError)
}

