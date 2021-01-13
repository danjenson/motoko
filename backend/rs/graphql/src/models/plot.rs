use crate::{
    gql::data,
    models::{Dataview, Role, Status},
    utils::get_presigned_url,
    Db,
};
use async_graphql::{Context, Enum, Json as GQLJson, Result as GQLResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use sqlx::{query, query_as, FromRow, Result as SQLxResult};
use uuid::Uuid;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, sqlx::Type,
)]
#[graphql(name = "PlotType")]
#[sqlx(rename = "PLOT_TYPE")]
#[sqlx(rename_all = "snake_case")]
#[serde(rename_all = "UPPERCASE")]
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
    pub error: Option<Json>,
}

impl Plot {
    pub async fn create(
        db: &Db,
        dataview_uuid: &Uuid,
        name: &str,
        type_: &Type,
        args: &Json,
    ) -> SQLxResult<Self> {
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
        .fetch_one(&db.meta)
        .await
    }

    pub async fn get(db: &Db, uuid: &Uuid) -> SQLxResult<Self> {
        query_as("SELECT * FROM plots WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(&db.meta)
            .await
    }

    pub async fn rename(db: &Db, uuid: &Uuid, name: &str) -> SQLxResult<Self> {
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
        .fetch_one(&db.meta)
        .await?;
        Ok(row.0)
    }

    pub async fn delete(db: &Db, uuid: &Uuid) -> SQLxResult<()> {
        query("DELETE FROM plots WHERE uuid = $1")
            .bind(uuid)
            .execute(&db.meta)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Plot {
    pub async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub async fn dataview(&self, ctx: &Context<'_>) -> GQLResult<Dataview> {
        let d = data(ctx)?;
        Dataview::get(&d.db, &self.dataview_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn name(&self) -> &String {
        &self.name
    }

    #[graphql(name = "type")]
    pub async fn type_(&self) -> &Type {
        &self.type_
    }

    pub async fn args(&self) -> GQLJson<Json> {
        GQLJson(self.args.to_owned())
    }

    pub async fn status(&self) -> &Status {
        &self.status
    }

    pub async fn uri(&self, ctx: &Context<'_>) -> Option<String> {
        if self.status != Status::Completed {
            return None;
        }
        let d = data(ctx).ok()?;
        let key = format!("plots/{}.svg", &self.uuid.to_string());
        Some(get_presigned_url(
            &d.storage.region,
            &d.storage.bucket,
            &key,
            &d.auth.aws_credentials,
        ))
    }

    pub async fn error(&self) -> Option<GQLJson<Json>> {
        self.error.to_owned().map(|v| GQLJson(v))
    }
}
