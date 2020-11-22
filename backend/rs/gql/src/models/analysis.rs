use crate::{
    db::Pool,
    models::{dataset::Dataset, dataview::Dataview, project_user_role::Role},
    utils::get_data,
};
use async_graphql::{Context, Result as GQLResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, Result as SQLxResult};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Analysis {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub dataset_uuid: Uuid,
    pub dataview_uuid: Uuid,
    pub uuid: Uuid,
    pub name: String,
}

impl Analysis {
    pub async fn create(
        pool: &Pool,
        dataset_uuid: &Uuid,
        name: &str,
    ) -> SQLxResult<Self> {
        query_as(
            r#"
            WITH dv AS (
                INSERT INTO dataviews (analysis_uuid, uuid, parent_uuid)
                VALUES ($1, $3, $3)
                RETURNING *
            )
            INSERT INTO analyses (uuid, dataset_uuid, dataview_uuid, name)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(dataset_uuid)
        .bind(Uuid::new_v4())
        .bind(name)
        .fetch_one(pool)
        .await
    }

    pub async fn get(pool: &Pool, uuid: &Uuid) -> SQLxResult<Self> {
        query_as("SELECT * FROM analyses WHERE uuid = $1")
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
            UPDATE analyses
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
        // TODO(danj): update once sqlx allows enums to derive FromRow
        let row: (Role,) = query_as(
            r#"
            SELECT pur.role as "role: Role"
            FROM analyses a
            JOIN datasets d
            ON a.dataset_uuid = d.uuid
            AND a.uuid = $1
            JOIN project_user_roles pur
            ON d.project_uuid = pur.project_uuid
            AND pur.user_uuid = $2
            "#,
        )
        .bind(&uuid)
        .bind(&user_uuid)
        .fetch_one(pool)
        .await?;
        Ok(row.0)
    }

    pub async fn point_to(
        pool: &Pool,
        uuid: &Uuid,
        dataview_uuid: &Uuid,
    ) -> SQLxResult<Self> {
        query_as(
            r#"
            UPDATE analyses
            SET dataview_uuid = $2
            WHERE uuid = $1
            RETURNING *
            "#,
        )
        .bind(uuid)
        .bind(dataview_uuid)
        .fetch_one(pool)
        .await
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
impl Analysis {
    pub async fn created_at(&self) -> GQLResult<DateTime<Utc>> {
        Ok(self.created_at)
    }

    pub async fn updated_at(&self) -> GQLResult<DateTime<Utc>> {
        Ok(self.updated_at)
    }

    pub async fn dataset(&self, ctx: &Context<'_>) -> GQLResult<Dataset> {
        let d = get_data(ctx)?;
        Dataset::get(&d.pool, &self.dataset_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn dataview(&self, ctx: &Context<'_>) -> GQLResult<Dataview> {
        let d = get_data(ctx)?;
        Dataview::get(&d.pool, &self.dataview_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn name(&self) -> GQLResult<&String> {
        Ok(&self.name)
    }
}
