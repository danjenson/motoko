use crate::{GenericError, Json, Vars};
use async_graphql::Variables;
use bytes::Bytes;
use rusoto_core::Region;
use rusoto_credential::AwsCredentials;
use rusoto_s3::{
    util::PreSignedRequest, util::PreSignedRequestOption, GetObjectRequest,
};
use serde::Serialize;
use std::{collections::BTreeMap, env, str, time::Duration};
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

pub fn get_presigned_url(
    region: &Region,
    bucket: &str,
    key: &str,
    aws_credentials: &AwsCredentials,
) -> String {
    // maximum expiration is 7 days:
    // https://aws.amazon.com/premiumsupport/knowledge-center/presigned-url-s3-bucket-expiration/
    let req = GetObjectRequest {
        bucket: bucket.to_owned(),
        key: key.to_owned(),
        ..Default::default()
    };
    let seconds_per_week = 60 * 60 * 24 * 7;
    let opt = PreSignedRequestOption {
        expires_in: Duration::from_secs(seconds_per_week),
    };
    req.get_presigned_url(region, aws_credentials, &opt)
}

pub fn run_mode() -> String {
    env::var("RUN_MODE").expect("RUN_MODE not defined")
}

pub fn user_name_from_email(email: &str) -> String {
    email
        .split("@")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .replace(".", "_")
}

pub fn vars_to_json_string(vars: &Vars) -> String {
    vars_to_json(vars).to_string()
}

pub fn vars_to_json(vars: &Vars) -> Json {
    serde_json::to_value(vars.iter().cloned().collect::<BTreeMap<&str, &str>>())
        .unwrap()
}

pub fn vars_to_variables(vars: &Vars) -> Variables {
    Variables::from_json(vars_to_json(vars))
}
