use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct AuthUser {
    #[validate(email)]
    #[validate(length(min = 5, max = 100))]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
#[allow(dead_code)]
pub struct CreateUser {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    #[validate(length(min = 5, max = 100))]
    pub email: String,
    #[validate(length(min = 3, max = 100))]
    pub full_name: Option<String>,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
