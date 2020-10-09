use crate::{
    db::Pool,
    models::{dataview::Dataview, project_user_role::Role, status::Status},
    utils::get_data,
};
use async_graphql::{Context, FieldResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use sqlx::{query, query_as, FromRow, Result};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Model {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dataview_uuid: Uuid,
    pub uuid: Uuid,
    pub name: String,
    pub target: Option<String>,
    pub features: Vec<String>,
    pub args: Json,
    pub status: Status,
    pub transformer_uri: Option<String>,
    pub estimator_uri: Option<String>,
    pub evaluation: Option<Json>,
    pub decisions: Option<Json>,
}

impl Model {
    pub async fn create(
        pool: &Pool,
        dataview_uuid: &Uuid,
        name: &str,
        target: &Option<String>,
        features: &Vec<String>,
        args: &Json,
    ) -> Result<Self> {
        query_as(
            r#"
            INSERT INTO models (dataview_uuid, name, target, features, args)
            VALUES ($1, $2, $3, $4, $5) RETURNING *
            "#,
        )
        .bind(dataview_uuid)
        .bind(name)
        .bind(target.to_owned())
        .bind(features)
        .bind(args)
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &Pool, uuid: &Uuid) -> Result<Self> {
        query_as("SELECT * FROM models WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(pool)
            .await
    }

    pub async fn rename(pool: &Pool, uuid: &Uuid, name: &str) -> Result<Self> {
        query_as(
            r#"
            UPDATE models
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
            FROM models m
            JOIN dataviews dv
            ON m.dataview_uuid = dv.uuid
            AND m.uuid = $1
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
        query("DELETE FROM models WHERE uuid = $1")
            .bind(uuid)
            .execute(pool)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Model {
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

    pub async fn target(&self) -> FieldResult<&Option<String>> {
        Ok(&self.target)
    }

    pub async fn features(&self) -> FieldResult<&Vec<String>> {
        Ok(&self.features)
    }

    pub async fn args(&self) -> FieldResult<&str> {
        Ok(&self.args.as_str().unwrap())
    }

    pub async fn status(&self) -> FieldResult<&Status> {
        Ok(&self.status)
    }

    pub async fn transformer_uri(&self) -> FieldResult<&Option<String>> {
        Ok(&self.transformer_uri)
    }

    pub async fn estimator_uri(&self) -> FieldResult<&Option<String>> {
        Ok(&self.estimator_uri)
    }

    pub async fn evaluation(&self) -> FieldResult<Option<String>> {
        Ok(self.evaluation.clone().map(|v| v.to_string()))
    }

    pub async fn decisions(&self) -> FieldResult<Option<String>> {
        Ok(self.decisions.clone().map(|v| v.to_string()))
    }
}
