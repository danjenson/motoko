use crate::{
    db::Pool,
    models::{
        analysis::Analysis,
        dataset::Dataset,
        project_user_role::{ProjectUserRole, Role},
    },
    utils::get_data,
};
use async_graphql::{Context, FieldResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, Result};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uuid: Uuid,
    pub name: String,
}

impl Project {
    pub async fn create(
        pool: &Pool,
        name: &str,
        user_uuid: &Uuid,
    ) -> Result<Self> {
        let mut tx = pool.begin().await?;
        let project: Self =
            query_as("INSERT INTO projects (name) VALUES ($1) RETURNING *")
                .bind(name)
                .fetch_one(&mut tx)
                .await?;
        query(
            r#"
            INSERT INTO project_user_roles (project_uuid, user_uuid, role)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(&project.uuid)
        .bind(user_uuid)
        .bind(&Role::Admin)
        .execute(&mut tx)
        .await?;
        tx.commit().await?;
        Ok(project)
    }

    pub async fn get(pool: &Pool, uuid: &Uuid) -> Result<Self> {
        query_as("SELECT * FROM projects WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(pool)
            .await
    }

    pub async fn rename(pool: &Pool, uuid: &Uuid, name: &str) -> Result<Self> {
        query_as(
            r#"
            UPDATE projects
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

    pub async fn make_public(pool: &Pool, uuid: &Uuid) -> Result<Self> {
        query_as(
            r#"
            UPDATE projects
            SET is_public = true
            WHERE uuid = $1
            RETURNING *
            "#,
        )
        .bind(uuid)
        .fetch_one(pool)
        .await
    }

    pub async fn delete(pool: &Pool, uuid: &Uuid) -> Result<()> {
        query("DELETE FROM projects WHERE uuid = $1")
            .bind(uuid)
            .execute(pool)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Project {
    pub async fn created_at(&self) -> FieldResult<DateTime<Utc>> {
        Ok(self.created_at)
    }

    pub async fn updated_at(&self) -> FieldResult<DateTime<Utc>> {
        Ok(self.updated_at)
    }

    pub async fn name(&self) -> FieldResult<&str> {
        Ok(&self.name)
    }

    pub async fn datasets(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<Dataset>> {
        let d = get_data(ctx)?;
        query_as("SELECT * FROM datasets WHERE project_uuid = $1")
            .bind(self.uuid)
            .fetch_all(&d.pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn analyses(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<Analysis>> {
        let d = get_data(ctx)?;
        query_as("SELECT * FROM analyses WHERE project_uuid = $1")
            .bind(self.uuid)
            .fetch_all(&d.pool)
            .await
            .map_err(|e| e.into())
    }

    pub async fn roles(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<ProjectUserRole>> {
        let d = get_data(ctx)?;
        query_as("SELECT * FROM project_user_roles WHERE project_uuid = $1")
            .bind(self.uuid)
            .fetch_all(&d.pool)
            .await
            .map_err(|e| e.into())
    }
}
