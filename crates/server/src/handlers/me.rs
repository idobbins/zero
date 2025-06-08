use crate::schema::users;
use crate::types::user::User;
use axum::http::StatusCode;
use axum::{Extension, Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use serde::Serialize;
use tower_sessions::Session;

type DbPool = diesel_async::pooled_connection::deadpool::Pool<diesel_async::AsyncPgConnection>;

#[derive(Serialize, Debug, Clone)]
pub struct Response {
    pub email: String,
    pub email_verified: bool,
}

pub async fn me(
    Extension(db): Extension<DbPool>,
    session: Session,
) -> Result<Json<Response>, StatusCode> {
    // Check if user_id exists; return UNAUTHORIZED if not
    let user_id = match session.get::<uuid::Uuid>("user_id").await {
        Ok(Some(id)) => id,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let mut conn = match db.get().await {
        Ok(conn) => conn,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Fetch user from database
    let user = match users::table
        .filter(users::id.eq(user_id))
        .first::<User>(&mut conn)
        .await
        .optional()
    {
        Ok(Some(user)) => user,
        Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // If successful, construct and return the Me struct as JSON
    let me = Response {
        email: user.email,
        email_verified: user.email_verified.unwrap_or(false),
    };

    Ok(Json(me))
}
