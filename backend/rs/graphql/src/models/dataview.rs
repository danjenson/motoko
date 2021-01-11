use crate::{
    gql::data,
    models::{Analysis, Role, Status},
    types::{ColumnDataType, Db},
    utils::dataview_view_name,
};
use async_graphql::{Context, Enum, Json as GQLJson, Result as GQLResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use sqlx::{
    query, query_as, query_scalar, FromRow, Result as SQLxResult, Type,
};
use uuid::Uuid;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, Type,
)]
#[sqlx(rename = "DATAVIEW_OPERATION")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "UPPERCASE")]
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
        db: &Db,
        dataview_uuid: &Uuid,
        operation: &Operation,
        args: &Json,
    ) -> SQLxResult<Self> {
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
        .fetch_one(&db.meta)
        .await
    }

    pub async fn get(db: &Db, uuid: &Uuid) -> SQLxResult<Self> {
        query_as("SELECT * FROM dataviews WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(&db.meta)
            .await
    }

    pub async fn role(
        db: &Db,
        uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> SQLxResult<Role> {
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
        .fetch_one(&db.meta)
        .await?;
        Ok(row.0)
    }

    pub async fn delete(db: &Db, uuid: &Uuid) -> SQLxResult<()> {
        query("DELETE FROM dataviews WHERE uuid = $1")
            .bind(uuid)
            .execute(&db.meta)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Dataview {
    pub async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub async fn analysis(&self, ctx: &Context<'_>) -> GQLResult<Analysis> {
        let d = data(ctx)?;
        Analysis::get(&d.db, &self.analysis_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn parent(&self, ctx: &Context<'_>) -> GQLResult<Self> {
        let d = data(ctx)?;
        Self::get(&d.db, &self.parent_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn operation(&self) -> &Operation {
        &self.operation
    }

    pub async fn args(&self) -> Option<GQLJson<Json>> {
        self.args.to_owned().map(|v| GQLJson(v))
    }

    pub async fn status(&self) -> &Status {
        &self.status
    }

    pub async fn schema(
        &self,
        ctx: &Context<'_>,
    ) -> Option<Vec<ColumnDataType>> {
        let d = data(ctx).ok()?;
        let view = dataview_view_name(&self.uuid);
        query_as(
            r#"
            SELECT column_name, data_type
            FROM information_schema.columns
            WHERE table_name = $1
            "#,
        )
        .bind(&view)
        .fetch_all(&d.db.data)
        .await
        .ok()
    }

    pub async fn n_rows(&self, ctx: &Context<'_>) -> Option<i64> {
        let d = data(ctx).ok()?;
        let view = dataview_view_name(&self.uuid);
        query_scalar::<_, i64>(&format!("SELECT COUNT(*) FROM {}", &view))
            .fetch_one(&d.db.data)
            .await
            .ok()
    }

    pub async fn sample_rows(
        &self,
        ctx: &Context<'_>,
    ) -> Option<GQLJson<Json>> {
        let d = data(ctx).ok()?;
        let view = dataview_view_name(&self.uuid);
        query_scalar::<_, Json>(&format!(
            r#"
            SELECT JSON_AGG(t)
            FROM (SELECT * FROM {} LIMIT 25) t
            "#,
            &view
        ))
        .fetch_one(&d.db.data)
        .await
        .map(|v| GQLJson(v))
        .ok()
    }
}
