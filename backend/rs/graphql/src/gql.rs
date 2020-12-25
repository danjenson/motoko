use crate::{
    models::User, utils::run_mode, ContextData, Error, ModelKeys, Mutation,
    Query,
};
use async_graphql::{
    from_value, Context, EmptySubscription, Error as GQLError,
    Request as GQLRequest, Response as GQLResponse, Result as GQLResult,
    Schema, Value as GQLValue, ID,
};
use serde::de::DeserializeOwned;
use std::str;
use uuid::Uuid;

pub fn current_user<'ctx>(ctx: &'ctx Context<'_>) -> GQLResult<&'ctx User> {
    let d = data(ctx)?;
    match &d.user {
        Some(user) => Ok(user),
        None => Err(Error::InvalidPermissions.into()),
    }
}

pub fn data<'ctx>(ctx: &'ctx Context<'_>) -> GQLResult<&'ctx ContextData> {
    ctx.data::<ContextData>()
}

pub fn from_response<T: DeserializeOwned>(res: GQLResponse) -> GQLResult<T> {
    if let GQLValue::Object(x) = res.data {
        let first = x
            .values()
            .cloned()
            .next()
            .ok_or::<GQLError>(Error::Serde.into())?;
        return from_value::<T>(first).map_err(|e| e.into());
    }
    Err(Error::Serde.into())
}

pub fn get_invocation_type() -> Option<String> {
    // TODO(danj): can't invoke async locally [invocation_type Some("Event")]
    // https://github.com/aws/aws-sam-cli/pull/749
    // None defaults to RequestResponse
    if run_mode().as_str() != "local" {
        Some("Event".to_owned())
    } else {
        None
    }
}

pub fn graphql_id_to_uuid(id: &ID) -> GQLResult<Uuid> {
    let mkeys = model_keys(id)?;
    let first_key = mkeys
        .keys
        .first()
        .ok_or::<GQLError>(Error::InvalidGraphQLID.into())?;
    Uuid::parse_str(first_key).map_err(|e| e.into())
}

pub fn is_current_user(user_uuid: &Uuid, ctx: &Context<'_>) -> GQLResult<()> {
    let curr_user = current_user(ctx)?;
    if curr_user.uuid != *user_uuid {
        return Err(Error::InvalidPermissions.into());
    }
    Ok(())
}

pub fn model_keys(id: &ID) -> GQLResult<ModelKeys> {
    let decoded = base64::decode(id.to_string())?;
    let value = str::from_utf8(&decoded)?;
    let model_keys = value
        .to_string()
        .split(":")
        .map(|v| v.to_string())
        .collect::<Vec<String>>();
    let (model, keys) = model_keys
        .split_first()
        .ok_or::<GQLError>(Error::InvalidGraphQLID.into())?;
    Ok(ModelKeys {
        model: model.to_string(),
        keys: keys.to_vec(),
    })
}

