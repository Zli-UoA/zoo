use sea_orm::DatabaseConnection;

pub struct Context {
    pub env: String,
    pub db: DatabaseConnection,
}
