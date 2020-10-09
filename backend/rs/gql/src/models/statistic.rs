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
use sqlx::{self, query, query_as, FromRow, Result, Type};
use uuid::Uuid;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, Type,
)]
#[sqlx(rename = "STATISTIC_NAME")]
#[sqlx(rename_all = "snake_case")]
pub enum Name {
    Correlation,
    Mean,
    Median,
    Mode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Statistic {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dataview_uuid: Uuid,
    pub uuid: Uuid,
    pub name: Name,
    pub args: Json,
    pub status: Status,
    pub value: Option<f64>,
}

impl Statistic {
    pub async fn create(
        pool: &Pool,
        dataview_uuid: &Uuid,
        name: &Name,
        args: &Json,
    ) -> Result<Self> {
        query_as(
            r#"
            INSERT INTO statistics (dataview_uuid, name, args)
            VALUES ($1, $2, $3) RETURNING *
            "#,
        )
        .bind(dataview_uuid)
        .bind(name)
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
                name as "name: Name",
                args,
                status as "status: Status",
                value
            FROM statistics
            WHERE uuid = $1
            "#,
        )
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
            FROM statistics s
            JOIN dataviews dv
            ON s.dataview_uuid = dv.uuid
            AND s.uuid = $1
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
        query("DELETE FROM statistics WHERE uuid = $1")
            .bind(uuid)
            .execute(pool)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Statistic {
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

    pub async fn name(&self) -> FieldResult<&Name> {
        Ok(&self.name)
    }

    pub async fn args(&self) -> FieldResult<&str> {
        Ok(&self.args.as_str().unwrap())
    }

    pub async fn status(&self) -> FieldResult<&Status> {
        Ok(&self.status)
    }

    pub async fn value(&self) -> FieldResult<&Option<f64>> {
        Ok(&self.value)
    }
}