use crate::{
    db::Pool,
    models::{
        analysis::Analysis, dataset::Dataset, dataview::Dataview, model::Model,
        plot::Plot, project::Project, project_user_role::ProjectUserRole,
        statistic::Statistic, user::User,
    },
    utils::graphql_id_to_db_keys,
};
use async_graphql::{Result, Interface, ID};
use std::str;

#[derive(Clone, Debug, Interface)]
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
    let dbks = graphql_id_to_db_keys(id);
    let pk1 = dbks.keys.first().unwrap();

    match dbks.model_name.as_str() {
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
            let pk2 = dbks.keys.get(1).unwrap();
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
