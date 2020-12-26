use crate::models::{PlotType, StatisticType};
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

pub type Json = serde_json::Value;
pub type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
pub type Vars<'v> = [(&'v str, &'v str)];

#[derive(Debug, Clone)]
pub struct Db {
    pub meta: sqlx::PgPool,
    pub data: sqlx::PgPool,
}

pub struct ModelKeys {
    pub model: String,
    pub keys: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadDatasetPayload {
    pub uri: String,
    pub uuid: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePlotPayload {
    pub view: String,
    pub uuid: Uuid,
    #[serde(rename = "type")]
    pub type_: PlotType,
    pub args: Json,
}

#[derive(Serialize, Deserialize)]
pub struct CreateStatisticPayload {
    pub view: String,
    pub uuid: Uuid,
    #[serde(rename = "type")]
    pub type_: StatisticType,
    pub args: Json,
}

#[derive(
    Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow, SimpleObject,
)]
#[serde(rename_all = "camelCase")]
pub struct ColumnDataType {
    pub column_name: String,
    pub data_type: String,
}