pub async fn respond(req: GQLRequest, ctx: &ContextData) -> GQLResponse {
    Schema::new(Query, Mutation, EmptySubscription)
        .execute(req.data(ctx.clone()))
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::User, queries::*, utils::vars_to_json_string, GenericError,
    };
    use async_graphql::Result as GQLResult;
    use rusoto_core::Region;
    use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};
    use sqlx::{query, Result as SQLxResult};
    use std::{env, process::Command, thread, time};
    use tokio_compat_02::FutureExt;

    #[tokio::test]
    async fn graphql_round_trip() -> GQLResult<()> {
        let mut sam_process = sam_local_start_lambda();
        // wait for SAM lambda service to start
        thread::sleep(time::Duration::from_secs(2));
        let res = _graphql_round_trip().compat().await;
        sam_process.kill().expect("unable to kill SAM process");
        res
    }

    fn sam_local_start_lambda() -> std::process::Child {
        env::set_var("RUN_MODE", "local");
        Command::new("sam")
            .args(&["local", "start-lambda", "--env-vars", "test_env.json"])
            .current_dir("../..")
            .spawn()
            .expect("unable to start sam local lambda service")
    }

    async fn _graphql_round_trip() -> GQLResult<()> {
        // setup
        let ctx = test_ctx().await?;

        // create lambda client
        let region = Region::Custom {
            name: "us-west-1".to_owned(),
            endpoint: "http://127.0.0.1:3001".to_owned(),
        };
        let lambda = LambdaClient::new(region);

        // create project
        let mut res =
            respond(create_project(&[("name", "Test Project")]), &ctx).await;
        let mut project: ProjectResponse = from_response(res)?;

        // make project public
        res = respond(
            make_project_public(&[("projectId", &project.id.clone())]),
            &ctx,
        )
        .await;
        project = from_response(res)?;
        if !project.is_public {
            return Err(GQLError::new("failed to make project public"));
        }

        // rename project
        let new_project_name = "Test Project Renamed";
        res = respond(
            rename_project(&[
                ("projectId", &project.id.clone()),
                ("name", new_project_name),
            ]),
            &ctx,
        )
        .await;
        project = from_response(res)?;
        if project.name != new_project_name {
            return Err(GQLError::new("failed to rename project"));
        }

        // create dataset
        res = respond(create_dataset(&[
            ("projectId", &project.id.clone()),
            ("name", "iris"),
            ("uri", "https://drive.google.com/file/d/12q0KWJAUaVba9RZrVY8QEXThK1x5GoF8/view?usp=sharing")
        ]), &ctx).await;
        let mut dataset: DatasetResponse = from_response(res)?;

        // rename dataset
        let new_dataset_name = "iris renamed";
        res = respond(
            rename_dataset(&[
                ("datasetId", &dataset.id.clone()),
                ("name", new_dataset_name),
            ]),
            &ctx,
        )
        .await;
        dataset = from_response(res)?;
        if dataset.name != new_dataset_name {
            return Err(GQLError::new("failed to rename dataset"));
        }

        // create analysis
        res = respond(
            create_analysis(&[
                ("datasetId", &dataset.id.clone()),
                ("name", "test analysis"),
            ]),
            &ctx,
        )
        .await;
        let mut analysis: AnalysisResponse = from_response(res)?;

        // rename analysis
        let new_analysis_name = "test analysis renamed";
        res = respond(
            rename_analysis(&[
                ("analysisId", &analysis.id.clone()),
                ("name", new_analysis_name),
            ]),
            &ctx,
        )
        .await;
        analysis = from_response(res)?;
        if analysis.name != new_analysis_name {
            return Err(GQLError::new("failed to rename analysis"));
        }

        // create bar plot
        res = respond(
            create_plot(&[
                ("dataviewId", &analysis.dataview.id.clone()),
                ("name", "bar"),
                ("type", "BAR"),
                (
                    "args",
                    &vars_to_json_string(&[
                        ("x", "species"),
                        ("color", "species"),
                        ("title", "Species"),
                    ]),
                ),
            ]),
            &ctx,
        )
        .await;
        from_response::<PlotResponse>(res)?;

        // create histogram plot
        res = respond(
            create_plot(&[
                ("dataviewId", &analysis.dataview.id.clone()),
                ("name", "histogram"),
                ("type", "HISTOGRAM"),
                ("args", &vars_to_json_string(&[("x", "sepal_width")])),
            ]),
            &ctx,
        )
        .await;
        from_response::<PlotResponse>(res)?;

        // create line plot
        res = respond(
            create_plot(&[
                ("dataviewId", &analysis.dataview.id.clone()),
                ("name", "histogram"),
                ("type", "LINE"),
                (
                    "args",
                    &vars_to_json_string(&[
                        ("x", "sepal_width"),
                        ("y", "petal_width"),
                        ("title", "Sepal vs. Petal Width"),
                        ("color", "species"),
                    ]),
                ),
            ]),
            &ctx,
        )
        .await;
        from_response::<PlotResponse>(res)?;

        // create scatter plot
        res = respond(
            create_plot(&[
                ("dataviewId", &analysis.dataview.id.clone()),
                ("name", "scatter"),
                ("type", "SCATTER"),
                (
                    "args",
                    &vars_to_json_string(&[
                        ("x", "sepal_width"),
                        ("y", "petal_width"),
                        ("title", "Sepal vs. Petal Width"),
                        ("color", "species"),
                        ("shape", "species"),
                    ]),
                ),
            ]),
            &ctx,
        )
        .await;
        from_response::<PlotResponse>(res)?;

        // create smooth plot
        res = respond(
            create_plot(&[
                ("dataviewId", &analysis.dataview.id.clone()),
                ("name", "smooth plot"),
                ("type", "SMOOTH"),
                (
                    "args",
                    &vars_to_json_string(&[
                        ("x", "sepal_width"),
                        ("y", "petal_width"),
                        ("title", "Sepal vs. Petal Width"),
                        ("color", "species"),
                        ("shape", "species"),
                    ]),
                ),
            ]),
            &ctx,
        )
        .await;
        let mut plot = from_response::<PlotResponse>(res)?;

        // rename plot
        let new_plot_name = "smooth plot renamed";
        res = respond(
            rename_plot(&[
                ("plotId", &plot.id.clone()),
                ("name", new_plot_name),
            ]),
            &ctx,
        )
        .await;
        plot = from_response(res)?;
        if plot.name != new_plot_name {
            return Err(GQLError::new("failed to rename plot"));
        }

        // delete project
        res = respond(delete_node(&[("id", &project.id.clone())]), &ctx).await;
        if !res.is_ok() {
            return Err(GQLError::new("failed to delete project"));
        }

        // TODO(danj): 'no entry found for key "lambda-runtime-invoked-function-arn"'
        // https://github.com/aws/aws-lambda-runtime-interface-emulator/issues/11

        // garbage collect resources
        let lambda_req = InvocationRequest {
            function_name: "motoko-garbage-collect".to_owned(),
            ..Default::default()
        };
        lambda.invoke(lambda_req).await?;

        Ok(())
    }

    async fn test_ctx() -> Result<ContextData, GenericError> {
        let mut ctx = ContextData::default().await?;
        reset_databases(&ctx).await?;
        ctx.user = Some(
            User::create(
                &ctx.db,
                "motoko",
                "Motoko Kusanagi",
                "motoko.kusanagi@motoko.ai",
            )
            .await?,
        );
        Ok(ctx)
    }

    async fn reset_databases(ctx: &ContextData) -> SQLxResult<()> {
        truncate_tables(&ctx.db.meta).await?;
        drop_views(&ctx.db.data).await?;
        drop_tables(&ctx.db.data).await?;
        Ok(())
    }

    async fn truncate_tables(db: &sqlx::PgPool) -> SQLxResult<()> {
        query("SELECT truncate_tables()")
            .execute(db)
            .await
            .map(|_| ())
    }

    async fn drop_tables(db: &sqlx::PgPool) -> SQLxResult<()> {
        query("SELECT drop_tables()").execute(db).await.map(|_| ())
    }

    async fn drop_views(db: &sqlx::PgPool) -> SQLxResult<()> {
        query("SELECT drop_views()").execute(db).await.map(|_| ())
    }
}
