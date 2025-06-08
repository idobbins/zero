use crate::config::{BaseUrl, FromEmail};
use crate::email::localsmtp::Client;
use crate::schema::users;
use crate::types::user::User;
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

pub async fn resend_verification(
    Extension(db): Extension<DbPool>,
    Extension(client): Extension<Client>,
    Extension(base_url): Extension<BaseUrl>,
    Extension(from_email): Extension<FromEmail>,
    Form(request): Form<Request>,
) -> StatusCode {
    let mut conn = match db.get().await {
        Ok(conn) => conn,
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR,
    };

    // Check if user exists
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

    // Only send verification email if email is not yet verified
    if user.email_verified.unwrap_or(false) {
        return StatusCode::OK; // Email already verified
    }

    let now = Utc::now();
    let email_validation_token = Uuid::new_v4();

    // Update user with new email validation token
    let result = diesel::update(users::table.filter(users::id.eq(user.id)))
        .set((
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
                &request.email,
                &base_url.0,
                &from_email.0,
                email_validation_token,
            );
            StatusCode::OK
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

fn send_verification_email(
    client: &Client,
    email: &str,
    base_url: &str,
    from_email: &str,
    token: Uuid,
) {
    let verification_link = format!("{}/verify-email?token={}", base_url, token);
    let subject = "Verify Your Email Address".to_string();
    let body = format!(
        "Please verify your email address by clicking the link below:\n\n{}\n\nIf you didn't create an account, you can safely ignore this email.",
        verification_link
    );

    client.send(from_email.to_string(), email.to_string(), subject, body);
}
