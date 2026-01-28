use actix_failwrap::ErrorResponse;
use thiserror::Error;

#[derive(Debug, ErrorResponse, Error)]
pub enum AuthErrors {
    #[error("Invalid credentials")]
    #[status_code(401)]
    InvalidCredentials,

    #[error("Email already registered")]
    #[status_code(400)]
    EmailAlreadyRegistered,

    #[error(
        "Password must be at least 8 characters long, contain at least one uppercase letter, and one special character"
    )]
    #[status_code(400)]
    WeakPassword,

    #[allow(dead_code)]
    #[error("User not found")]
    #[status_code(404)]
    UserNotFound,

    #[error("Default role not found")]
    #[status_code(500)]
    DefaultRoleNotFound,

    #[error("Database transaction error")]
    #[status_code(500)]
    TransactionError,

    #[error("Error hashing password")]
    #[status_code(500)]
    PasswordHashError,

    #[allow(dead_code)]
    #[error("Error generating token")]
    #[status_code(500)]
    TokenGenerationError,

    #[error("Database error")]
    #[status_code(500)]
    DatabaseError,
}
