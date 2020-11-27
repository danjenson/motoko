use crate::{
    models::{Project, UserRefreshToken},
    types::Pool,
    utils::{data, is_current_user},
};
use async_graphql::{Context, Result as GQLResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, Result as SQLxResult};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct User {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uuid: Uuid,
    pub display_name: String,
    pub name: String,
    pub email: String,
}

impl User {
    pub async fn create(
        pool: &Pool,
        display_name: &str,
        name: &str,
        email: &str,
    ) -> SQLxResult<Self> {
        query_as(
            r#"
            INSERT INTO users (display_name, name, email)
            VALUES ($1, $2, $3) RETURNING *
            "#,
        )
        .bind(display_name)
        .bind(name)
        .bind(email)
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &Pool, uuid: &Uuid) -> SQLxResult<Self> {
        query_as("SELECT * FROM users WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(pool)
            .await
    }

    pub async fn get_by_email(pool: &Pool, email: &str) -> SQLxResult<Self> {
        query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(pool)
            .await
    }

    pub async fn get_by_name(pool: &Pool, name: &str) -> SQLxResult<Self> {
        query_as("SELECT * FROM users WHERE name = $1")
            .bind(name)
            .fetch_one(pool)
            .await
    }

    pub async fn rename(
        pool: &Pool,
        uuid: &Uuid,
        name: &str,
    ) -> SQLxResult<Self> {
        query_as(
            r#"
            UPDATE users
            SET name = $2
            WHERE uuid = $1
            RETURNING *
            "#,
        )
        .bind(uuid)
        .bind(name)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &Pool, uuid: &Uuid) -> SQLxResult<()> {
        query("DELETE FROM users WHERE uuid = $1")
            .bind(uuid)
            .execute(pool)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl User {
    // pub async fn id(&self) -> ID {
    //     "a".into()
    // }

    pub async fn created_at(
        &self,
        ctx: &Context<'_>,
    ) -> GQLResult<DateTime<Utc>> {
        is_current_user(self, ctx)?;
        Ok(self.created_at)
    }

    pub async fn updated_at(
        &self,
        ctx: &Context<'_>,
    ) -> GQLResult<DateTime<Utc>> {
        is_current_user(self, ctx)?;
        Ok(self.updated_at)
    }

    pub async fn display_name(&self) -> &str {
        &self.display_name
    }

    pub async fn name(&self) -> &str {
        &self.name
    }

    pub async fn email(&self) -> &str {
        &self.email
    }

    pub async fn projects(&self, ctx: &Context<'_>) -> GQLResult<Vec<Project>> {
        is_current_user(self, ctx)?;
        let d = data(ctx)?;
        query_as(
            r#"
            SELECT p.*
            FROM projects p
            JOIN project_user_roles pur
            ON p.uuid = pur.project_uuid
            AND pur.user_uuid = $1
            "#,
        )
        .bind(self.uuid)
        .fetch_all(&d.pool)
        .await
        .map_err(|e| e.into())
    }

    pub async fn refresh_tokens(
        &self,
        ctx: &Context<'_>,
    ) -> GQLResult<Vec<UserRefreshToken>> {
        is_current_user(self, ctx)?;
        let d = data(ctx)?;
        query_as("SELECT * FROM user_refresh_tokens WHERE user_uuid = $1")
            .bind(self.uuid)
            .fetch_all(&d.pool)
            .await
            .map_err(|e| e.into())
    }
}
