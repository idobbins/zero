use crate::schema::users;
use crate::types::user::User;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Extension, Form};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use tower_sessions::Session;

type DbPool = diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncPgConnection>;

#[derive(Deserialize)]
pub struct Request {
    email: String,
    password: String,
}

pub async fn login(
    db: Extension<DbPool>,
    session: Session,
    Form(login): Form<Request>,
) -> impl IntoResponse {
    let mut conn = match db.get().await {
        Ok(conn) => conn,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    let user = match users::table
        .filter(users::email.eq(&login.email))
        .first::<User>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::UNAUTHORIZED,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    if !user.is_active.unwrap_or(false) {
        return StatusCode::FORBIDDEN;
    }

    if !user.email_verified.unwrap_or(false) {
        return StatusCode::FORBIDDEN;
    }

    let hash = match PasswordHash::new(&user.password_hash) {
        Ok(h) => h,
        Err(_) => return StatusCode::UNAUTHORIZED,
    };

    match Argon2::default().verify_password(login.password.as_bytes(), &hash) {
        Ok(_) => {}
        Err(_) => return StatusCode::UNAUTHORIZED,
    }

    match session.insert("user_id", user.id).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
