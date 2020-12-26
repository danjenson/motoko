use crate::{
    gql::{current_user, data, graphql_id_to_uuid},
    id_to_node,
    models::{
        Analysis, Dataset, Dataview, Model, Plot, Project, Statistic, User,
    },
    Node,
};
use async_graphql::{Context, Result, ID};
use sqlx::query_as;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn me<'ctx>(&self, ctx: &'ctx Context<'_>) -> Result<&'ctx User> {
        current_user(ctx)
    }

    async fn node(&self, ctx: &Context<'_>, id: ID) -> Result<Node> {
        let d = data(ctx)?;
        id_to_node(&d.db, &id).await
    }

    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<Project>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        query_as(
            r#"
            SELECT p.*
            FROM projects p
            JOIN project_user_roles pur
            ON p.uuid = pur.project_uuid
            AND pur.user_uuid = $1
            "#,
        )
        .bind(&user.uuid)
        .fetch_all(&d.db.meta)
        .await
        .map_err(|e| e.into())
    }

    async fn datasets(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
    ) -> Result<Vec<Dataset>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let project_uuid = graphql_id_to_uuid(&project_id)?;
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
        .bind(&project_uuid)
        .fetch_all(&d.db.meta)
        .await
        .map_err(|e| e.into())
    }

    async fn analyses(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
    ) -> Result<Vec<Analysis>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let project_uuid = graphql_id_to_uuid(&project_id)?;
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
        .bind(&project_uuid)
        .fetch_all(&d.db.meta)
        .await
        .map_err(|e| e.into())
    }

    async fn roles(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
    ) -> Result<Vec<Dataset>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let project_uuid = graphql_id_to_uuid(&project_id)?;
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
        .bind(&project_uuid)
        .fetch_all(&d.db.meta)
        .await
        .map_err(|e| e.into())
    }

    async fn dataviews(
        &self,
        ctx: &Context<'_>,
        analysis_id: ID,
    ) -> Result<Vec<Dataview>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let analysis_uuid = graphql_id_to_uuid(&analysis_id)?;
        query_as(
            r#"
            SELECT x.*
            FROM (
                -- children
                WITH RECURSIVE sub_dataviews AS (
                    SELECT dv1.*
                    FROM dataviews dv1
                    JOIN analyses a
                    ON dv1.uuid = a.dataview_uuid
                    JOIN datasets ds
                    ON a.dataset_uuid = ds.uuid
                    JOIN project_user_roles pur
                    ON ds.project_uuid = pur.project_uuid
                    AND pur.user_uuid = $1
                    AND a.uuid = $2
                    AND dv1.uuid != dv1.parent_uuid
                    UNION ALL
                    SELECT dv2.*
                    FROM dataviews dv2
                    JOIN sub_dataviews sdv
                    ON sdv.uuid = dv2.parent_uuid
                )
                SELECT *
                FROM sub_dataviews

                UNION ALL

                -- roots
                SELECT dv1.*
                FROM dataviews dv1
                JOIN analyses a
                ON dv1.uuid = a.dataview_uuid
                JOIN datasets ds
                ON a.dataset_uuid = ds.uuid
                JOIN project_user_roles pur
                ON ds.project_uuid = pur.project_uuid
                AND pur.user_uuid = $1
                AND a.uuid = $2
                AND dv1.uuid = dv1.parent_uuid
            ) x
            ORDER BY x.created_at
            "#,
        )
        .bind(&user.uuid)
        .bind(&analysis_uuid)
        .fetch_all(&d.db.meta)
        .await
        .map_err(|e| e.into())
    }

    async fn plots(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
    ) -> Result<Vec<Plot>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let dataview_uuid = graphql_id_to_uuid(&dataview_id)?;
        query_as(
            r#"
            SELECT x.*
            FROM plots x
            JOIN dataviews dv
            ON x.dataview_uuid = dv.uuid
            AND dv.uuid = $1
            JOIN analyses a
            ON dv.analysis_uuid = a.uuid
            JOIN datasets ds
            ON a.dataset_uuid = ds.uuid
            JOIN project_user_roles pur
            ON ds.project_uuid = pur.project_uuid
            AND pur.user_uuid = $2
            "#,
        )
        .bind(&dataview_uuid)
        .bind(&user.uuid)
        .fetch_all(&d.db.meta)
        .await
        .map_err(|e| e.into())
    }

    async fn statistics(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
    ) -> Result<Vec<Statistic>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let dataview_uuid = graphql_id_to_uuid(&dataview_id)?;
        query_as(
            r#"
            SELECT x.*
            FROM statistics x
            JOIN dataviews dv
            ON x.dataview_uuid = dv.uuid
            AND dv.uuid = $1
            JOIN analyses a
            ON dv.analysis_uuid = a.uuid
            JOIN datasets ds
            ON a.dataset_uuid = ds.uuid
            JOIN project_user_roles pur
            ON ds.project_uuid = pur.project_uuid
            AND pur.user_uuid = $2
            "#,
        )
        .bind(&dataview_uuid)
        .bind(&user.uuid)
        .fetch_all(&d.db.meta)
        .await
        .map_err(|e| e.into())
    }

    async fn models(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
    ) -> Result<Vec<Model>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let dataview_uuid = graphql_id_to_uuid(&dataview_id)?;
        query_as(
            r#"
            SELECT x.*
            FROM models x
            JOIN dataviews dv
            ON x.dataview_uuid = dv.uuid
            AND dv.uuid = $1
            JOIN analyses a
            ON dv.analysis_uuid = a.uuid
            JOIN datasets ds
            ON a.dataset_uuid = ds.uuid
            JOIN project_user_roles pur
            ON ds.project_uuid = pur.project_uuid
            AND pur.user_uuid = $2
            "#,
        )
        .bind(&dataview_uuid)
        .bind(&user.uuid)
        .fetch_all(&d.db.meta)
        .await
        .map_err(|e| e.into())
    }
}
