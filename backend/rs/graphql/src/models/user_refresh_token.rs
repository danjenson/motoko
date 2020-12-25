use crate::{gql::data, models::User, types::Db};
use async_graphql::{Context, Result as GQLResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, Result as SQLxResult};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct UserRefreshToken {
    pub expires_at: DateTime<Utc>,
    pub user_uuid: Uuid,
    pub value: String,
}

impl UserRefreshToken {
    pub async fn create(
        db: &Db,
        user_uuid: &Uuid,
        value: &str,
        expires_at: &DateTime<Utc>,
    ) -> SQLxResult<Self> {
        query_as(
            r#"
            INSERT INTO user_refresh_tokens (user_uuid, value, expires_at)
            VALUES ($1, $2, $3) RETURNING *
            "#,
        )
        .bind(user_uuid)
        .bind(value)
        .bind(expires_at)
        .fetch_one(&db.meta)
        .await
    }

    pub async fn get(db: &Db, value: &str) -> SQLxResult<Self> {
        query_as("SELECT * FROM user_refresh_tokens WHERE value = $1")
            .bind(value)
            .fetch_one(&db.meta)
            .await
    }

    pub async fn delete(db: &Db, value: &str) -> SQLxResult<()> {
        query("DELETE FROM user_refresh_tokens WHERE value = $1")
            .bind(value)
            .execute(&db.meta)
            .await
            .map(|_| ())
    }
}

#[node(value)]
#[async_graphql::Object]
impl UserRefreshToken {
    pub async fn user(&self, ctx: &Context<'_>) -> GQLResult<User> {
        let d = data(ctx)?;
        User::get(&d.db, &self.user_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn value(&self) -> &String {
        &self.value
    }

    pub async fn expires_at(&self) -> DateTime<Utc> {
        self.expires_at
    }
}
