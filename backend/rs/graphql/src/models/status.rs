use async_graphql::Enum;
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, Type,
)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(rename = "STATUS")]
#[sqlx(rename_all = "lowercase")]
pub enum Status {
    Queued,
    Running,
    Completed,
    Failed,
}
