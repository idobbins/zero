use crate::Config;
use crate::schema::users;
use crate::types::user::User;
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::http::StatusCode;
use axum::{Extension, Form};
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

type DbPool = diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncPgConnection>;

#[derive(Deserialize)]
pub struct Request {
    token: Uuid,
    password: String,
}

pub async fn reset_password(
    Extension(db): Extension<DbPool>,
    Extension(config): Extension<Config>,
    Form(request): Form<Request>,
) -> StatusCode {
    // Validate password length
    if request.password.len() < config.min_password_length {
        return StatusCode::BAD_REQUEST;
    }

    let mut conn = match db.get().await {
        Ok(conn) => conn,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Get user by password reset token
    let user = match users::table
        .filter(users::password_reset_token.eq(request.token))
        .first::<User>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::BAD_REQUEST,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Check if token has expired (1 hour)
    if let Some(issued_at) = user.password_reset_token_issued_at {
        let now = Utc::now();
        let expiry_time = issued_at + Duration::hours(1);
        if now > expiry_time {
            return StatusCode::BAD_REQUEST;
        }
    } else {
        return StatusCode::BAD_REQUEST;
    }

    // Hash the new password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = match argon2.hash_password(request.password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Update user with new password and clear reset token
    let result = diesel::update(users::table.filter(users::id.eq(user.id)))
        .set((
            users::password_hash.eq(&password_hash),
            users::password_reset_token.eq(None::<Uuid>),
            users::password_reset_token_issued_at.eq(None::<chrono::DateTime<chrono::Utc>>),
            users::updated_at.eq(Utc::now()),
        ))
        .execute(&mut conn)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
