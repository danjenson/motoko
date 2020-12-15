use crate::{models::User, Db, GenericError};
use dotenv::dotenv;
use rusoto_core::Region;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tokio_compat_02::FutureExt;

#[derive(Debug, Clone)]
pub struct ContextData {
    pub user: Option<User>,
    pub db: Db,
    pub data_db: Db,
    pub region: Region,
    pub auth: Auth,
}

#[derive(Debug, Clone)]
pub struct Auth {
    pub jwt_secret: String,
    pub google_oauth2_client_id: String,
}

impl ContextData {
    pub async fn default() -> Result<Self, GenericError> {
        dotenv().ok();
        let google_oauth2_client_id = env::var("GOOGLE_OAUTH2_CLIENT_ID")?;
        let jwt_secret = env::var("JWT_KEY")?;
        let db_url = env::var("DATABASE_URL")?;
        let data_db_url = env::var("DATA_DATABASE_URL")?;
        let region = match env::var("RUN_MODE_LOCAL") {
            Ok(_) => Region::Custom {
                name: "us-west-1".to_owned(),
                endpoint: "http://127.0.0.1:3001".to_owned(),
            },
            Err(_) => Region::UsWest1,
        };
        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .compat()
            .await?;
        let data_db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&data_db_url)
            .compat()
            .await?;
        Ok(Self {
            user: None,
            db,
            data_db,
            region,
            auth: Auth {
                jwt_secret,
                google_oauth2_client_id,
            },
        })
    }
}
