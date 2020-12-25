use crate::{
    gql::data,
    models::{Dataview, Role, Status},
    types::Db,
};
use async_graphql::{Context, Enum, Json as GQLJson, Result as GQLResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use serde_json::Value as Json;
use sqlx::{self, query, query_as, FromRow, Result as SQLxResult, Type};
use uuid::Uuid;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, Type,
)]
#[graphql(name = "StatisticName")]
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
        db: &Db,
        dataview_uuid: &Uuid,
        name: &Name,
        args: &Json,
    ) -> SQLxResult<Self> {
        query_as(
            r#"
            INSERT INTO statistics (dataview_uuid, name, args)
            VALUES ($1, $2, $3) RETURNING *
            "#,
        )
        .bind(dataview_uuid)
        .bind(name)
        .bind(args)
        .fetch_one(&db.meta)
        .await
    }

    pub async fn get(db: &Db, uuid: &Uuid) -> SQLxResult<Self> {
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
        .fetch_one(&db.meta)
        .await?;
        Ok(row.0)
    }

    pub async fn delete(db: &Db, uuid: &Uuid) -> SQLxResult<()> {
        query("DELETE FROM statistics WHERE uuid = $1")
            .bind(uuid)
            .execute(&db.meta)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Statistic {
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

    pub async fn name(&self) -> &Name {
        &self.name
    }

    pub async fn args(&self) -> GQLJson<Json> {
        GQLJson(self.args.to_owned())
    }

    pub async fn status(&self) -> &Status {
        &self.status
    }

    pub async fn value(&self) -> &Option<f64> {
        &self.value
    }
}
