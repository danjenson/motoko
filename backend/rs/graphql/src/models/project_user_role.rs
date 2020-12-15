use crate::{
    gql::data,
    models::{Project, User},
    types::Db,
};
use async_graphql::{Context, Enum, Result as GQLResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, Result as SQLxResult, Type};
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
        db: &Db,
        project_uuid: &Uuid,
        user_uuid: &Uuid,
        role: &Role,
    ) -> SQLxResult<Self> {
        query_as(
            r#"
            INSERT INTO project_user_roles (project_uuid, user_uuid, role)
            VALUES ($1, $2, $3) RETURNING *
            "#,
        )
        .bind(project_uuid)
        .bind(user_uuid)
        .bind(role)
        .fetch_one(db)
        .await
    }

    pub async fn get(
        db: &Db,
        project_uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> SQLxResult<Self> {
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
        .fetch_one(db)
        .await
    }

    pub async fn modify(
        db: &Db,
        project_uuid: &Uuid,
        user_uuid: &Uuid,
        role: &Role,
    ) -> SQLxResult<Self> {
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
        .fetch_one(db)
        .await
    }

    pub async fn by_project(
        db: &Db,
        project_uuid: &Uuid,
    ) -> SQLxResult<Vec<Self>> {
        query_as(
            r#"
            SELECT *
            FROM project_user_roles
            WHERE project_uuid = $1
            "#,
        )
        .bind(project_uuid)
        .fetch_all(db)
        .await
    }

    pub async fn by_user(db: &Db, user_uuid: &Uuid) -> SQLxResult<Vec<Self>> {
        query_as(
            r#"
            SELECT *
            FROM project_user_roles
            WHERE user_uuid = $1
            "#,
        )
        .bind(user_uuid)
        .fetch_all(db)
        .await
    }

    pub async fn delete(
        db: &Db,
        project_uuid: &Uuid,
        user_uuid: &Uuid,
    ) -> SQLxResult<()> {
        query(
            r#"
            DELETE FROM project_user_roles
            WHERE project_uuid = $1
            AND user_uuid = $2
            "#,
        )
        .bind(project_uuid)
        .bind(user_uuid)
        .execute(db)
        .await
        .map(|_| ())
    }
}

#[node(project_uuid, user_uuid)]
#[async_graphql::Object]
impl ProjectUserRole {
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

    pub async fn user(&self, ctx: &Context<'_>) -> GQLResult<User> {
        let d = data(ctx)?;
        User::get(&d.db, &self.user_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn role(&self) -> &Role {
        &self.role
    }
}
