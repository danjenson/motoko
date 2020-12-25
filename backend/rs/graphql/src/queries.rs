use crate::{
    models::{Operation, PlotType, Status},
    utils::vars_to_variables,
    ColumnDataType, Json, Vars,
};
use async_graphql::Request;
use chrono::{DateTime, Utc};
use serde::Deserialize;

fn make_request(query: String, vars: &Vars) -> Request {
    Request::new(query).variables(vars_to_variables(vars))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectResponse {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub is_public: bool,
}

const PROJECT_FRAGMENT: &'static str = r#"
    __typename 
    id
    createdAt
    updatedAt
    name
    isPublic
"#;

pub fn create_project(vars: &Vars) -> Request {
    make_request(
        format!(
            r#"
        mutation CreateProject($name: String!) {{
            createProject(name: $name) {{
                {}
            }}
        }}
    "#,
            PROJECT_FRAGMENT
        ),
        vars,
    )
}

pub fn rename_project(vars: &Vars) -> Request {
    make_request(
        format!(
            r#"
        mutation RenameProject($projectId: ID!, $name: String!) {{
            renameProject(projectId: $projectId, name: $name) {{
                {}
        }}
    }}
    "#,
            PROJECT_FRAGMENT
        ),
        vars,
    )
}

pub fn make_project_public(vars: &Vars) -> Request {
    make_request(
        format!(
            r#"
        mutation MakeProjectPublic($projectId: ID!) {{
            makeProjectPublic(projectId: $projectId) {{
                {}
            }}
        }}
    "#,
            PROJECT_FRAGMENT
        ),
        vars,
    )
}

pub fn delete_node(vars: &Vars) -> Request {
    make_request(
        r#"
        mutation DeleteNode($id: ID!) {
            deleteNode(id: $id)
        }
        "#
        .to_owned(),
        vars,
    )
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetResponse {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub status: Status,
    pub schema: Vec<ColumnDataType>,
    pub sample_rows: Json,
}

const DATASET_FRAGMENT: &'static str = r#"
    __typename 
    id
    createdAt
    updatedAt
    name
    status
    schema {
        columnName
        dataType
    }
    sampleRows
"#;

pub fn create_dataset(vars: &Vars) -> Request {
    make_request(
        format!(
            r#"
        mutation CreateDataset($projectId: ID!, $name: String!, $uri: String!) {{
            createDataset(projectId: $projectId, name: $name, uri: $uri) {{
                {}
            }}
        }}
        "#,
            DATASET_FRAGMENT
        ),
        vars,
    )
}

pub fn rename_dataset(vars: &Vars) -> Request {
    make_request(
        format!(
            r#"
        mutation RenameDataset($datasetId: ID!, $name: String!) {{
            renameDataset(datasetId: $datasetId, name: $name) {{
                {}
            }}
        }}
        "#,
            DATASET_FRAGMENT
        ),
        vars,
    )
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnalysisResponse {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub dataset: DatasetResponse,
    pub dataview: DataviewResponse,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataviewResponse {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub operation: Operation,
    pub args: Option<Json>,
    pub status: Status,
}

const ANALYSIS_FRAGMENT: &'static str = r#"
    __typename 
    id
    createdAt
    updatedAt
    name
    dataset {
        __typename 
        id
        createdAt
        updatedAt
        name
        status
        schema {
            columnName
            dataType
        }
        sampleRows
    }
    dataview {
        __typename
        id
        createdAt
        updatedAt
        operation
        args
        status
    }
"#;

pub fn create_analysis(vars: &Vars) -> Request {
    make_request(
        format!(
            r#"
        mutation CreateAnalysis($datasetId: ID!, $name: String!) {{
            createAnalysis(datasetId: $datasetId, name: $name) {{
                {}
            }}
        }}
        "#,
            ANALYSIS_FRAGMENT,
        ),
        vars,
    )
}

pub fn rename_analysis(vars: &Vars) -> Request {
    make_request(
        format!(
            r#"
        mutation RenameAnalysis($analysisId: ID!, $name: String!) {{
            renameAnalysis(analysisId: $analysisId, name: $name) {{
                {}
            }}
        }}
        "#,
            ANALYSIS_FRAGMENT,
        ),
        vars,
    )
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlotResponse {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(rename = "type")]
    pub type_: PlotType,
    pub args: Json,
    pub status: Status,
}

const PLOT_FRAGMENT: &'static str = r#"
    __typename 
    id
    createdAt
    updatedAt
    name
    type
    args
    status
"#;

pub fn create_plot(vars: &Vars) -> Request {
    make_request(
        format!(
            r#"
        mutation CreatePlot(
            $dataviewId: ID!,
            $name: String!,
            $type: PlotType!,
            $args: JSON!,
        ) {{
            createPlot(
                dataviewId: $dataviewId,
                name: $name,
                type: $type,
                args: $args) {{
                {}
            }}
        }}
        "#,
            PLOT_FRAGMENT,
        ),
        vars,
    )
}

pub fn rename_plot(vars: &Vars) -> Request {
    make_request(
        format!(
            r#"
        mutation RenamePlot($plotId: ID!, $name: String!) {{
            renamePlot(plotId: $plotId, name: $name) {{
                {}
            }}
        }}
        "#,
            PLOT_FRAGMENT,
        ),
        vars,
    )
}
