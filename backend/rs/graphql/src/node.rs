use crate::{
    models::{
        Analysis, Dataset, Dataview, Model, Plot, Project, ProjectUserRole,
        Statistic, User,
    },
    types::Pool,
    utils::model_keys,
};
use async_graphql::{Interface, Result, ID};

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
}

pub async fn id_to_node(pool: &Pool, id: ID) -> Result<Node> {
    let mkeys = model_keys(id);
    let pk1 = mkeys.keys.first().unwrap();

    match mkeys.model.as_str() {
        "Analysis" => {
            let analysis = Analysis::get(pool, pk1).await?;
            Ok(Node::Analysis(analysis))
        }
        "Dataset" => {
            let dataset = Dataset::get(pool, pk1).await?;
            Ok(Node::Dataset(dataset))
        }
        "Dataview" => {
            let dataview = Dataview::get(pool, pk1).await?;
            Ok(Node::Dataview(dataview))
        }
        "Model" => {
            let model = Model::get(pool, pk1).await?;
            Ok(Node::Model(model))
        }
        "Plot" => {
            let plot = Plot::get(pool, pk1).await?;
            Ok(Node::Plot(plot))
        }
        "Project" => {
            let project = Project::get(pool, pk1).await?;
            Ok(Node::Project(project))
        }
        "ProjectUserRole" => {
            let pk2 = mkeys.keys.get(1).unwrap();
            let pur = ProjectUserRole::get(pool, pk1, pk2).await?;
            Ok(Node::ProjectUserRole(pur))
        }
        "Statistic" => {
            let stat = Statistic::get(pool, pk1).await?;
            Ok(Node::Statistic(stat))
        }
        "User" => {
            let user = User::get(pool, pk1).await?;
            Ok(Node::User(user))
        }
        _ => Err("invalid data type".into()),
    }
}
