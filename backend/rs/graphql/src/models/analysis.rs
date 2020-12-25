use crate::{
    gql::data,
    models::{Dataset, Dataview, Role},
    types::Db,
    utils::{dataset_table_name, dataview_view_name},
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
        db: &Db,
        dataset_uuid: &Uuid,
        name: &str,
    ) -> SQLxResult<Self> {
        let analysis: Self = query_as(
            r#"
            WITH dv AS (
                INSERT INTO dataviews (analysis_uuid, uuid, parent_uuid, status)
                VALUES ($1, $3, $3, 'completed')
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
        .fetch_one(&db.meta)
        .await?;
        query(&format!(
            "CREATE VIEW {} AS SELECT * FROM {}",
            dataview_view_name(&analysis.dataview_uuid),
            dataset_table_name(dataset_uuid)
        ))
        .execute(&db.data)
        .await?;
        Ok(analysis)
    }

    pub async fn get(db: &Db, uuid: &Uuid) -> SQLxResult<Self> {
        query_as("SELECT * FROM analyses WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(&db.meta)
            .await
    }

    pub async fn rename(db: &Db, uuid: &Uuid, name: &str) -> SQLxResult<Self> {
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
        .fetch_one(&db.meta)
        .await?;
        Ok(row.0)
    }

    pub async fn point_to(
        db: &Db,
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
        .fetch_one(&db.meta)
        .await
    }

    pub async fn delete(db: &Db, uuid: &Uuid) -> SQLxResult<()> {
        query("DELETE FROM analyses WHERE uuid = $1")
            .bind(uuid)
            .execute(&db.meta)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Analysis {
    pub async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub async fn dataset(&self, ctx: &Context<'_>) -> GQLResult<Dataset> {
        let d = data(ctx)?;
        Dataset::get(&d.db, &self.dataset_uuid)
            .await
            .map_err(|e| e.into())
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
}
