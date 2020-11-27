use uuid::Uuid;
pub type Pool = sqlx::PgPool;

pub struct ModelKeys {
    pub model: String,
    pub keys: Vec<Uuid>,
}
