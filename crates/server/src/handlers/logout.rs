use axum::http::StatusCode;
use axum::response::IntoResponse;
use tower_sessions::Session;

pub async fn logout(session: Session) -> impl IntoResponse {
    match session.delete().await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
