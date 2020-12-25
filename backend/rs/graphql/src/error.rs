use crate::models::Status;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    BadRequest,
    InvalidGraphQLID,
    InvalidIDToken(String),
    InvalidPermissions,
    RequiresAdminPermissions,
    RequiresEditorPermissions,
    ResultUnavailable(Status),
    Serde,
    UnsupportedOperation,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = match self {
            Error::BadRequest => "Bad Request".into(),
            Error::InvalidGraphQLID => "Invalid GraphQL ID".into(),
            Error::InvalidIDToken(msg) => format!("Invalid ID Token: {}", msg),
            Error::InvalidPermissions => "Invalid Permissions".into(),
            Error::RequiresAdminPermissions => {
                "Requires Admin privileges".into()
            }
            Error::RequiresEditorPermissions => {
                "Requires Editor privileges".into()
            }
            Error::ResultUnavailable(status) => {
                format!("Result unavailable; status: {:?}", status)
            }
            Error::Serde => "Error (de)serializing".into(),
            Error::UnsupportedOperation => "Unsupported Operation".into(),
        };
        write!(f, "{}", &v)
    }
}
