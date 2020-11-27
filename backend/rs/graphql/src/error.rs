use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    BadRequest,
    InvalidIDToken,
    InvalidPermissions,
    RequiresAdminPermissions,
    RequiresEditorPermissions,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = match self {
            Error::BadRequest => "Bad Request",
            Error::InvalidIDToken => "Invalid ID Token",
            Error::InvalidPermissions => "Invalid Permissions",
            Error::RequiresAdminPermissions => "Requires Admin privileges",
            Error::RequiresEditorPermissions => "Requires Editor privileges",
        };
        write!(f, "{}", v)
    }
}
