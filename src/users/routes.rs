use actix_failwrap::proof_route;
use actix_web::{HttpResponse, web};

use crate::errors::http::HttpErrors;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get);
}

#[proof_route("GET /users")]
async fn get() -> Result<HttpResponse, HttpErrors> {
    Err(HttpErrors::InternalServerError)
}
