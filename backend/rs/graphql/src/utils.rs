use crate::GenericError;
use bytes::Bytes;
use serde::Serialize;
use std::str;
use uuid::Uuid;

pub fn as_bytes<T: Serialize>(v: &T) -> Result<Bytes, GenericError> {
    Ok(Bytes::from(serde_json::to_vec(v)?))
}

pub fn dataset_table_name(uuid: &Uuid) -> String {
    let uuid_str = str::replace(&uuid.to_string(), "-", "_");
    format!("dataset_{}", uuid_str)
}

pub fn dataview_view_name(uuid: &Uuid) -> String {
    let uuid_str = str::replace(&uuid.to_string(), "-", "_");
    format!("dataview_{}", uuid_str)
}

pub fn user_name_from_email(email: &str) -> String {
    email
        .split("@")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .replace(".", "_")
}
