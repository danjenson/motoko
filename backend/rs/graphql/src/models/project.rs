use crate::{
    gql::{current_user, data},
    models::{
        analysis::Analysis,
        dataset::Dataset,
        project_user_role::{ProjectUserRole, Role},
    },
    types::Db,
};
use async_graphql::{Context, Result as GQLResult, ID};
use chrono::{DateTime, Utc};
use node_derive::node;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, Result as SQLxResult};
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub uuid: Uuid,
    pub name: String,
    pub is_public: bool,
}

impl Project {
    pub async fn create(
        db: &Db,
        name: &str,
        user_uuid: &Uuid,
    ) -> SQLxResult<Self> {
        let mut tx = db.meta.begin().await?;
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

    pub async fn get(db: &Db, uuid: &Uuid) -> SQLxResult<Self> {
        query_as("SELECT * FROM projects WHERE uuid = $1")
            .bind(uuid)
            .fetch_one(&db.meta)
            .await
    }

    pub async fn rename(db: &Db, uuid: &Uuid, name: &str) -> SQLxResult<Self> {
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
        .fetch_one(&db.meta)
        .await
    }

    pub async fn make_public(db: &Db, uuid: &Uuid) -> SQLxResult<Self> {
        query_as(
            r#"
            UPDATE projects
            SET is_public = true
            WHERE uuid = $1
            RETURNING *
            "#,
        )
        .bind(uuid)
        .fetch_one(&db.meta)
        .await
    }

    pub async fn delete(db: &Db, uuid: &Uuid) -> SQLxResult<()> {
        query("DELETE FROM projects WHERE uuid = $1")
            .bind(uuid)
            .execute(&db.meta)
            .await
            .map(|_| ())
    }
}

#[node(uuid)]
#[async_graphql::Object]
impl Project {
    pub async fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub async fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub async fn name(&self) -> &String {
        &self.name
    }

    pub async fn is_public(&self) -> &bool {
        &self.is_public
    }

    pub async fn datasets(&self, ctx: &Context<'_>) -> GQLResult<Vec<Dataset>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        query_as(
            r#"
            SELECT d.*
            FROM datasets d
            JOIN projects p
            ON p.uuid = d.project_uuid
            JOIN project_user_roles pur
            ON p.uuid = pur.project_uuid
            WHERE pur.user_uuid = $1
            AND p.uuid = $2
            "#,
        )
        .bind(&user.uuid)
        .bind(&self.uuid)
        .fetch_all(&d.db.meta)
        .await
        .map_err(|e| e.into())
    }

    pub async fn analyses(
        &self,
        ctx: &Context<'_>,
    ) -> GQLResult<Vec<Analysis>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        query_as(
            r#"
            SELECT a.*
            FROM analyses a
            JOIN datasets ds
            ON a.dataset_uuid = ds.uuid
            JOIN project_user_roles pur
            ON ds.project_uuid = pur.project_uuid
            WHERE pur.user_uuid = $1
            AND ds.project_uuid = $2
            "#,
        )
        .bind(&user.uuid)
        .bind(&self.uuid)
        .fetch_all(&d.db.meta)
        .await
        .map_err(|e| e.into())
    }

    pub async fn roles(
        &self,
        ctx: &Context<'_>,
    ) -> GQLResult<Vec<ProjectUserRole>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        query_as(
            r#"
            SELECT pur.*
            FROM project_user_roles pur
            JOIN projects p
            ON p.uuid = pur.project_uuid
            WHERE pur.user_uuid = $1
            AND p.uuid = $2
            "#,
        )
        .bind(&user.uuid)
        .bind(&self.uuid)
        .fetch_all(&d.db.meta)
        .await
        .map_err(|e| e.into())
    }
}
