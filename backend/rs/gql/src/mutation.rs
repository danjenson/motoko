use crate::{
    auth::{
        credentials_for_user, validate_google_id_token, Credentials, Provider,
    },
    models::{
        analysis::Analysis,
        dataset::Dataset,
        dataview::{Dataview, Operation},
        model::Model,
        plot::{Plot, Type},
        project::Project,
        project_user_role::{ProjectUserRole, Role},
        statistic::{Name, Statistic},
        user::User,
        user_refresh_token::UserRefreshToken,
    },
    utils::{
        current_user, get_data, graphql_id_to_db_keys, user_name_from_email,
    },
};
use async_graphql::{Context, FieldError, FieldResult, Json, ID};
use futures::TryFutureExt;
use lazy_static::lazy_static;
use uuid::Uuid;

#[derive(Debug)]
pub struct MutationRoot;

lazy_static! {
    static ref INVALID_PERMISSIONS: FieldError = "invalid permissions".into();
    static ref REQUIRES_ADMIN: FieldError = "requires admin privileges".into();
    static ref REQUIRES_EDITOR: FieldError =
        "requires editor or admin privileges".into();
}

#[async_graphql::Object]
impl MutationRoot {
    pub async fn login(
        &self,
        ctx: &Context<'_>,
        provider: Provider,
        token: String,
    ) -> FieldResult<Credentials> {
        let d = get_data(ctx)?;
        let oauth2_user = match provider {
            Provider::Google => {
                validate_google_id_token(
                    &d.auth.google_oauth2_client_id,
                    &token,
                )
                .await?
            }
        };
        let user = User::get_by_email(&d.pool, &oauth2_user.email)
            .or_else(|_| async {
                User::create(
                    &d.pool,
                    &user_name_from_email(&oauth2_user.email),
                    &oauth2_user.display_name,
                    &oauth2_user.email,
                )
                .await
            })
            .await?;
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
    ) -> FieldResult<Credentials> {
        let d = get_data(ctx)?;
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
    ) -> FieldResult<String> {
        let d = get_data(ctx)?;
        let token = UserRefreshToken::get(&d.pool, &refresh_token).await?;
        token.delete(&d.pool).await?;
        Ok(refresh_token)
    }

