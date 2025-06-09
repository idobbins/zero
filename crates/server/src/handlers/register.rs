use crate::email::localsmtp::Client;
use crate::schema::users;
use crate::types::user::{NewUser, User};
use crate::Config;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::http::StatusCode;
use axum::{Extension, Form};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use tracing::error;
use uuid::Uuid;

type DbPool = diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncPgConnection>;

#[derive(Deserialize)]
pub struct Request {
    email: String,
    password: String,
}

pub async fn register(
    Extension(db): Extension<DbPool>,
    Extension(client): Extension<Client>,
    Extension(config): Extension<Config>,
    Form(register): Form<Request>,
) -> StatusCode {
    // Validate password length
    if register.password.len() < config.min_password_length {
        return StatusCode::BAD_REQUEST;
    }

    // Hash the password (done first to mitigate timing attacks)
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(register.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let mut conn = match db.get().await {
        Ok(conn) => conn,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Check if user already exists
    let existing_user = match users::table
        .filter(users::email.eq(&register.email))
        .first::<User>(&mut conn)
        .await
        .optional()
    {
        Ok(user) => user,
        Err(e) => {
            error!("{}", e);
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };

    let now = Utc::now();
    let email_validation_token = Uuid::new_v4();

    match existing_user {
        Some(user) => {
            // User exists - check if email is verified
            if user.email_verified.unwrap_or(false) {
                // Email is already verified - registration not allowed
                return StatusCode::CONFLICT;
            }

            // Email is not verified - update existing user with new registration attempt
            let result = diesel::update(users::table.filter(users::id.eq(user.id)))
                .set((
                    users::password_hash.eq(&password_hash),
                    users::email_validation_token.eq(email_validation_token),
                    users::email_validation_token_issued_at.eq(now),
                    users::updated_at.eq(now),
                ))
                .execute(&mut conn)
                .await;

            match result {
                Ok(_) => {
                    send_verification_email(
                        &client,
                        &register.email,
                        &config.base_url,
                        &config.from_email,
                        email_validation_token,
                    );
                    StatusCode::CREATED
                }
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
        None => {
            // User doesn't exist - create new user
            let user_id = Uuid::new_v4();

            let new_user = NewUser {
                id: user_id,
                email: register.email.clone(),
                password_hash: password_hash,
                is_active: Some(true),
                email_verified: Some(false),
                created_at: Some(now),
                updated_at: Some(now),
                email_validation_token: Some(email_validation_token),
                email_validation_token_issued_at: Some(now),
            };

            let result = diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut conn)
                .await;

            match result {
                Ok(_) => {
                    // Send verification email
                    send_verification_email(
                        &client,
                        &register.email,
                        &config.base_url,
                        &config.from_email,
                        email_validation_token,
                    );
                    StatusCode::CREATED
                }
                Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
            }
        }
    }
}

fn send_verification_email(
    client: &Client,
    email: &str,
    client_url: &str,
    from_email: &str,
    token: Uuid,
) {
    let verification_link = format!("{}/verify-email?token={}", client_url, token);
    let subject = "Verify Your Email Address".to_string();
    let body = format!(
        "Welcome! Please verify your email address by clicking the link below:\n\n{}\n\nIf you didn't create an account, you can safely ignore this email.",
        verification_link
    );

    client.send(from_email.to_string(), email.to_string(), subject, body);
}
