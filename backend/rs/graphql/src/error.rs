use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    BadRequest,
    InvalidGraphQLID,
    InvalidIDToken,
    InvalidPermissions,
    RequiresAdminPermissions,
    RequiresEditorPermissions,
    Serde,
    UnsupportedOperation,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = match self {
            Error::BadRequest => "Bad Request",
            Error::InvalidGraphQLID => "Invalid GraphQL ID",
            Error::InvalidIDToken => "Invalid ID Token",
            Error::InvalidPermissions => "Invalid Permissions",
            Error::RequiresAdminPermissions => "Requires Admin privileges",
            Error::RequiresEditorPermissions => "Requires Editor privileges",
            Error::Serde => "Error (de)serializing",
            Error::UnsupportedOperation => "Unsupported Operation",
        };
        write!(f, "{}", v)
    }
}
