use anyhow::Result;
use askama::Template;
use axum::{
    http::StatusCode,
    response::Html,
};
use zero_macros::register_route;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    app_name: String,
}

#[register_route("/", "GET")]
pub async fn index() -> Result<Html<String>, StatusCode> {
    let template = IndexTemplate {
        app_name: "Zero".to_string(),
    };
    
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
