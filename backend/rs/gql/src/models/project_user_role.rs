use crate::{
    db::Pool,
    models::{project::Project, user::User},
    utils::get_data,
};
use async_graphql::{Context, Enum, FieldResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, Result, Type};
use uuid::Uuid;

#[derive(
    Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Enum, Type,
)]
#[sqlx(rename = "PROJECT_USER_ROLE")]
#[sqlx(rename_all = "snake_case")]
pub enum Role {
    Viewer,
    Editor,
    Admin,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct ProjectUserRole {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub project_uuid: Uuid,
    pub user_uuid: Uuid,
    pub role: Role,
}

impl ProjectUserRole {
    pub async fn create(
        pool: &Pool,
        project_uuid: &Uuid,
        user_uuid: &Uuid,
        role: &Role,
    ) -> Result<Self> {
        query_as(
            r#"
            INSERT INTO project_user_roles (project_uuid, user_uuid, role)
            VALUES ($1, $2, $3) RETURNING *
            "#,
        )
        .bind(project_uuid)
        .bind(user_uuid)
        .bind(role)
        .fetch_one(pool)
        .await
    }

    pub async fn get(
        pool: &Pool,
        project_uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> Result<Self> {
        query_as(
            r#"
            SELECT *
            FROM project_user_roles
            WHERE project_uuid = $1
            AND user_uuid = $2
            "#,
        )
        .bind(project_uuid)
        .bind(user_uuid)
        .fetch_one(pool)
        .await
    }

    pub async fn modify(
        pool: &Pool,
        project_uuid: &Uuid,
        user_uuid: &Uuid,
        role: &Role,
    ) -> Result<Self> {
        query_as(
            r#"
            UPDATE project_user_roles
            SET role = $3
            WHERE project_uuid = $1
            AND user_uuid = $2
            RETURNING *
            "#,
        )
        .bind(project_uuid)
        .bind(user_uuid)
        .bind(role)
        .fetch_one(pool)
        .await
    }

    pub async fn by_project(
        pool: &Pool,
        project_uuid: &Uuid,
    ) -> Result<Vec<Self>> {
        query_as(
            r#"
            SELECT *
            FROM project_user_roles
            WHERE project_uuid = $1
            "#,
        )
        .bind(project_uuid)
        .fetch_all(pool)
        .await
    }

    pub async fn by_user(pool: &Pool, user_uuid: &Uuid) -> Result<Vec<Self>> {
        query_as(
            r#"
            SELECT *
            FROM project_user_roles
            WHERE user_uuid = $1
            "#,
        )
        .bind(user_uuid)
        .fetch_all(pool)
        .await
    }

    pub async fn delete(
        pool: &Pool,
        project_uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> Result<()> {
        query(
            r#"
            DELETE FROM project_user_roles
            WHERE project_uuid = $1
            AND user_uuid = $2
            "#,
        )
        .bind(project_uuid)
        .bind(user_uuid)
        .execute(pool)
        .await
        .map(|_| ())
    }
}

#[node(project_uuid, user_uuid)]
#[async_graphql::Object]
impl ProjectUserRole {
    pub async fn created_at(&self) -> FieldResult<DateTime<Utc>> {
        Ok(self.created_at)
    }

    pub async fn updated_at(&self) -> FieldResult<DateTime<Utc>> {
        Ok(self.updated_at)
    }

    pub async fn project(&self, ctx: &Context<'_>) -> FieldResult<Project> {
        let d = get_data(ctx)?;
        Project::get(&d.pool, &self.project_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn user(&self, ctx: &Context<'_>) -> FieldResult<User> {
        let d = get_data(ctx)?;
        User::get(&d.pool, &self.user_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn role(&self) -> FieldResult<&Role> {
        Ok(&self.role)
    }
}
