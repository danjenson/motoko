use crate::{db::Pool, models::user::User, utils::get_data};
use async_graphql::{Context, FieldResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, Result};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct UserRefreshToken {
    pub expires_at: DateTime<Utc>,
    pub user_uuid: Uuid,
    pub value: String,
}

impl UserRefreshToken {
    pub async fn create(
        pool: &Pool,
        user_uuid: &Uuid,
        value: &str,
        expires_at: &DateTime<Utc>,
    ) -> Result<Self> {
        query_as(
            r#"
            INSERT INTO user_refresh_tokens (user_uuid, value, expires_at)
            VALUES ($1, $2, $3) RETURNING *
            "#,
        )
        .bind(user_uuid)
        .bind(value)
        .bind(expires_at)
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &Pool, value: &str) -> Result<Self> {
        query_as("SELECT * FROM user_refresh_tokens WHERE value = $1")
            .bind(value)
            .fetch_one(pool)
            .await
    }

    pub async fn delete(&self, pool: &Pool) -> Result<()> {
        query("DELETE FROM user_refresh_tokens WHERE value = $1")
            .bind(&self.value)
            .execute(pool)
            .await
            .map(|_| ())
    }
}

#[node(value)]
#[async_graphql::Object]
impl UserRefreshToken {
    pub async fn user(&self, ctx: &Context<'_>) -> FieldResult<User> {
        let d = get_data(ctx)?;
        User::get(&d.pool, &self.user_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn value(&self) -> FieldResult<String> {
        Ok(self.value.clone())
    }

    pub async fn expires_at(&self) -> FieldResult<DateTime<Utc>> {
        Ok(self.expires_at)
    }
}
