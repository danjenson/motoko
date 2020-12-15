use crate::{
    gql::{current_user, data, graphql_id_to_uuid},
    id_to_node,
    models::{Analysis, Dataset, Dataview, Model, Plot, Statistic, User},
    Node,
};
use async_graphql::{Context, Result, ID};
use sqlx::query_as;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn me<'ctx>(
        &'ctx self,
        ctx: &'ctx Context<'_>,
    ) -> Result<&'ctx User> {
        current_user(ctx)
    }

    async fn node(&self, ctx: &Context<'_>, id: ID) -> Result<Node> {
        let d = data(ctx)?;
        id_to_node(&d.db, &id).await
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
        .fetch_all(&d.db)
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
            JOIN projects p
            ON p.uuid = a.project_uuid
            JOIN project_user_roles pur
            ON p.uuid = pur.project_uuid
            WHERE pur.user_uuid = $1
            AND p.uuid = $2
            "#,
        )
        .bind(&user.uuid)
        .bind(&project_uuid)
        .fetch_all(&d.db)
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
        .fetch_all(&d.db)
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
            WITH RECURSIVE sub_dataviews AS (
                SELECT dv1.*
                FROM dataviews dv1
                JOIN analyses a
                ON dv1.uuid = a.dataview_uuid
                JOIN project_user_roles pur
                ON a.project_uuid = pur.project_uuid
                WHERE pur.user_uuid = $1
                AND a.uuid = $2
                UNION
                SELECT dv2.*
                FROM dataviews dv2
                JOIN sub_dataviews sdv
                ON sdv.uuid = dv2.parent_uuid
            )
            SELECT *
            FROM sub_dataviews
            "#,
        )
        .bind(&user.uuid)
        .bind(&analysis_uuid)
        .fetch_all(&d.db)
        .await
        .map_err(|e| e.into())
    }

    async fn plots(
        &self,
        ctx: &Context<'_>,
        analysis_id: ID,
    ) -> Result<Vec<Plot>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let analysis_uuid = graphql_id_to_uuid(&analysis_id)?;
        query_as(
            r#"
            SELECT pl.*
            FROM plots pl
            JOIN analyses a
            ON pl.analysis_uuid = a.uuid
            JOIN project_user_roles pur
            ON a.project_uuid = pur.project_uuid
            WHERE pur.user_uuid = $1
            "#,
        )
        .bind(&user.uuid)
        .bind(&analysis_uuid)
        .fetch_all(&d.db)
        .await
        .map_err(|e| e.into())
    }

    async fn statistics(
        &self,
        ctx: &Context<'_>,
        analysis_id: ID,
    ) -> Result<Vec<Statistic>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let analysis_uuid = graphql_id_to_uuid(&analysis_id)?;
        query_as(
            r#"
            SELECT s.*
            FROM statistics s
            JOIN analyses a
            ON s.analysis_uuid = a.uuid
            JOIN project_user_roles pur
            ON a.project_uuid = pur.project_uuid
            WHERE pur.user_uuid = $1
            "#,
        )
        .bind(&user.uuid)
        .bind(&analysis_uuid)
        .fetch_all(&d.db)
        .await
        .map_err(|e| e.into())
    }

    async fn models(
        &self,
        ctx: &Context<'_>,
        analysis_id: ID,
    ) -> Result<Vec<Model>> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let analysis_uuid = graphql_id_to_uuid(&analysis_id)?;
        query_as(
            r#"
            SELECT m.*
            FROM models m
            JOIN analyses a
            ON m.analysis_uuid = a.uuid
            JOIN project_user_roles pur
            ON a.project_uuid = pur.project_uuid
            WHERE pur.user_uuid = $1
            "#,
        )
        .bind(&user.uuid)
        .bind(&analysis_uuid)
        .fetch_all(&d.db)
        .await
        .map_err(|e| e.into())
    }
}
