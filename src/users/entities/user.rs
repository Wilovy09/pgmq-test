use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Serialize, FromRow)]
#[allow(dead_code)]
pub struct FullUser {
    pub id: Uuid,
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    #[validate(length(min = 5, max = 100))]
    pub email: String,
    #[validate(length(min = 3, max = 100))]
    pub full_name: Option<String>,
    pub password_hash: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, FromRow)]
pub struct PartialUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, FromRow)]
pub struct UserWithRole {
    pub id: Uuid,
    #[allow(dead_code)]
    pub email: String,
    pub password_hash: String,
    pub role_name: String,
}
