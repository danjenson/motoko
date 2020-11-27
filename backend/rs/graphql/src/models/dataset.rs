use crate::{
    models::{Project, Role, Status},
    types::Pool,
    utils::data,
};
use async_graphql::{Context, Result as GQLResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, Result as SQLxResult};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Dataset {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub project_uuid: Uuid,
    pub uuid: Uuid,
    pub name: String,
    pub uri: String,
    pub status: Status,
}

impl Dataset {
    pub async fn create(
        pool: &Pool,
        project_uuid: &Uuid,
        name: &str,
        uri: &str,
    ) -> SQLxResult<Self> {
        query_as(
            r#"
            INSERT INTO datasets (project_uuid, name, uri)
            VALUES ($1, $2, $3) RETURNING *
            "#,
        )
        .bind(project_uuid)
        .bind(name)
        .bind(uri)
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &Pool, uuid: &Uuid) -> SQLxResult<Self> {
        query_as("SELECT * FROM datasets WHERE uuid = $1")
            .bind(uuid)
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
            UPDATE datasets
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

    pub async fn role(
        pool: &Pool,
        uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> SQLxResult<Role> {
        let row: (Role,) = query_as(
            r#"
            SELECT pur.role as "role: Role"
            FROM datasets d
            JOIN project_user_roles pur
            ON d.project_uuid = pur.project_uuid
            AND d.uuid = $1
            AND pur.user_uuid = $2
            "#,
        )
        .bind(&uuid)
        .bind(&user_uuid)
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }

    pub async fn delete(pool: &Pool, uuid: &Uuid) -> SQLxResult<()> {
        query("DELETE FROM datasets WHERE uuid = $1")
            .bind(uuid)
            .execute(pool)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Dataset {
    pub async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub async fn project(&self, ctx: &Context<'_>) -> GQLResult<Project> {
        let d = data(ctx)?;
        Project::get(&d.pool, &self.project_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn name(&self) -> &String {
        &self.name
    }

    pub async fn status(&self) -> &Status {
        &self.status
    }
}
