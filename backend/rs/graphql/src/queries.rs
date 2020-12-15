use crate::{models::Status, types::ColumnDataType};
use async_graphql::{Name, Request, Value, Variables};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value as Json;
use std::collections::BTreeMap;

type Vars<'v> = [(&'v str, &'v str)];

fn make_request(query: String, vars: &Vars) -> Request {
    let m: BTreeMap<Name, Value> = vars
        .iter()
        .map(|(k, v)| (Name::new(k.to_owned()), v.to_owned().into()))
        .collect();
    Request::new(query).variables(Variables(m))
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

pub fn delete_project(vars: &Vars) -> Request {
    make_request(
        r#"
        mutation DeleteProject($projectId: ID!) {
            deleteProject(projectId: $projectId)
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
