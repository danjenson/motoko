use crate::{models::User, Pool};

#[derive(Debug, Clone)]
pub struct ContextData {
    pub user: Option<User>,
    pub pool: Pool,
    pub auth: Auth,
}

#[derive(Debug, Clone)]
pub struct Auth {
    pub jwt_secret: String,
    pub google_oauth2_client_id: String,
}
