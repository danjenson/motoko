use crate::{models::User, utils::run_mode, Db, GenericError, Secrets};
use rusoto_core::Region;
use rusoto_credential::AwsCredentials;
use rusoto_lambda::LambdaClient;
use sqlx::postgres::PgPoolOptions;
use tokio_compat_02::FutureExt;

#[derive(Clone)]
pub struct ContextData {
    pub user: Option<User>,
    pub db: Db,
    pub auth: Auth,
    pub lambda: LambdaClient,
    pub storage: Storage,
}

#[derive(Debug, Clone)]
pub struct Auth {
    pub aws_credentials: AwsCredentials,
    pub jwt_secret: String,
    pub client_ids: OAuth2ClientIds,
}

#[derive(Debug, Clone)]
pub struct OAuth2ClientIds {
    pub google: GoogleOAuthClientIds,
}

#[derive(Debug, Clone)]
pub struct GoogleOAuthClientIds {
    pub android: String,
    pub ios: String,
}

#[derive(Debug, Clone)]
pub struct Storage {
    pub region: Region,
    pub bucket: String,
}

impl ContextData {
    pub async fn default() -> Result<Self, GenericError> {
        let (region, secrets) = match run_mode().as_str() {
            "local" => (
                Region::Custom {
                    name: "us-west-1".to_owned(),
                    endpoint: "http://127.0.0.1:3001".to_owned(),
                },
                Secrets::local(),
            ),
            _ => (Region::UsWest1, Secrets::aws().await?),
        };
        let lambda = LambdaClient::new(region);
        let meta_db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&secrets.meta_db_url)
            .compat()
            .await?;
        let data_db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&secrets.data_db_url)
            .compat()
            .await?;
        let client_ids = OAuth2ClientIds {
            google: GoogleOAuthClientIds {
                android: secrets.google_oauth2_client_id_android,
                ios: secrets.google_oauth2_client_id_ios,
            },
        };
        Ok(Self {
            user: None,
            db: Db {
                meta: meta_db,
                data: data_db,
            },
            auth: Auth {
                aws_credentials: AwsCredentials::new(
                    secrets.aws_access_key_id,
                    secrets.aws_secret_access_key,
                    None,
                    None,
                ),
                jwt_secret: secrets.jwt_secret.clone(),
                client_ids,
            },
            lambda,
            storage: Storage {
                region: Region::UsWest1,
                bucket: "motoko-data".to_owned(),
            },
        })
    }
}
