use actix_web::{HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum MailerErrors {
    #[error("SMTP connection failed")]
    SmtpConnectionError,

    #[error("Failed to send email")]
    EmailSendError,

    #[error("Invalid email template")]
    InvalidTemplate,

    #[error("Token generation failed")]
    TokenGenerationError,

    #[error("Token not found or expired")]
    TokenNotFoundOrExpired,

    #[error("Token already used")]
    TokenAlreadyUsed,

    #[error("Database error occurred")]
    DatabaseError,

    #[error("User not found")]
    UserNotFound,

    #[error("Password hash error")]
    PasswordHashError,
}

impl ResponseError for MailerErrors {
    fn error_response(&self) -> HttpResponse {
        let status_code = match self {
            MailerErrors::SmtpConnectionError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            MailerErrors::EmailSendError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            MailerErrors::InvalidTemplate => actix_web::http::StatusCode::BAD_REQUEST,
            MailerErrors::TokenGenerationError => {
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
            }
            MailerErrors::TokenNotFoundOrExpired => actix_web::http::StatusCode::BAD_REQUEST,
            MailerErrors::TokenAlreadyUsed => actix_web::http::StatusCode::BAD_REQUEST,
            MailerErrors::DatabaseError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            MailerErrors::UserNotFound => actix_web::http::StatusCode::NOT_FOUND,
            MailerErrors::PasswordHashError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status_code).json(json!({
            "error": self.to_string(),
            "code": status_code.as_u16()
        }))
    }
}
