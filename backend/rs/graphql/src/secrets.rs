use crate::GenericError;
use rusoto_core::Region;
use rusoto_secretsmanager::{
    GetSecretValueRequest, SecretsManager, SecretsManagerClient,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secrets {
    pub data_db_url: String,
    pub meta_db_url: String,
    pub google_oauth2_client_id: String,
    pub jwt_secret: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
}

impl Secrets {
    pub fn local() -> Self {
        Self {
            data_db_url: "postgres://postgres@localhost/motoko_data".to_owned(),
            meta_db_url: "postgres://postgres@localhost/motoko_meta".to_owned(),
            google_oauth2_client_id: "dummy".to_owned(),
            jwt_secret: "dummy".to_owned(),
            aws_access_key_id: "dummy".to_owned(),
            aws_secret_access_key: "dummy".to_owned(),
        }
    }

    pub fn docker() -> Self {
        Self {
            data_db_url: "postgres://postgres@172.17.0.1:5432/motoko_data"
                .to_owned(),
            meta_db_url: "postgres://postgres@172.17.0.1:5432/motoko_meta"
                .to_owned(),
            google_oauth2_client_id: "dummy".to_owned(),
            jwt_secret: "dummy".to_owned(),
            aws_access_key_id: "dummy".to_owned(),
            aws_secret_access_key: "dummy".to_owned(),
        }
    }

    pub async fn aws() -> Result<Self, GenericError> {
        let res = SecretsManagerClient::new(Region::UsWest1)
            .get_secret_value(GetSecretValueRequest {
                secret_id: "motoko".to_owned(),
                ..Default::default()
            })
            .await?;
        serde_json::from_str::<Secrets>(&res.secret_string.unwrap())
            .map_err(|e| e.into())
    }
}
