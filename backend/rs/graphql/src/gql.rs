use crate::{models::User, ContextData, Error, ModelKeys, Mutation, Query};
use async_graphql::{
    from_value, Context, EmptySubscription, Error as GQLError,
    Request as GQLRequest, Response as GQLResponse, Result as GQLResult,
    Schema, Value as GQLValue, ID,
};
use rusoto_core::Region;
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

pub fn get_invocation_type<'ctx>(ctx: &'ctx Context<'_>) -> Option<String> {
    // TODO(danj): can't invoke async locally [invocation_type Some("Event")]
    // https://github.com/aws/aws-sam-cli/pull/749
    // None defaults to RequestResponse
    match data(ctx) {
        Ok(d) => {
            // async
            if d.region == Region::UsWest1 {
                Some("Event".to_owned())
            } else {
                None
            }
        }
        Err(_) => None,
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
    use crate::{models::User, queries::*, Db, GenericError};
    use async_graphql::Result as GQLResult;
    use rusoto_core::Region;
    use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};
    use sqlx::{query, Result as SQLxResult};
    use std::process::Command;
    use std::{thread, time};
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
        Command::new("sam")
            .args(&["local", "start-lambda"])
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

        // create project2
        let mut res =
            respond(create_project(&[("name", "Test Project")]), &ctx).await;
        let mut p: ProjectResponse = from_response(res)?;
        let project_id = p.id;

        // make project public
        res = respond(
            make_project_public(&[("projectId", &project_id.clone())]),
            &ctx,
        )
        .await;
        p = from_response(res)?;
        if !p.is_public {
            return Err(GQLError::new("failed to make project public"));
        }

        // rename project
        let new_project_name = "Test Project Renamed";
        res = respond(
            rename_project(&[
                ("projectId", &project_id.clone()),
                ("name", new_project_name),
            ]),
            &ctx,
        )
        .await;
        p = from_response(res)?;
        if p.name != new_project_name {
            return Err(GQLError::new("failed to rename project"));
        }

        // create dataset
        res = respond(create_dataset(&[
            ("projectId", &project_id.clone()),
            ("name", "iris"),
            ("uri", "https://drive.google.com/file/d/12q0KWJAUaVba9RZrVY8QEXThK1x5GoF8/view?usp=sharing")
        ]), &ctx).await;
        let mut ds: DatasetResponse = from_response(res)?;
        let dataset_id = ds.id;

        // rename dataset
        let new_dataset_name = "iris renamed";
        res = respond(
            rename_dataset(&[
                ("datasetId", &dataset_id.clone()),
                ("name", new_dataset_name),
            ]),
            &ctx,
        )
        .await;
        ds = from_response(res)?;
        if ds.name != new_dataset_name {
            return Err(GQLError::new("failed to rename dataset"));
        }

        // delete project
        res = respond(
            delete_project(&[("projectId", &project_id.clone())]),
            &ctx,
        )
        .await;
        if !res.is_ok() {
            return Err(GQLError::new("failed to delete project"));
        }

        // TODO(danj): 'no entry found for key "lambda-runtime-invoked-function-arn"'
        // https://github.com/aws/aws-lambda-runtime-interface-emulator/issues/11

        // garbage collect resources
        let lambda_req = InvocationRequest {
            client_context: None,
            function_name: "motoko-garbage-collect".to_owned(),
            invocation_type: None,
            log_type: None,
            payload: None,
            qualifier: None,
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
        truncate_tables(&ctx.db).await?;
        drop_views(&ctx.data_db).await?;
        drop_tables(&ctx.data_db).await?;
        Ok(())
    }

    async fn truncate_tables(db: &Db) -> SQLxResult<()> {
        query("SELECT truncate_tables()")
            .execute(db)
            .await
            .map(|_| ())
    }

    async fn drop_tables(db: &Db) -> SQLxResult<()> {
        query("SELECT drop_tables()").execute(db).await.map(|_| ())
    }

    async fn drop_views(db: &Db) -> SQLxResult<()> {
        query("SELECT drop_views()").execute(db).await.map(|_| ())
    }
}