    pub async fn change_user_name(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> FieldResult<User> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        User::rename(&d.pool, &user.uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn create_project(
        &self,
        ctx: &Context<'_>,
        name: String,
    ) -> FieldResult<Project> {
        let d = get_data(ctx)?;
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
    ) -> FieldResult<Project> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(project_id);
        let project_uuid = dbks.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| REQUIRES_EDITOR.clone())?;
        if pur.role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
        }
        Project::rename(&d.pool, &project_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn make_project_public(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
    ) -> FieldResult<Project> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(project_id);
        let project_uuid = dbks.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| REQUIRES_EDITOR.clone())?;
        if pur.role != Role::Admin {
            return Err(REQUIRES_ADMIN.clone());
        }
        Project::make_public(&d.pool, &project_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_project(
        &self,
        ctx: &Context<'_>,
        project_id: ID,
    ) -> FieldResult<bool> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(project_id);
        let project_uuid = dbks.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if pur.role != Role::Admin {
            return Err(REQUIRES_ADMIN.clone());
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
    ) -> FieldResult<Dataset> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(project_id);
        let project_uuid = dbks.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if pur.role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
        }
        let ds = Dataset::create(&d.pool, &project_uuid, &name, &uri)
            .await
            .map_err(|e| -> FieldError { e.into() })?;
        // TODO(danj): call lambda function upload_dataset(uuid)
        Ok(ds)
    }

    pub async fn rename_dataset(
        &self,
        ctx: &Context<'_>,
        dataset_id: ID,
        name: String,
    ) -> FieldResult<Dataset> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(dataset_id);
        let dataset_uuid = dbks.keys.first().unwrap();
        let role = Dataset::role(&d.pool, &dataset_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
        }
        Dataset::rename(&d.pool, &dataset_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_dataset(
        &self,
        ctx: &Context<'_>,
        dataset_id: ID,
    ) -> FieldResult<bool> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(dataset_id);
        let dataset_uuid = dbks.keys.first().unwrap();
        let role = Dataset::role(&d.pool, &dataset_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
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
        args: Json<serde_json::Value>,
    ) -> FieldResult<Dataview> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(dataview_id);
        let dataview_uuid = dbks.keys.first().unwrap();
        let role = Dataview::role(&d.pool, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
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
    ) -> FieldResult<bool> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(dataview_id);
        let dataview_uuid = dbks.keys.first().unwrap();
        let role = Dataview::role(&d.pool, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
        }
        let dv = Dataview::get(&d.pool, &dataview_uuid)
            .await
            .map_err(|e| -> FieldError { e.into() })?;
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
    ) -> FieldResult<Analysis> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(dataset_id);
        let dataset_uuid = dbks.keys.first().unwrap();
        let role = Dataset::role(&d.pool, &dataset_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
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
    ) -> FieldResult<Analysis> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(analysis_id);
        let analysis_uuid = dbks.keys.first().unwrap();
        let role = Analysis::role(&d.pool, &analysis_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
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
    ) -> FieldResult<Analysis> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let aks = graphql_id_to_db_keys(analysis_id);
        let dks = graphql_id_to_db_keys(dataview_id);
        let analysis_uuid = aks.keys.first().unwrap();
        let dataview_uuid = dks.keys.first().unwrap();
        let role = Analysis::role(&d.pool, &analysis_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
        }
        Analysis::point_to(&d.pool, &analysis_uuid, &dataview_uuid)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_analysis(
        &self,
        ctx: &Context<'_>,
        analysis_id: ID,
    ) -> FieldResult<bool> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(analysis_id);
        let analysis_uuid = dbks.keys.first().unwrap();
        let role = Analysis::role(&d.pool, &analysis_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
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
    ) -> FieldResult<ProjectUserRole> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let pks = graphql_id_to_db_keys(project_id);
        let uks = graphql_id_to_db_keys(user_id);
        let project_uuid = pks.keys.first().unwrap();
        let user_uuid = uks.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if pur.role != Role::Admin {
            return Err(REQUIRES_ADMIN.clone());
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
    ) -> FieldResult<ProjectUserRole> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let pks = graphql_id_to_db_keys(project_id);
        let uks = graphql_id_to_db_keys(user_id);
        let project_uuid = pks.keys.first().unwrap();
        let user_uuid = uks.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if pur.role != Role::Admin {
            return Err(REQUIRES_ADMIN.clone());
        }
        let prev_role =
            ProjectUserRole::get(&d.pool, &project_uuid, &user_uuid)
                .await
                .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if prev_role.role == role {
            return Ok(prev_role);
        }
        if prev_role.role == Role::Admin {
            let roles = ProjectUserRole::by_project(&d.pool, &project_uuid)
                .await
                .map_err(|e| -> FieldError { e.into() })?;
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
    ) -> FieldResult<bool> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let pks = graphql_id_to_db_keys(project_id);
        let uks = graphql_id_to_db_keys(user_id);
        let project_uuid = pks.keys.first().unwrap();
        let user_uuid = uks.keys.first().unwrap();
        let pur = ProjectUserRole::get(&d.pool, &project_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if pur.role != Role::Admin {
            return Err(REQUIRES_ADMIN.clone());
        }
        let roles = ProjectUserRole::by_project(&d.pool, &project_uuid)
            .await
            .map_err(|e| -> FieldError { e.into() })?;
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
        name: Name,
        args: Json<serde_json::Value>,
    ) -> FieldResult<Statistic> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(dataview_id);
        let dataview_uuid = dbks.keys.first().unwrap();
        let role = Dataview::role(&d.pool, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
        }
        Statistic::create(&d.pool, &dataview_uuid, &name, &args)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_statistic(
        &self,
        ctx: &Context<'_>,
        statistic_id: ID,
    ) -> FieldResult<bool> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(statistic_id);
        let statistic_uuid = dbks.keys.first().unwrap();
        let role = Statistic::role(&d.pool, &statistic_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
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
        type_: Type,
        args: Json<serde_json::Value>,
    ) -> FieldResult<Plot> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(dataview_id);
        let dataview_uuid = dbks.keys.first().unwrap();
        let role = Dataview::role(&d.pool, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
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
    ) -> FieldResult<Plot> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(plot_id);
        let plot_uuid = dbks.keys.first().unwrap();
        let role = Plot::role(&d.pool, &plot_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
        }
        Plot::rename(&d.pool, &plot_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_plot(
        &self,
        ctx: &Context<'_>,
        plot_id: ID,
    ) -> FieldResult<bool> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(plot_id);
        let plot_uuid = dbks.keys.first().unwrap();
        let role = Plot::role(&d.pool, &plot_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
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
        args: Json<serde_json::Value>,
    ) -> FieldResult<Model> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(dataview_id);
        let dataview_uuid = dbks.keys.first().unwrap();
        let role = Dataview::role(&d.pool, &dataview_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
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
    ) -> FieldResult<Model> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(model_id);
        let model_uuid = dbks.keys.first().unwrap();
        let role = Model::role(&d.pool, &model_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
        }
        Model::rename(&d.pool, &model_uuid, &name)
            .await
            .map_err(|e| e.into())
    }

    pub async fn delete_model(
        &self,
        ctx: &Context<'_>,
        model_id: ID,
    ) -> FieldResult<bool> {
        let d = get_data(ctx)?;
        let user = current_user(ctx)?;
        let dbks = graphql_id_to_db_keys(model_id);
        let model_uuid = dbks.keys.first().unwrap();
        let role = Model::role(&d.pool, &model_uuid, &user.uuid)
            .await
            .map_err(|_| INVALID_PERMISSIONS.clone())?;
        if role == Role::Viewer {
            return Err(REQUIRES_EDITOR.clone());
        }
        Model::delete(&d.pool, &model_uuid)
            .await
            .map(|_| true)
            .map_err(|e| e.into())
    }
}