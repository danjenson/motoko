use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub type Db = sqlx::PgPool;
pub type Json = serde_json::Value;
pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub struct ModelKeys {
    pub model: String,
    pub keys: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadDatasetPayload {
    pub uri: String,
    pub uuid: Uuid,
}

#[derive(
    Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow, SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDataType {
    pub column_name: String,
    pub data_type: String,
}
