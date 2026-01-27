use actix_failwrap::ErrorResponse;
use thiserror::Error;

#[derive(Debug, ErrorResponse, Error)]
pub enum HttpErrors {
    #[error("Internal server error")]
    #[status_code(500)]
    InternalServerError,

    #[error("Bad request")]
    #[status_code(400)]
    BadRequest,

    #[error("Not found")]
    #[status_code(404)]
    NotFound,

    #[error("Unauthorized")]
    #[status_code(401)]
    Unauthorized,
}
