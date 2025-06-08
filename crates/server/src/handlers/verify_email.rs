use crate::schema::users;
use crate::types::user::User;
use axum::Extension;
use axum::extract::Query;
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

type DbPool = diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncPgConnection>;

#[derive(Deserialize)]
pub struct Request {
    token: Uuid,
}

pub async fn verify_email(
    Extension(db): Extension<DbPool>,
    Query(request): Query<Request>,
) -> StatusCode {
    let mut conn = match db.get().await {
        Ok(conn) => conn,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Get user by email validation token
    let user = match users::table
        .filter(users::email_validation_token.eq(request.token))
        .first::<User>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::BAD_REQUEST,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Check if email is already verified
    if user.email_verified.unwrap_or(false) {
        return StatusCode::OK;
    }

    // Check if token has expired (24 hours)
    if let Some(issued_at) = user.email_validation_token_issued_at {
        let now = Utc::now();
        let expiry_time = issued_at + Duration::hours(24);
        if now > expiry_time {
            return StatusCode::BAD_REQUEST;
        }
    } else {
        return StatusCode::BAD_REQUEST;
    }

    // Update user to mark email as verified and clear validation token
    let result = diesel::update(users::table.filter(users::id.eq(user.id)))
        .set((
            users::email_verified.eq(true),
            users::email_validation_token.eq(None::<Uuid>),
            users::email_validation_token_issued_at.eq(None::<chrono::DateTime<chrono::Utc>>),
            users::updated_at.eq(Utc::now()),
        ))
        .execute(&mut conn)
        .await;

    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
