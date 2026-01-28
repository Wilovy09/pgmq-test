use std::borrow::Cow;

use uuid::Uuid;

use crate::{
    AppState,
    helpers::hash_password::hash_password,
    middlewares::jwt::{TokenStruct, generate_token},
    users::{
        AuthUser,
        entities::{PartialUser, UserWithRole},
        errors::auth::AuthErrors,
    },
};

impl PartialUser {
    pub async fn create_user(
        state: &AppState,
        auth_user: &AuthUser,
    ) -> Result<TokenStruct, AuthErrors> {
        let hashed_password =
            hash_password(auth_user.password.clone()).map_err(|_| AuthErrors::PasswordHashError)?;

        let email = auth_user.email.clone();
        let username = auth_user
            .email
            .split('@')
            .next()
            .unwrap_or("user")
            .to_string();

        let mut tx = state
            .db_pool
            .begin()
            .await
            .map_err(|_| AuthErrors::DatabaseError)?;

        let user_result = sqlx::query!(
            "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id",
            username,
            email,
            hashed_password
        )
        .fetch_one(&mut *tx)
        .await;

        let user_id = match user_result {
            Ok(user) => user.id,
            Err(e) => {
                let _ = tx.rollback().await;
                if let sqlx::Error::Database(db_err) = &e
                    && db_err.code() == Some(Cow::Borrowed("23505"))
                {
                    return Err(AuthErrors::EmailAlreadyRegistered);
                }
                return Err(AuthErrors::DatabaseError);
            }
        };

        let role_result = sqlx::query!("SELECT id FROM catalogs.roles WHERE name = 'user'")
            .fetch_one(&mut *tx)
            .await;

        let role_id = match role_result {
            Ok(role) => role.id,
            Err(_) => {
                let _ = tx.rollback().await;
                return Err(AuthErrors::DefaultRoleNotFound);
            }
        };

        match sqlx::query!(
            "INSERT INTO users_role (user_id, role_id) VALUES ($1, $2)",
            user_id,
            role_id
        )
        .execute(&mut *tx)
        .await
        {
            Ok(_) => {}
            Err(_) => {
                let _ = tx.rollback().await;
                return Err(AuthErrors::TransactionError);
            }
        }

        if tx.commit().await.is_err() {
            return Err(AuthErrors::TransactionError);
        }

        Self::generate_user_token(user_id, "user".to_string())
    }

    pub async fn authenticate_user(
        state: &AppState,
        email: &str,
    ) -> Result<UserWithRole, AuthErrors> {
        let user_result = sqlx::query_as::<_, UserWithRole>(
            "SELECT u.id, u.email, u.password_hash, r.name as role_name
             FROM users u 
             JOIN users_role ur ON u.id = ur.user_id 
             JOIN catalogs.roles r ON ur.role_id = r.id 
             WHERE u.email = $1 
             LIMIT 1",
        )
        .bind(email)
        .fetch_optional(&state.db_pool)
        .await
        .map_err(|_| AuthErrors::DatabaseError)?;

        match user_result {
            Some(user) => Ok(user),
            None => {
                // Try to get user without role (fallback)
                let user = sqlx::query_as::<_, PartialUser>(
                    "SELECT id, email, password_hash FROM users WHERE email = $1",
                )
                .bind(email)
                .fetch_optional(&state.db_pool)
                .await
                .map_err(|_| AuthErrors::DatabaseError)?;

                match user {
                    Some(user) => Ok(UserWithRole {
                        id: user.id,
                        email: user.email,
                        password_hash: user.password_hash,
                        role_name: "user".to_string(), // default role
                    }),
                    None => Err(AuthErrors::InvalidCredentials),
                }
            }
        }
    }

    pub fn generate_user_token(
        user_id: Uuid,
        user_role: String,
    ) -> Result<TokenStruct, AuthErrors> {
        let iss = "PGMQ-Backend";
        let duration_in_minutes: i64 = 525600; // 1 year
        let token = generate_token(
            iss.to_string(),
            duration_in_minutes,
            "access".to_owned(),
            user_id,
            user_role,
        );

        Ok(TokenStruct { token })
    }
}
