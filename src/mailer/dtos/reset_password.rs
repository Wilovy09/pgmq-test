use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
pub struct ForgotPasswordRequest {
    #[validate(email)]
    #[validate(length(min = 5, max = 100))]
    pub email: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct ResetPasswordRequest {
    #[validate(length(min = 32, max = 255))]
    pub token: String,
    #[validate(length(min = 8))]
    pub new_password: String,
}

#[derive(Debug, Serialize)]
pub struct ForgotPasswordResponse {
    pub message: String,
}
