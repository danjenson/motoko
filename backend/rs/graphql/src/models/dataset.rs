use crate::{
    gql::data,
    models::{Project, Role, Status},
    types::{ColumnDataType, Db, Json},
    utils::dataset_table_name,
};
use async_graphql::{Context, Json as GQLJson, Result as GQLResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, query_scalar, FromRow, Result as SQLxResult};
use std::str;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Dataset {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub project_uuid: Uuid,
    pub uuid: Uuid,
    pub name: String,
    pub uri: String,
    pub status: Status,
}

impl Dataset {
    pub async fn create(
        db: &Db,
        project_uuid: &Uuid,
        name: &str,
        uri: &str,
    ) -> SQLxResult<Self> {
        query_as(
            r#"
            INSERT INTO datasets (project_uuid, name, uri)
            VALUES ($1, $2, $3) RETURNING *
            "#,
        )
        .bind(project_uuid)
        .bind(name)
        .bind(uri)
        .fetch_one(&db.meta)
        .await
    }

    pub async fn get(db: &Db, uuid: &Uuid) -> SQLxResult<Self> {
        query_as("SELECT * FROM datasets WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(&db.meta)
            .await
    }

    pub async fn rename(db: &Db, uuid: &Uuid, name: &str) -> SQLxResult<Self> {
        query_as(
            r#"
            UPDATE datasets
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
        let row: (Role,) = query_as(
            r#"
            SELECT pur.role as "role: Role"
            FROM datasets d
            JOIN project_user_roles pur
            ON d.project_uuid = pur.project_uuid
            AND d.uuid = $1
            AND pur.user_uuid = $2
            "#,
        )
        .bind(&uuid)
        .bind(&user_uuid)
        .fetch_one(&db.meta)
        .await?;
        Ok(row.0)
    }

    pub async fn delete(db: &Db, uuid: &Uuid) -> SQLxResult<()> {
        query("DELETE FROM datasets WHERE uuid = $1")
            .bind(uuid)
            .execute(&db.meta)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Dataset {
    pub async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub async fn project(&self, ctx: &Context<'_>) -> GQLResult<Project> {
        let d = data(ctx)?;
        Project::get(&d.db, &self.project_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn name(&self) -> &String {
        &self.name
    }

    pub async fn status(&self) -> &Status {
        &self.status
    }

    pub async fn schema(
        &self,
        ctx: &Context<'_>,
    ) -> GQLResult<Vec<ColumnDataType>> {
        let d = data(ctx)?;
        let table_name = dataset_table_name(&self.uuid);
        query_as(
            r#"
            SELECT column_name, data_type
            FROM information_schema.columns
            WHERE table_name = $1
            "#,
        )
        .bind(&table_name)
        .fetch_all(&d.db.data)
        .await
        .map_err(|e| e.into())
    }

    pub async fn sample_rows(
        &self,
        ctx: &Context<'_>,
    ) -> GQLResult<GQLJson<Json>> {
        let d = data(ctx)?;
        let table_name = dataset_table_name(&self.uuid);
        query_scalar::<_, Json>(&format!(
            r#"
            SELECT JSON_AGG(t)
            FROM (SELECT * FROM {} TABLESAMPLE SYSTEM_ROWS(100)) t
            "#,
            &table_name
        ))
        .fetch_one(&d.db.data)
        .await
        .map(|v| GQLJson(v))
        .map_err(|e| e.into())
    }
}
