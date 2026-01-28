use std::time::Duration;

use lettre::{Message, SmtpTransport, Transport, message::header::ContentType};
use uuid::Uuid;

use crate::{
    AppState,
    config::Config,
    helpers::hash_password::hash_password,
    mailer::{
        entities::{EmailTemplate, PasswordResetToken},
        errors::MailerErrors,
    },
    users::entities::PartialUser,
};

pub struct MailerService;

impl MailerService {
    pub async fn send_password_reset_email(
        state: &AppState,
        email: &str,
    ) -> Result<String, MailerErrors> {
        let user = sqlx::query_as::<_, PartialUser>(
            "SELECT id, email, password_hash FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| MailerErrors::DatabaseError)?;

        let user = user.ok_or(MailerErrors::UserNotFound)?;

        let token = Self::generate_reset_token();
        let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);

        let _token_record = sqlx::query!(
            "INSERT INTO password_reset_tokens (id, user_id, token, expires_at, used, created_at) 
             VALUES ($1, $2, $3, $4, $5, $6) 
             RETURNING id",
            Uuid::new_v4(),
            user.id,
            token,
            expires_at,
            false,
            chrono::Utc::now()
        )
        .fetch_one(&state.db_pool)
        .await
        .map_err(|_| MailerErrors::DatabaseError)?;

        let reset_url = format!(
            "{}/reset-password?token={}",
            Config::from_env().frontend_url,
            token
        );
        let email_template = EmailTemplate {
            to: email.to_string(),
            subject: "Password Reset Request".to_string(),
            body: format!(
                "Hello,\n\nYou have requested to reset your password. Please click the link below to reset your password:\n\n{}\n\nThis link will expire in 1 hour.\n\nIf you did not request this, please ignore this email.\n\nBest regards,\nPGMQ Team",
                reset_url
            ),
        };

        Self::send_email(&email_template).await?;

        Ok("Password reset email sent successfully".to_string())
    }

    pub async fn reset_password(
        state: &AppState,
        token: &str,
        new_password: &str,
    ) -> Result<String, MailerErrors> {
        let token_record = sqlx::query_as::<_, PasswordResetToken>(
            "SELECT id, user_id, token, expires_at, used, created_at 
             FROM password_reset_tokens 
             WHERE token = $1",
        )
        .bind(token)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| MailerErrors::DatabaseError)?;

        let token_record = token_record.ok_or(MailerErrors::TokenNotFoundOrExpired)?;

        if !token_record.is_valid() {
            if token_record.used {
                return Err(MailerErrors::TokenAlreadyUsed);
            } else {
                return Err(MailerErrors::TokenNotFoundOrExpired);
            }
        }

        let hashed_password =
            hash_password(new_password.to_string()).map_err(|_| MailerErrors::PasswordHashError)?;

        let mut tx = state
            .db_pool
            .begin()
            .await
            .map_err(|_| MailerErrors::DatabaseError)?;

        sqlx::query!(
            "UPDATE users SET password_hash = $1 WHERE id = $2",
            hashed_password,
            token_record.user_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| MailerErrors::DatabaseError)?;

        sqlx::query!(
            "UPDATE password_reset_tokens SET used = true WHERE id = $1",
            token_record.id
        )
        .execute(&mut *tx)
        .await
        .map_err(|_| MailerErrors::DatabaseError)?;

        tx.commit().await.map_err(|_| MailerErrors::DatabaseError)?;

        Ok("Password reset successfully".to_string())
    }

    async fn send_email(template: &EmailTemplate) -> Result<(), MailerErrors> {
        let from_address = format!(
            "{} <{}>",
            Config::from_env().smtp_from_name,
            Config::from_env().smtp_from_email
        );
        let email = Message::builder()
            .from(
                from_address
                    .parse()
                    .map_err(|_| MailerErrors::InvalidTemplate)?,
            )
            .to(template
                .to
                .parse()
                .map_err(|_| MailerErrors::InvalidTemplate)?)
            .subject(&template.subject)
            .header(ContentType::TEXT_PLAIN)
            .body(template.body.clone())
            .map_err(|_| MailerErrors::InvalidTemplate)?;

        let mailer = SmtpTransport::builder_dangerous(&Config::from_env().smtp_host)
            .port(Config::from_env().smtp_port)
            .timeout(Some(Duration::from_secs(10)))
            .build();

        mailer
            .send(&email)
            .map_err(|_| MailerErrors::EmailSendError)?;

        Ok(())
    }

    fn generate_reset_token() -> String {
        use uuid::Uuid;
        format!("{}{}", Uuid::new_v4(), Uuid::new_v4()).replace("-", "")
    }
}
