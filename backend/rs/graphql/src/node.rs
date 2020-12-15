use crate::{
    gql::{graphql_id_to_uuid, model_keys},
    models::{
        Analysis, Dataset, Dataview, Model, Plot, Project, ProjectUserRole,
        Statistic, User, UserRefreshToken,
    },
    types::Db,
    Error,
};
use async_graphql::{Error as GQLError, Interface, Result as GQLResult, ID};
use uuid::Uuid;

#[derive(Interface)]
#[graphql(field(name = "id", type = "ID"))]
pub enum Node {
    Analysis(Analysis),
    Dataset(Dataset),
    Dataview(Dataview),
    Model(Model),
    Plot(Plot),
    Project(Project),
    ProjectUserRole(ProjectUserRole),
    Statistic(Statistic),
    User(User),
    UserRefreshToken(UserRefreshToken),
}

pub async fn id_to_node(db: &Db, id: &ID) -> GQLResult<Node> {
    let mkeys = model_keys(&id)?;

    match mkeys.model.as_str() {
        "Analysis" => {
            let uuid = graphql_id_to_uuid(&id)?;
            let analysis = Analysis::get(db, &uuid).await?;
            Ok(Node::Analysis(analysis))
        }
        "Dataset" => {
            let uuid = graphql_id_to_uuid(&id)?;
            let dataset = Dataset::get(db, &uuid).await?;
            Ok(Node::Dataset(dataset))
        }
        "Dataview" => {
            let uuid = graphql_id_to_uuid(&id)?;
            let dataview = Dataview::get(db, &uuid).await?;
            Ok(Node::Dataview(dataview))
        }
        "Model" => {
            let uuid = graphql_id_to_uuid(&id)?;
            let model = Model::get(db, &uuid).await?;
            Ok(Node::Model(model))
        }
        "Plot" => {
            let uuid = graphql_id_to_uuid(&id)?;
            let plot = Plot::get(db, &uuid).await?;
            Ok(Node::Plot(plot))
        }
        "Project" => {
            let uuid = graphql_id_to_uuid(&id)?;
            let project = Project::get(db, &uuid).await?;
            Ok(Node::Project(project))
        }
        "ProjectUserRole" => {
            let uuid = graphql_id_to_uuid(&id)?;
            let user_key = mkeys
                .keys
                .get(1)
                .ok_or::<GQLError>(Error::InvalidGraphQLID.into())?;
            let user_uuid = Uuid::parse_str(user_key)
                .map_err(|_| -> GQLError { Error::InvalidGraphQLID.into() })?;
            let pur = ProjectUserRole::get(db, &uuid, &user_uuid).await?;
            Ok(Node::ProjectUserRole(pur))
        }
        "Statistic" => {
            let uuid = graphql_id_to_uuid(&id)?;
            let stat = Statistic::get(db, &uuid).await?;
            Ok(Node::Statistic(stat))
        }
        "User" => {
            let uuid = graphql_id_to_uuid(&id)?;
            let user = User::get(db, &uuid).await?;
            Ok(Node::User(user))
        }
        "UserRefreshToken" => {
            let value = mkeys
                .keys
                .first()
                .ok_or::<GQLError>(Error::InvalidGraphQLID.into())?;
            let token = UserRefreshToken::get(db, value).await?;
            Ok(Node::UserRefreshToken(token))
        }
        _ => Err("invalid data type".into()),
    }
}
