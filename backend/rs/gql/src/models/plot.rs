use crate::{
    db::Pool,
    models::{dataview::Dataview, project_user_role::Role, status::Status},
    utils::get_data,
};
use async_graphql::{Context, Enum, FieldResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use sqlx::{query, query_as, FromRow, Result};
use uuid::Uuid;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, sqlx::Type,
)]
#[sqlx(rename = "PLOT_TYPE")]
#[sqlx(rename_all = "snake_case")]
pub enum Type {
    Bar,
    Histogram,
    Line,
    Scatter,
    Smooth,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Plot {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dataview_uuid: Uuid,
    pub uuid: Uuid,
    pub name: String,
    #[sqlx(rename = "type")]
    pub type_: Type,
    pub args: Json,
    pub status: Status,
    pub uri: Option<String>,
}

impl Plot {
    pub async fn create(
        pool: &Pool,
        dataview_uuid: &Uuid,
        name: &str,
        type_: &Type,
        args: &Json,
    ) -> Result<Self> {
        query_as(
            r#"
            INSERT INTO plots (dataview_uuid, name, type, args)
            VALUES ($1, $2, $3, $4) RETURNING *
            "#,
        )
        .bind(dataview_uuid)
        .bind(name)
        .bind(type_)
        .bind(args)
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &Pool, uuid: &Uuid) -> Result<Self> {
        query_as(
            r#"
            SELECT
                created_at,
                updated_at,
                dataview_uuid,
                uuid,
                name,
                type as "type_: Type",
                args,
                status as "status: Status",
                uri
            FROM plots
            WHERE uuid = $1"#,
        )
        .bind(uuid)
        .fetch_one(pool)
        .await
    }

    pub async fn rename(pool: &Pool, uuid: &Uuid, name: &str) -> Result<Self> {
        query_as(
            r#"
            UPDATE plots
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
    ) -> Result<Role> {
        // TODO(danj): update once sqlx allows enums to derive FromRow
        let row: (Role,) = query_as(
            r#"
            SELECT pur.role as "role: Role"
            FROM plots p
            JOIN dataviews dv
            ON p.dataview_uuid = dv.uuid
            AND p.uuid = $1
            JOIN analyses a
            ON dv.analysis_uuid = a.uuid
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
        query("DELETE FROM plots WHERE uuid = $1")
            .bind(uuid)
            .execute(pool)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Plot {
    pub async fn created_at(&self) -> FieldResult<DateTime<Utc>> {
        Ok(self.created_at)
    }

    pub async fn updated_at(&self) -> FieldResult<DateTime<Utc>> {
        Ok(self.updated_at)
    }

    pub async fn dataview(&self, ctx: &Context<'_>) -> FieldResult<Dataview> {
        let d = get_data(ctx)?;
        Dataview::get(&d.pool, &self.dataview_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn name(&self) -> FieldResult<&str> {
        Ok(&self.name)
    }

    #[field(name = "type")]
    pub async fn type_(&self) -> FieldResult<&Type> {
        Ok(&self.type_)
    }

    pub async fn args(&self) -> FieldResult<&str> {
        Ok(&self.args.as_str().unwrap())
    }

    pub async fn status(&self) -> FieldResult<&Status> {
        Ok(&self.status)
    }

    pub async fn uri(&self) -> FieldResult<&Option<String>> {
        Ok(&self.uri)
    }
}
