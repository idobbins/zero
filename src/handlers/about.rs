use askama::Template;
use axum::{
    http::StatusCode,
    response::Html,
};
use zero_macros::register_route;

#[derive(Template)]
#[template(path = "about.html")]
struct AboutTemplate {
    app_name: String,
    version: String,
}

#[register_route("/about", "GET")]
pub async fn about() -> Result<Html<String>, StatusCode> {
    let template = AboutTemplate {
        app_name: "Zero".to_string(),
        version: "0.1.0".to_string(),
    };
    
    match template.render() {
        Ok(html) => Ok(Html(html)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
