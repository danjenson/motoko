use crate::{
    db::Pool,
    models::{analysis::Analysis, project_user_role::Role, status::Status},
    utils::get_data,
};
use async_graphql::{Context, Enum, FieldResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use sqlx::{query, query_as, FromRow, Result, Type};
use uuid::Uuid;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, Type,
)]
#[sqlx(rename = "DATAVIEW_OPERATION")]
#[sqlx(rename_all = "snake_case")]
pub enum Operation {
    Create,
    Filter,
    Mutate,
    Select,
    Sort,
    Summarize,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Dataview {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub analysis_uuid: Uuid,
    pub uuid: Uuid,
    pub parent_uuid: Uuid,
    pub operation: Operation,
    pub args: Option<Json>,
    pub status: Status,
}

impl Dataview {
    pub async fn create(
        pool: &Pool,
        dataview_uuid: &Uuid,
        operation: &Operation,
        args: &Json,
    ) -> Result<Self> {
        query_as(
            r#"
            INSERT INTO dataviews (
                analysis_uuid,
                parent_uuid,
                operation,
                args
            )
            SELECT analysis_uuid, uuid, $2, $3
            FROM dataviews
            WHERE uuid = $1
            RETURNING *
            "#,
        )
        .bind(dataview_uuid)
        .bind(operation)
        .bind(args)
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &Pool, uuid: &Uuid) -> Result<Self> {
        query_as("SELECT * FROM dataviews WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(pool)
            .await
    }

    pub async fn role(
        pool: &Pool,
        uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> Result<Role> {
        // TODO(danj): update once sqlx allows enums to derive FromRow
        let row: (Role,) = query_as(
            r#"
            SELECT pur.role as "role: Role"
            FROM dataviews dv
            JOIN analyses a
            ON dv.analysis_uuid = a.uuid
            AND dv.uuid = $1
            JOIN datasets d
            ON a.dataset_uuid = d.uuid
            JOIN project_user_roles pur
            ON d.project_uuid = pur.project_uuid
            AND pur.user_uuid = $2
            "#,
        )
        .bind(uuid)
        .bind(user_uuid)
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }

    pub async fn delete(pool: &Pool, uuid: &Uuid) -> Result<()> {
        query("DELETE FROM dataviews WHERE uuid = $1")
            .bind(uuid)
            .execute(pool)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Dataview {
    pub async fn created_at(&self) -> FieldResult<DateTime<Utc>> {
        Ok(self.created_at)
    }

    pub async fn updated_at(&self) -> FieldResult<DateTime<Utc>> {
        Ok(self.updated_at)
    }

    pub async fn analysis(&self, ctx: &Context<'_>) -> FieldResult<Analysis> {
        let d = get_data(ctx)?;
        Analysis::get(&d.pool, &self.analysis_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn parent(&self, ctx: &Context<'_>) -> FieldResult<Self> {
        let d = get_data(ctx)?;
        Self::get(&d.pool, &self.parent_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn operation(&self) -> FieldResult<&Operation> {
        Ok(&self.operation)
    }

    pub async fn args(&self) -> FieldResult<Option<String>> {
        Ok(self.args.clone().map(|v| v.to_string()))
    }

    pub async fn status(&self) -> FieldResult<&Status> {
        Ok(&self.status)
    }
}
