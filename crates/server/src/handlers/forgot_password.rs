use crate::email::localsmtp::Client;
use crate::schema::users;
use crate::types::user::User;
use crate::Config;
use axum::http::StatusCode;
use axum::{Extension, Form};
use chrono::Utc;
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Deserialize;
use uuid::Uuid;

type DbPool = diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncPgConnection>;

#[derive(Deserialize)]
pub struct Request {
    email: String,
}

pub async fn forgot_password(
    Extension(db): Extension<DbPool>,
    Extension(client): Extension<Client>,
    Extension(config): Extension<Config>,
    Form(request): Form<Request>,
) -> StatusCode {
    let mut conn = match db.get().await {
        Ok(conn) => conn,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Check if user exists and email is verified
    let user = match users::table
        .filter(users::email.eq(&request.email))
        .first::<User>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(user)) => user,
        Ok(None) => return StatusCode::OK, // Always return OK for security
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Only send reset email if user exists and email is verified
    if !user.email_verified.unwrap_or(false) {
        return StatusCode::OK; // Always return OK for security
    }

    let now = Utc::now();
    let password_reset_token = Uuid::new_v4();

    // Update user with password reset token
    let result = diesel::update(users::table.filter(users::id.eq(user.id)))
        .set((
            users::password_reset_token.eq(password_reset_token),
            users::password_reset_token_issued_at.eq(now),
            users::updated_at.eq(now),
        ))
        .execute(&mut conn)
        .await;

    match result {
        Ok(_) => {
            send_password_reset_email(
                &client,
                &request.email,
                &config.base_url,
                &config.from_email,
                password_reset_token,
            );
            StatusCode::OK
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn send_password_reset_email(
    client: &Client,
    email: &str,
    base_url: &str,
    from_email: &str,
    token: Uuid,
) {
    let reset_link = format!("{}/reset-password?token={}", base_url, token);
    let subject = "Reset Your Password".to_string();
    let body = format!(
        "You requested a password reset. Click the link below to reset your password:\n\n{}\n\nThis link will expire in 1 hour. If you didn't request this reset, you can safely ignore this email.",
        reset_link
    );

    client.send(from_email.to_string(), email.to_string(), subject, body);
}
