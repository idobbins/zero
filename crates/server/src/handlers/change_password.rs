use crate::config::MinPasswordLength;
use crate::schema::users;
use crate::types::user::User;
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::http::StatusCode;
use axum::{Extension, Form};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use tower_sessions::Session;
use uuid::Uuid;

type DbPool = diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncPgConnection>;

#[derive(Deserialize)]
pub struct Request {
    current_password: String,
    new_password: String,
}

pub async fn change_password(
    Extension(db): Extension<DbPool>,
    Extension(min_password_length): Extension<MinPasswordLength>,
    session: Session,
    Form(request): Form<Request>,
) -> StatusCode {
    // Check if user is authenticated
    let user_id = match session.get::<Uuid>("user_id").await {
        Ok(Some(id)) => id,
        Ok(None) => return StatusCode::UNAUTHORIZED,
        Err(_) => return StatusCode::UNAUTHORIZED,
    };

    // Validate new password length
    if request.new_password.len() < min_password_length.0 {
        return StatusCode::BAD_REQUEST;
    }

    let mut conn = match db.get().await {
        Ok(conn) => conn,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Get user from database
    let user = match users::table
        .filter(users::id.eq(user_id))
        .first::<User>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Verify current password
    let hash = match PasswordHash::new(&user.password_hash) {
        Ok(h) => h,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    match Argon2::default().verify_password(request.current_password.as_bytes(), &hash) {
        Ok(_) => {}
        Err(_) => return StatusCode::BAD_REQUEST,
    }

    // Hash the new password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let new_password_hash = match argon2.hash_password(request.new_password.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Update user with new password
    let result = diesel::update(users::table.filter(users::id.eq(user_id)))
        .set((
            users::password_hash.eq(&new_password_hash),
            users::updated_at.eq(Utc::now()),
        ))
        .execute(&mut conn)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
