use actix_failwrap::proof_route;
use actix_web::{
    HttpResponse, Result,
    web::{self, Data, Json},
};
use serde_json::json;

use crate::{
    AppState,
    helpers::hash_password::verify_password,
    helpers::validate_password::is_valid_password,
    mailer::errors::MailerErrors,
    mailer::{ForgotPasswordRequest, ForgotPasswordResponse, MailerService, ResetPasswordRequest},
    users::dtos::AuthUser,
    users::entities::PartialUser,
    users::errors::auth::AuthErrors,
};

/// Configure user routes
///
/// `POST` `/register` - Register a new user
///
/// `POST` `/login` - Login an existing user
/// Auth User entity:
/// ```no_run
/// #[derive(Debug, Validate, Deserialize)]
/// pub struct AuthUser {
///     #[validate(email)]
///     #[validate(length(min = 5, max = 100))]
///     pub email: String,
///     #[validate(length(min = 8))]
///     pub password: String,
/// }
/// ```
/// `POST` `/forgot-password` - Request password reset email
///
/// Forgot Password Request entity:
/// ```no_run
/// #[derive(Debug, Validate, Deserialize)]
/// pub struct ForgotPasswordRequest {
///     #[validate(email)]
///     #[validate(length(min = 5, max = 100))]
///     pub email: String,
/// }
/// ```
///
/// `POST` `/reset-password` - Reset password with token
///
/// Reset Password Request entity:
/// ```no_run
/// #[derive(Debug, Validate, Deserialize)]
/// pub struct ResetPasswordRequest {
///     #[validate(length(min = 32, max = 255))]
///     pub token: String,
///     #[validate(length(min = 8))]
///     pub new_password: String,
/// }
/// ```
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(register_user)
        .service(login_user)
        .service(forgot_password)
        .service(reset_password);
}

#[proof_route("POST /register")]
async fn register_user(
    state: Data<AppState>,
    body: Json<AuthUser>,
) -> Result<HttpResponse, AuthErrors> {
    if !is_valid_password(&body.password) {
        return Err(AuthErrors::WeakPassword);
    }

    let token = PartialUser::create_user(&state, &body).await?;
    Ok(HttpResponse::Ok().json(token))
}

#[proof_route("POST /login")]
async fn login_user(
    state: Data<AppState>,
    body: Json<AuthUser>,
) -> Result<HttpResponse, AuthErrors> {
    let user = PartialUser::authenticate_user(&state, &body.email).await?;

    if !verify_password(body.password.clone(), user.password_hash) {
        return Err(AuthErrors::InvalidCredentials);
    }

    let token = PartialUser::generate_user_token(user.id, user.role_name)?;
    Ok(HttpResponse::Ok().json(token))
}

#[proof_route("POST /forgot-password")]
async fn forgot_password(
    state: Data<AppState>,
    body: Json<ForgotPasswordRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    let message = MailerService::send_password_reset_email(&state, &body.email)
        .await
        .map_err(actix_web::Error::from)?;
    Ok(HttpResponse::Ok().json(ForgotPasswordResponse { message }))
}

#[proof_route("POST /reset-password")]
async fn reset_password(
    state: Data<AppState>,
    body: Json<ResetPasswordRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    if !is_valid_password(&body.new_password) {
        return Err(actix_web::Error::from(MailerErrors::PasswordHashError));
    }

    let message = MailerService::reset_password(&state, &body.token, &body.new_password)
        .await
        .map_err(actix_web::Error::from)?;
    Ok(HttpResponse::Ok().json(json!({ "message": message })))
}
