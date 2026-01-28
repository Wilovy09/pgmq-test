use crate::AppState;
use actix_web::web::Data;
use actix_web::{Error, dev::ServiceRequest, error};
use actix_web_grants::authorities::AttachAuthorities;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sqlx::Error as SqlxError;
use std::env;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenStruct {
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub iss: String,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
    pub user_id: Uuid,
    pub user_role: String,
}

pub fn get_secret_key() -> String {
    env::var("SECRET_KEY").expect("SECRET_KEY must be set")
}

pub fn generate_token(
    iss: String,
    duration_minutes: i64,
    token_type: String,
    user_id: Uuid,
    user_role: String,
) -> String {
    let header = Header::new(Algorithm::HS512);
    let encoding_key = EncodingKey::from_secret(get_secret_key().as_bytes());
    let exp = (Utc::now() + Duration::minutes(duration_minutes)).timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;
    let my_claims = Claims {
        iss,
        exp,
        iat,
        token_type,
        user_id,
        user_role,
    };
    encode(&header, &my_claims, &encoding_key).unwrap()
}

pub fn validate_token(token: String) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(Algorithm::HS512);
    let decoding_key = DecodingKey::from_secret(get_secret_key().as_bytes());
    let result = decode::<Claims>(&token, &decoding_key, &validation);
    match result {
        Ok(c) => Ok(c.claims),
        Err(e) => Err(e),
    }
}

pub async fn validator(
    req: ServiceRequest,
    credenciales: Option<BearerAuth>,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let Some(credenciales) = credenciales else {
        return Err((error::ErrorBadRequest("Token not specified"), req));
    };
    let token = credenciales.token();
    let state = match req.app_data::<Data<AppState>>() {
        Some(data) => data,
        None => {
            return Err((
                error::ErrorInternalServerError("Could not get application state."),
                req,
            ));
        }
    };
    match validate_token(token.to_owned()) {
        Ok(token) => {
            match sqlx::query!(
                "SELECT r.name as role_name FROM users u 
                 JOIN users_role ur ON u.id = ur.user_id 
                 JOIN catalogs.roles r ON ur.role_id = r.id 
                 WHERE u.id = $1",
                token.user_id
            )
            .fetch_one(&state.db_pool)
            .await
            {
                Ok(record) => {
                    let role = record.role_name;
                    match role.as_str() {
                        "admin" => req.attach(vec!["admin".to_string()]),
                        "user" => req.attach(vec!["user".to_string()]),
                        "store_manager" => req.attach(vec!["store_manager".to_string()]),
                        "store_admin" => req.attach(vec!["store_admin".to_string()]),
                        _ => req.attach(vec!["user".to_string()]), // default role
                    }
                    Ok(req)
                }
                Err(SqlxError::RowNotFound) => Err((error::ErrorNotFound("User not found."), req)),
                Err(_) => Err((
                    error::ErrorInternalServerError("Database query error."),
                    req,
                )),
            }
        }
        Err(_) => Err((error::ErrorForbidden("Access denied."), req)),
    }
}
