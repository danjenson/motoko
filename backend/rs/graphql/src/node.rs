use crate::{
    models::{
        Analysis, Dataset, Dataview, Model, Plot, Project, ProjectUserRole,
        Statistic, User, UserRefreshToken,
    },
    types::Pool,
    utils::model_keys,
};
use async_graphql::{Interface, Result, ID};
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

pub async fn id_to_node(pool: &Pool, id: ID) -> Result<Node> {
    let mkeys = model_keys(id);
    let pk1 = mkeys.keys.first().unwrap();

    match mkeys.model.as_str() {
        "Analysis" => {
            let uuid = Uuid::parse_str(pk1)?;
            let analysis = Analysis::get(pool, &uuid).await?;
            Ok(Node::Analysis(analysis))
        }
        "Dataset" => {
            let uuid = Uuid::parse_str(pk1)?;
            let dataset = Dataset::get(pool, &uuid).await?;
            Ok(Node::Dataset(dataset))
        }
        "Dataview" => {
            let uuid = Uuid::parse_str(pk1)?;
            let dataview = Dataview::get(pool, &uuid).await?;
            Ok(Node::Dataview(dataview))
        }
        "Model" => {
            let uuid = Uuid::parse_str(pk1)?;
            let model = Model::get(pool, &uuid).await?;
            Ok(Node::Model(model))
        }
        "Plot" => {
            let uuid = Uuid::parse_str(pk1)?;
            let plot = Plot::get(pool, &uuid).await?;
            Ok(Node::Plot(plot))
        }
        "Project" => {
            let uuid = Uuid::parse_str(pk1)?;
            let project = Project::get(pool, &uuid).await?;
            Ok(Node::Project(project))
        }
        "ProjectUserRole" => {
            let project_uuid = Uuid::parse_str(pk1)?;
            let user_uuid = Uuid::parse_str(mkeys.keys.get(1).unwrap())?;
            let pur =
                ProjectUserRole::get(pool, &project_uuid, &user_uuid).await?;
            Ok(Node::ProjectUserRole(pur))
        }
        "Statistic" => {
            let uuid = Uuid::parse_str(pk1)?;
            let stat = Statistic::get(pool, &uuid).await?;
            Ok(Node::Statistic(stat))
        }
        "User" => {
            let uuid = Uuid::parse_str(pk1)?;
            let user = User::get(pool, &uuid).await?;
            Ok(Node::User(user))
        }
        "UserRefreshToken" => {
            let token = UserRefreshToken::get(pool, pk1).await?;
            Ok(Node::UserRefreshToken(token))
        }
        _ => Err("invalid data type".into()),
    }
}
