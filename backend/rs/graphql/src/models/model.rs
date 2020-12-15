use crate::{
    gql::data,
    models::{Dataview, Role, Status},
    types::Db,
};
use async_graphql::{Context, Json as GQLJson, Result as GQLResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use sqlx::{query, query_as, FromRow, Result as SQLxResult};
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
        db: &Db,
        dataview_uuid: &Uuid,
        name: &str,
        target: &Option<String>,
        features: &Vec<String>,
        args: &Json,
    ) -> SQLxResult<Self> {
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
        .fetch_one(db)
        .await
    }

    pub async fn get(db: &Db, uuid: &Uuid) -> SQLxResult<Self> {
        query_as("SELECT * FROM models WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(db)
            .await
    }

    pub async fn rename(db: &Db, uuid: &Uuid, name: &str) -> SQLxResult<Self> {
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
        .fetch_one(db)
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
        .fetch_one(db)
        .await?;
        Ok(row.0)
    }

    pub async fn delete(db: &Db, uuid: &Uuid) -> SQLxResult<()> {
        query("DELETE FROM models WHERE uuid = $1")
            .bind(uuid)
            .execute(db)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Model {
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

    pub async fn target(&self) -> &Option<String> {
        &self.target
    }

    pub async fn features(&self) -> &Vec<String> {
        &self.features
    }

    pub async fn args(&self) -> GQLJson<Json> {
        GQLJson(self.args.to_owned())
    }

    pub async fn status(&self) -> &Status {
        &self.status
    }

    pub async fn transformer_uri(&self) -> &Option<String> {
        &self.transformer_uri
    }

    pub async fn estimator_uri(&self) -> &Option<String> {
        &self.estimator_uri
    }

    pub async fn evaluation(&self) -> Option<GQLJson<Json>> {
        self.evaluation.to_owned().map(|v| GQLJson(v))
    }

    pub async fn decisions(&self) -> Option<GQLJson<Json>> {
        self.decisions.to_owned().map(|v| GQLJson(v))
    }
}
