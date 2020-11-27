use crate::{
    auth::{
        credentials_for_user, validate_google_id_token, Credentials, Provider,
    },
    models::{
        Analysis, Dataset, Dataview, Model, Operation, Plot, PlotType, Project,
        ProjectUserRole, Role, Statistic, StatisticName, User,
        UserRefreshToken,
    },
    utils::{current_user, data, model_keys, user_name_from_email},
    Error,
};
use async_graphql::{Context, Error as GQLError, Json as GQLJson, Result, ID};
use serde_json::Value as Json;
use uuid::Uuid;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    pub async fn login(
        &self,
        ctx: &Context<'_>,
        provider: Provider,
        token: String,
    ) -> Result<Credentials> {
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
        let maybe_user = User::get_by_email(&d.pool, &oauth2_user.email).await;
        let user = match maybe_user {
            Ok(user) => user,
            Err(_) => {
                User::create(
                    &d.pool,
                    &user_name_from_email(&oauth2_user.email),
                    &oauth2_user.display_name,
                    &oauth2_user.email,
                )
                .await?
            }
        };
        let creds = credentials_for_user(&d.auth.jwt_secret, &user)?;
        UserRefreshToken::create(
            &d.pool,
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
    ) -> Result<Credentials> {
        let d = data(ctx)?;
        let token = UserRefreshToken::get(&d.pool, &refresh_token).await?;
        let user = User::get(&d.pool, &token.user_uuid).await?;
        let creds = credentials_for_user(&d.auth.jwt_secret, &user)?;
        UserRefreshToken::create(
            &d.pool,
            &user.uuid,
            &creds.refresh_token,
            &creds.refresh_token_expires_at,
        )
        .await?;
        token.delete(&d.pool).await?;
        Ok(creds)
    }

    pub async fn delete_refresh_token(
        &self,
        ctx: &Context<'_>,
        refresh_token: String,
    ) -> Result<String> {
        let d = data(ctx)?;
        let token = UserRefreshToken::get(&d.pool, &refresh_token).await?;
        token.delete(&d.pool).await?;
        Ok(refresh_token)
    }

    pub async fn change_user_name(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> Result<User> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        User::rename(&d.pool, &user.uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn create_project(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> Result<Project> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        Project::create(&d.pool, &name, &user.uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn rename_project(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
        name: String,
    ) -> Result<Project> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(project_id);
        let project_uuid = mkeys.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Project::rename(&d.pool, &project_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn make_project_public(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
    ) -> Result<Project> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(project_id);
        let project_uuid = mkeys.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role != Role::Admin {
            return Err(Error::RequiresAdminPermissions.into());
        }
        Project::make_public(&d.pool, &project_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_project(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
    ) -> Result<bool> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(project_id);
        let project_uuid = mkeys.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role != Role::Admin {
            return Err(Error::RequiresAdminPermissions.into());
        }
        Project::delete(&d.pool, &project_uuid)
            .await
            .map(|_| true)
            .map_err(|e| e.into())
    }

    pub async fn create_dataset(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
        name: String,
        uri: String,
    ) -> Result<Dataset> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(project_id);
        let project_uuid = mkeys.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        let ds = Dataset::create(&d.pool, &project_uuid, &name, &uri)
            .await
            .map_err(|e| -> GQLError { e.into() })?;
        // TODO(danj): call lambda function upload_dataset(uuid), and add data
        // types to upload
        Ok(ds)
    }

    pub async fn rename_dataset(
        &self,
        ctx: &Context<'_>,
        dataset_id: ID,
        name: String,
    ) -> Result<Dataset> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(dataset_id);
        let dataset_uuid = mkeys.keys.first().unwrap();
        let role = Dataset::role(&d.pool, &dataset_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Dataset::rename(&d.pool, &dataset_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_dataset(
        &self,
        ctx: &Context<'_>,
        dataset_id: ID,
    ) -> Result<bool> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(dataset_id);
        let dataset_uuid = mkeys.keys.first().unwrap();
        let role = Dataset::role(&d.pool, &dataset_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Dataset::delete(&d.pool, &dataset_uuid)
            .await
            .map(|_| true)
            .map_err(|e| e.into())
    }

    pub async fn create_dataview(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
        operation: Operation,
        args: GQLJson<Json>,
    ) -> Result<Dataview> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(dataview_id);
        let dataview_uuid = mkeys.keys.first().unwrap();
        let role = Dataview::role(&d.pool, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Dataview::create(&d.pool, &dataview_uuid, &operation, &args)
            .await
            .map_err(|e| e.into())
        // TODO(danj): copy (to avoid on delete casade problems)
        // plots/statistics/models that are still valid?
        // select,sort,mutate are valid
        // Plot::link_if_valid(&d.pool, &dv.uuid)
        // Statistic::link_if_valid(&d.pool, &dv.uuid)
        // Model::link_if_valid(&d.pool, &dv.uuid)
    }

    pub async fn delete_dataview(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
    ) -> Result<bool> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(dataview_id);
        let dataview_uuid = mkeys.keys.first().unwrap();
        let role = Dataview::role(&d.pool, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        let dv = Dataview::get(&d.pool, &dataview_uuid)
            .await
            .map_err(|e| -> GQLError { e.into() })?;
        if dv.uuid == dv.parent_uuid {
            return Err("cannot delete root dataview".into());
        }
        Dataview::delete(&d.pool, &dataview_uuid)
            .await
            .map(|_| true)
            .map_err(|e| e.into())
    }

    pub async fn create_analysis(
        &self,
        ctx: &Context<'_>,
        dataset_id: ID,
        name: String,
    ) -> Result<Analysis> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(dataset_id);
        let dataset_uuid = mkeys.keys.first().unwrap();
        let role = Dataset::role(&d.pool, &dataset_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Analysis::create(&d.pool, &dataset_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn rename_analysis(
        &self,
        ctx: &Context<'_>,
        analysis_id: ID,
        name: String,
    ) -> Result<Analysis> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(analysis_id);
        let analysis_uuid = mkeys.keys.first().unwrap();
        let role = Analysis::role(&d.pool, &analysis_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Analysis::rename(&d.pool, &analysis_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn set_analysis_dataview(
        &self,
        ctx: &Context<'_>,
        analysis_id: ID,
        dataview_id: ID,
    ) -> Result<Analysis> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let aks = model_keys(analysis_id);
        let dks = model_keys(dataview_id);
        let analysis_uuid = aks.keys.first().unwrap();
        let dataview_uuid = dks.keys.first().unwrap();
        let role = Analysis::role(&d.pool, &analysis_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Analysis::point_to(&d.pool, &analysis_uuid, &dataview_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_analysis(
        &self,
        ctx: &Context<'_>,
        analysis_id: ID,
    ) -> Result<bool> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(analysis_id);
        let analysis_uuid = mkeys.keys.first().unwrap();
        let role = Analysis::role(&d.pool, &analysis_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Analysis::delete(&d.pool, &analysis_uuid)
            .await
            .map(|_| true)
            .map_err(|e| e.into())
    }

    pub async fn create_role(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
        user_id: ID,
        role: Role,
    ) -> Result<ProjectUserRole> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let pks = model_keys(project_id);
        let uks = model_keys(user_id);
        let project_uuid = pks.keys.first().unwrap();
        let user_uuid = uks.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role != Role::Admin {
            return Err(Error::RequiresAdminPermissions.into());
        }
        ProjectUserRole::create(&d.pool, &project_uuid, &user_uuid, &role)
            .await
            .map_err(|e| e.into())
    }

    pub async fn modify_role(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
        user_id: ID,
        role: Role,
    ) -> Result<ProjectUserRole> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let pks = model_keys(project_id);
        let uks = model_keys(user_id);
        let project_uuid = pks.keys.first().unwrap();
        let user_uuid = uks.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role != Role::Admin {
            return Err(Error::RequiresAdminPermissions.into());
        }
        let prev_role =
            ProjectUserRole::get(&d.pool, &project_uuid, &user_uuid)
                .await
                .map_err(|_| -> GQLError {
                    Error::InvalidPermissions.into()
                })?;
        if prev_role.role == role {
            return Ok(prev_role);
        }
        if prev_role.role == Role::Admin {
            let roles = ProjectUserRole::by_project(&d.pool, &project_uuid)
                .await
                .map_err(|e| -> GQLError { e.into() })?;
            let admin_user_uuids: Vec<Uuid> = roles
                .iter()
                .filter(|r| r.role == Role::Admin)
                .map(|r| r.user_uuid)
                .collect();
            if admin_user_uuids.len() == 1 && admin_user_uuids[0] == *user_uuid
            {
                return Err("a project must always have an admin".into());
            }
        }
        ProjectUserRole::modify(&d.pool, &project_uuid, &user_uuid, &role)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_role(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
        user_id: ID,
    ) -> Result<bool> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let pks = model_keys(project_id);
        let uks = model_keys(user_id);
        let project_uuid = pks.keys.first().unwrap();
        let user_uuid = uks.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if pur.role != Role::Admin {
            return Err(Error::RequiresAdminPermissions.into());
        }
        let roles = ProjectUserRole::by_project(&d.pool, &project_uuid)
            .await
            .map_err(|e| -> GQLError { e.into() })?;
        let admin_user_uuids: Vec<Uuid> = roles
            .iter()
            .filter(|r| r.role == Role::Admin)
            .map(|r| r.user_uuid)
            .collect();
        if admin_user_uuids.len() == 1 && admin_user_uuids[0] == *user_uuid {
            return Err("a project must always have an admin".into());
        }
        ProjectUserRole::delete(&d.pool, &project_uuid, &user_uuid)
            .await
            .map(|_| true)
            .map_err(|e| e.into())
    }

    pub async fn create_statistic(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
        name: StatisticName,
        args: GQLJson<Json>,
    ) -> Result<Statistic> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(dataview_id);
        let dataview_uuid = mkeys.keys.first().unwrap();
        let role = Dataview::role(&d.pool, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Statistic::create(&d.pool, &dataview_uuid, &name, &args)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_statistic(
        &self,
        ctx: &Context<'_>,
        statistic_id: ID,
    ) -> Result<bool> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(statistic_id);
        let statistic_uuid = mkeys.keys.first().unwrap();
        let role = Statistic::role(&d.pool, &statistic_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Statistic::delete(&d.pool, &statistic_uuid)
            .await
            .map(|_| true)
            .map_err(|e| e.into())
    }

    pub async fn create_plot(
        &self,
        ctx: &Context<'_>,
        dataview_id: ID,
        name: String,
        type_: PlotType,
        args: GQLJson<Json>,
    ) -> Result<Plot> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(dataview_id);
        let dataview_uuid = mkeys.keys.first().unwrap();
        let role = Dataview::role(&d.pool, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Plot::create(&d.pool, &dataview_uuid, &name, &type_, &args)
            .await
            .map_err(|e| e.into())
    }

    pub async fn rename_plot(
        &self,
        ctx: &Context<'_>,
        plot_id: ID,
        name: String,
    ) -> Result<Plot> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(plot_id);
        let plot_uuid = mkeys.keys.first().unwrap();
        let role = Plot::role(&d.pool, &plot_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Plot::rename(&d.pool, &plot_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_plot(
        &self,
        ctx: &Context<'_>,
        plot_id: ID,
    ) -> Result<bool> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(plot_id);
        let plot_uuid = mkeys.keys.first().unwrap();
        let role = Plot::role(&d.pool, &plot_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Plot::delete(&d.pool, &plot_uuid)
            .await
            .map(|_| true)
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
    ) -> Result<Model> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(dataview_id);
        let dataview_uuid = mkeys.keys.first().unwrap();
        let role = Dataview::role(&d.pool, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Model::create(&d.pool, &dataview_uuid, &name, &target, &features, &args)
            .await
            .map_err(|e| e.into())
    }

    pub async fn rename_model(
        &self,
        ctx: &Context<'_>,
        model_id: ID,
        name: String,
    ) -> Result<Model> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(model_id);
        let model_uuid = mkeys.keys.first().unwrap();
        let role = Model::role(&d.pool, &model_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Model::rename(&d.pool, &model_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_model(
        &self,
        ctx: &Context<'_>,
        model_id: ID,
    ) -> Result<bool> {
        let d = data(ctx)?;
        let user = current_user(ctx)?;
        let mkeys = model_keys(model_id);
        let model_uuid = mkeys.keys.first().unwrap();
        let role = Model::role(&d.pool, &model_uuid, &user.uuid)
            .await
            .map_err(|_| -> GQLError { Error::InvalidPermissions.into() })?;
        if role == Role::Viewer {
            return Err(Error::RequiresEditorPermissions.into());
        }
        Model::delete(&d.pool, &model_uuid)
            .await
            .map(|_| true)
            .map_err(|e| e.into())
    }

    pub async fn infer_from_uri(
        &self,
        ctx: &Context<'_>,
        uri: String,
    ) -> Result<String> {
        // TODO(danj): call infer cloud function from here
        Ok("".into())
    }
}
