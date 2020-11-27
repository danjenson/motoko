use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Pool = sqlx::PgPool;

pub struct ModelKeys {
    pub model: String,
    pub keys: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UploadDatasetPayload {
    pub uri: String,
    pub uuid: Uuid,
}
