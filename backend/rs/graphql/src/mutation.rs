use crate::{
    auth::{
        credentials_for_user, validate_google_id_token, Credentials, Provider,
    },
    gql::{
        current_user, data, get_invocation_type, graphql_id_to_uuid,
        is_current_user, model_keys,
    },
    models::{
        Analysis, Dataset, Dataview, Model, Operation, Plot, PlotType, Project,
        ProjectUserRole, Role, Statistic, StatisticName, User,
        UserRefreshToken,
    },
    types::{CreatePlotPayload, UploadDatasetPayload},
    utils::{as_bytes, dataview_view_name, user_name_from_email},
    Error,
};
use async_graphql::{
    Context, Error as GQLError, Json as GQLJson, Result as GQLResult, ID,
};
use rusoto_lambda::{InvocationRequest, Lambda};
use serde_json::Value as Json;
use tokio_compat_02::FutureExt;
use uuid::Uuid;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    pub async fn login(
        &self,
        ctx: &Context<'_>,
        provider: Provider,
        token: String,
    ) -> GQLResult<Credentials> {
        let d = data(ctx)?;
        let oauth2_user = match provider {
            Provider::Google => {
                validate_google_id_token(
                    &d.auth.google_oauth2_client_id,
                    &token,
                )
                .await?
            }
        };
        let maybe_user = User::get_by_email(&d.db, &oauth2_user.email).await;
        let user = match maybe_user {
            Ok(user) => user,
            Err(_) => {
                User::create(
                    &d.db,
                    &user_name_from_email(&oauth2_user.email),
                    &oauth2_user.display_name,
                    &oauth2_user.email,
                )
                .await?
            }
        };
        let creds = credentials_for_user(&d.auth.jwt_secret, &user)?;
        UserRefreshToken::create(
            &d.db,
            &user.uuid,
            &creds.refresh_token,
            &creds.refresh_token_expires_at,
        )
        .await?;
        Ok(creds)
    }

    pub async fn refresh(
        &self,
        ctx: &Context<'_>,
        refresh_token: String,
    ) -> GQLResult<Credentials> {
        let d = data(ctx)?;
        let token = UserRefreshToken::get(&d.db, &refresh_token).await?;
        let user = User::get(&d.db, &token.user_uuid).await?;
        let creds = credentials_for_user(&d.auth.jwt_secret, &user)?;
        UserRefreshToken::create(
            &d.db,
            &user.uuid,
            &creds.refresh_token,
            &creds.refresh_token_expires_at,
        )
        .await?;
        UserRefreshToken::delete(&d.db, &refresh_token).await?;
        Ok(creds)
    }

    pub async fn delete_user_refresh_token(
        &self,
        ctx: &Context<'_>,
        user_refresh_token_id: ID,
    ) -> GQLResult<ID> {
        let d = data(ctx)?;
        let mkeys = model_keys(&user_refresh_token_id)?;
        let value = mkeys
            .keys
            .first()
            .ok_or::<GQLError>(Error::InvalidGraphQLID.into())?;
        let token = UserRefreshToken::get(&d.db, &value).await?;
        is_current_user(&token.user_uuid, ctx)?;
        UserRefreshToken::delete(&d.db, &value)
            .await
            .map(|_| user_refresh_token_id)
            .map_err(|e| e.into())
    }

    pub async fn change_user_name(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> GQLResult<User> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        User::rename(&d.db, &user.uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_node(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> GQLResult<ID> {
        let mkeys = model_keys(&id)?;
        match mkeys.model.as_str() {
            "Analysis" => Self::delete_analysis(&self, ctx, id).await?,
            "Dataset" => Self::delete_dataset(&self, ctx, id).await?,
            "Dataview" => Self::delete_dataview(&self, ctx, id).await?,
            "Model" => Self::delete_model(&self, ctx, id).await?,
            "Plot" => Self::delete_plot(&self, ctx, id).await?,
            "Project" => Self::delete_project(&self, ctx, id).await?,
            "ProjectUserRole" => {
                Self::delete_project_user_role(&self, ctx, id).await?
            }
            "Statistic" => Self::delete_statistic(&self, ctx, id).await?,
            "UserRefreshToken" => {
                Self::delete_user_refresh_token(&self, ctx, id).await?
            }
            _ => Err(Error::UnsupportedOperation.into()),
        }
    }

    pub async fn create_project(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> GQLResult<Project> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        Project::create(&d.db, &name, &user.uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn rename_project(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
        name: String,
    ) -> GQLResult<Project> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let project_uuid = graphql_id_to_uuid(&project_id)?;
        let pur = ProjectUserRole::get(&d.db, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Project::rename(&d.db, &project_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn make_project_public(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
    ) -> GQLResult<Project> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let project_uuid = graphql_id_to_uuid(&project_id)?;
        let pur = ProjectUserRole::get(&d.db, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role != Role::Admin {
            return Err(Error::RequiresAdminPermissions.into());
        }
        Project::make_public(&d.db, &project_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_project(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
    ) -> GQLResult<ID> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let project_uuid = graphql_id_to_uuid(&project_id)?;
        let pur = ProjectUserRole::get(&d.db, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role != Role::Admin {
            return Err(Error::RequiresAdminPermissions.into());
        }
        Project::delete(&d.db, &project_uuid)
            .await
            .map(|_| project_id)
            .map_err(|e| e.into())
    }

    pub async fn create_dataset(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
        name: String,
        uri: String,
    ) -> GQLResult<Dataset> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let project_uuid = graphql_id_to_uuid(&project_id)?;
        let pur = ProjectUserRole::get(&d.db, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        let ds = Dataset::create(&d.db, &project_uuid, &name, &uri)
            .await
            .map_err(|e| -> GQLError { e.into() })?;
        let payload = UploadDatasetPayload {
            uri: uri.clone(),
            uuid: ds.uuid.clone(),
        };
        let req = InvocationRequest {
            client_context: None,
            function_name: "motoko-uri-to-sql-db".to_owned(),
            invocation_type: get_invocation_type(),
            log_type: None,
            payload: Some(as_bytes(&payload)?),
            qualifier: None,
        };
        d.lambda.invoke(req).compat().await?;
        Ok(ds)
    }

    pub async fn rename_dataset(
        &self,
        ctx: &Context<'_>,
        dataset_id: ID,
        name: String,
    ) -> GQLResult<Dataset> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let dataset_uuid = graphql_id_to_uuid(&dataset_id)?;
        let role = Dataset::role(&d.db, &dataset_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Dataset::rename(&d.db, &dataset_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_dataset(
        &self,
        ctx: &Context<'_>,
        dataset_id: ID,
    ) -> GQLResult<ID> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let dataset_uuid = graphql_id_to_uuid(&dataset_id)?;
        let role = Dataset::role(&d.db, &dataset_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Dataset::delete(&d.db, &dataset_uuid)
            .await
            .map(|_| dataset_id)
            .map_err(|e| e.into())
    }

    pub async fn create_dataview(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
        operation: Operation,
        args: GQLJson<Json>,
    ) -> GQLResult<Dataview> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let dataview_uuid = graphql_id_to_uuid(&dataview_id)?;
        let role = Dataview::role(&d.db, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Dataview::create(&d.db, &dataview_uuid, &operation, &args)
            .await
            .map_err(|e| e.into())
        // TODO(danj): copy (to avoid on delete casade problems)
        // plots/statistics/models that are still valid?
        // select,sort,mutate are valid
        // Plot::link_if_valid(&d.db, &dv.uuid)
        // Statistic::link_if_valid(&d.db, &dv.uuid)
        // Model::link_if_valid(&d.db, &dv.uuid)
    }

    pub async fn delete_dataview(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
    ) -> GQLResult<ID> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let dataview_uuid = graphql_id_to_uuid(&dataview_id)?;
        let role = Dataview::role(&d.db, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        let dv = Dataview::get(&d.db, &dataview_uuid)
            .await
            .map_err(|e| -> GQLError { e.into() })?;
        if dv.uuid == dv.parent_uuid {
            return Err("cannot delete root dataview".into());
        }
        Dataview::delete(&d.db, &dataview_uuid)
            .await
            .map(|_| dataview_id)
            .map_err(|e| e.into())
    }

    pub async fn create_analysis(
        &self,
        ctx: &Context<'_>,
        dataset_id: ID,
        name: String,
    ) -> GQLResult<Analysis> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let dataset_uuid = graphql_id_to_uuid(&dataset_id)?;
        let role = Dataset::role(&d.db, &dataset_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Analysis::create(&d.db, &dataset_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn rename_analysis(
        &self,
        ctx: &Context<'_>,
        analysis_id: ID,
        name: String,
    ) -> GQLResult<Analysis> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let analysis_uuid = graphql_id_to_uuid(&analysis_id)?;
        let role = Analysis::role(&d.db, &analysis_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Analysis::rename(&d.db, &analysis_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn set_analysis_dataview(
        &self,
        ctx: &Context<'_>,
        analysis_id: ID,
        dataview_id: ID,
    ) -> GQLResult<Analysis> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let analysis_uuid = graphql_id_to_uuid(&analysis_id)?;
        let dataview_uuid = graphql_id_to_uuid(&dataview_id)?;
        let role = Analysis::role(&d.db, &analysis_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Analysis::point_to(&d.db, &analysis_uuid, &dataview_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_analysis(
        &self,
        ctx: &Context<'_>,
        analysis_id: ID,
    ) -> GQLResult<ID> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let analysis_uuid = graphql_id_to_uuid(&analysis_id)?;
        let role = Analysis::role(&d.db, &analysis_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Analysis::delete(&d.db, &analysis_uuid)
            .await
            .map(|_| analysis_id)
            .map_err(|e| e.into())
    }

    pub async fn create_role(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
        user_id: ID,
        role: Role,
    ) -> GQLResult<ProjectUserRole> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let project_uuid = graphql_id_to_uuid(&project_id)?;
        let user_uuid = graphql_id_to_uuid(&user_id)?;
        let pur = ProjectUserRole::get(&d.db, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role != Role::Admin {
            return Err(Error::RequiresAdminPermissions.into());
        }
        ProjectUserRole::create(&d.db, &project_uuid, &user_uuid, &role)
            .await
            .map_err(|e| e.into())
    }

    pub async fn modify_role(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
        user_id: ID,
        role: Role,
    ) -> GQLResult<ProjectUserRole> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let project_uuid = graphql_id_to_uuid(&project_id)?;
        let user_uuid = graphql_id_to_uuid(&user_id)?;
        let pur = ProjectUserRole::get(&d.db, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role != Role::Admin {
            return Err(Error::RequiresAdminPermissions.into());
        }
        let prev_role = ProjectUserRole::get(&d.db, &project_uuid, &user_uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if prev_role.role == role {
            return Ok(prev_role);
        }
        if prev_role.role == Role::Admin {
            let roles = ProjectUserRole::by_project(&d.db, &project_uuid)
                .await
                .map_err(|e| -> GQLError { e.into() })?;
            let admin_user_uuids: Vec<Uuid> = roles
                .iter()
                .filter(|r| r.role == Role::Admin)
                .map(|r| r.user_uuid)
                .collect();
            if admin_user_uuids.len() == 1 && admin_user_uuids[0] == user_uuid {
                return Err("a project must always have an admin".into());
            }
        }
        ProjectUserRole::modify(&d.db, &project_uuid, &user_uuid, &role)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_project_user_role(
        &self,
        ctx: &Context<'_>,
        project_user_role_id: ID,
    ) -> GQLResult<ID> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(&project_user_role_id)?;
        let project_uuid =
            mkeys
                .keys
                .get(0)
                .map(|v| Uuid::parse_str(v))
                .ok_or::<GQLError>(Error::InvalidGraphQLID.into())??;
        let user_uuid = mkeys
            .keys
            .get(1)
            .map(|v| Uuid::parse_str(v))
            .ok_or::<GQLError>(Error::InvalidGraphQLID.into())??;
        let pur = ProjectUserRole::get(&d.db, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role != Role::Admin {
            return Err(Error::RequiresAdminPermissions.into());
        }
        let roles = ProjectUserRole::by_project(&d.db, &project_uuid)
            .await
            .map_err(|e| -> GQLError { e.into() })?;
        let admin_user_uuids: Vec<Uuid> = roles
            .iter()
            .filter(|r| r.role == Role::Admin)
            .map(|r| r.user_uuid)
            .collect();
        if admin_user_uuids.len() == 1 && admin_user_uuids[0] == user_uuid {
            return Err("a project must always have an admin".into());
        }
        ProjectUserRole::delete(&d.db, &project_uuid, &user_uuid)
            .await
            .map(|_| project_user_role_id)
            .map_err(|e| e.into())
    }

    pub async fn create_statistic(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
        name: StatisticName,
        args: GQLJson<Json>,
    ) -> GQLResult<Statistic> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let dataview_uuid = graphql_id_to_uuid(&dataview_id)?;
        let role = Dataview::role(&d.db, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Statistic::create(&d.db, &dataview_uuid, &name, &args)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_statistic(
        &self,
        ctx: &Context<'_>,
        statistic_id: ID,
    ) -> GQLResult<ID> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let statistic_uuid = graphql_id_to_uuid(&statistic_id)?;
        let role = Statistic::role(&d.db, &statistic_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Statistic::delete(&d.db, &statistic_uuid)
            .await
            .map(|_| statistic_id)
            .map_err(|e| e.into())
    }

    pub async fn create_plot(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
        name: String,
        type_: PlotType,
        args: GQLJson<Json>,
    ) -> GQLResult<Plot> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let dataview_uuid = graphql_id_to_uuid(&dataview_id)?;
        let role = Dataview::role(&d.db, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        let p = Plot::create(&d.db, &dataview_uuid, &name, &type_, &args)
            .await
            .map_err(|e| -> GQLError { e.into() })?;
        let payload = CreatePlotPayload {
            view: dataview_view_name(&dataview_uuid),
            uuid: p.uuid.clone(),
            type_,
            args: (*args).clone(),
        };
        let req = InvocationRequest {
            function_name: "motoko-plot".to_owned(),
            invocation_type: get_invocation_type(),
            payload: Some(as_bytes(&payload)?),
            ..Default::default()
        };
        d.lambda.invoke(req).compat().await?;
        Ok(p)
    }

    pub async fn rename_plot(
        &self,
        ctx: &Context<'_>,
        plot_id: ID,
        name: String,
    ) -> GQLResult<Plot> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let plot_uuid = graphql_id_to_uuid(&plot_id)?;
        let role = Plot::role(&d.db, &plot_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Plot::rename(&d.db, &plot_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_plot(
        &self,
        ctx: &Context<'_>,
        plot_id: ID,
    ) -> GQLResult<ID> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let plot_uuid = graphql_id_to_uuid(&plot_id)?;
        let role = Plot::role(&d.db, &plot_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Plot::delete(&d.db, &plot_uuid)
            .await
            .map(|_| plot_id)
            .map_err(|e| e.into())
    }

    pub async fn create_model(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
        name: String,
        target: Option<String>,
        features: Vec<String>,
        args: GQLJson<Json>,
    ) -> GQLResult<Model> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let dataview_uuid = graphql_id_to_uuid(&dataview_id)?;
        let role = Dataview::role(&d.db, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Model::create(&d.db, &dataview_uuid, &name, &target, &features, &args)
            .await
            .map_err(|e| e.into())
    }

    pub async fn rename_model(
        &self,
        ctx: &Context<'_>,
        model_id: ID,
        name: String,
    ) -> GQLResult<Model> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let model_uuid = graphql_id_to_uuid(&model_id)?;
        let role = Model::role(&d.db, &model_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Model::rename(&d.db, &model_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_model(
        &self,
        ctx: &Context<'_>,
        model_id: ID,
    ) -> GQLResult<ID> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let model_uuid = graphql_id_to_uuid(&model_id)?;
        let role = Model::role(&d.db, &model_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Model::delete(&d.db, &model_uuid)
            .await
            .map(|_| model_id)
            .map_err(|e| e.into())
    }
}
