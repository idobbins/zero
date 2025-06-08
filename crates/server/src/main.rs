mod config;
mod email;
mod handlers;
mod schema;
mod types;

use crate::config::{BaseUrl, ClientUrl, FromEmail, MinPasswordLength};
use crate::email::localsmtp::Client;
use anyhow::Result;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::routing::{get, post};
use axum::{Extension, Router};
use clap::Parser;
use diesel_async::{
    AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager,
    pooled_connection::deadpool::Pool,
};
use diesel_async_migrations::{EmbeddedMigrations, embed_migrations};
use tokio::net::TcpListener;
use tokio::select;
use tower_http::cors::CorsLayer;
use tower_sessions::{MemoryStore, SessionManagerLayer};
use tracing::{Level, info};

type DbPool = Pool<AsyncPgConnection>;

#[derive(Parser)]
struct Config {
    #[clap(long, env = "ZERO_HOST", default_value = "127.0.0.1")]
    host: String,

    #[clap(long, env = "ZERO_PORT", default_value = "3000")]
    port: u16,

    #[clap(
        long,
        env = "ZERO_DATABASE_URL",
        default_value = "postgres://zero:password@localhost:5432/zero"
    )]
    database_url: String,

    #[clap(
        long,
        env = "ZERO_BASE_URL",
        default_value = "http://localhost:3000"
    )]
    base_url: String,

    #[clap(long, env = "ZERO_LOCAL_SMTP_HOST")]
    local_smtp_host: String,

    #[clap(long, env = "ZERO_LOCAL_SMTP_PORT")]
    local_smtp_port: u16,

    #[clap(
        long,
        env = "ZERO_FROM_EMAIL",
        default_value = "no-reply@jeddix.com"
    )]
    from_email: String,

    #[clap(
        long,
        env = "ZERO_CORS_ORIGIN",
        default_value = "http://localhost:5173"
    )]
    cors_origin: String,

    #[clap(long, env = "ZERO_MIN_PASSWORD_LENGTH", default_value = "12")]
    min_password_length: usize,

    #[clap(long, env = "ZERO_SESSION_SECURE", default_value = "false")]
    session_secure: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    dotenvy::dotenv()?;

    let config = Config::parse();

    let db_config =
        AsyncDieselConnectionManager::<AsyncPgConnection>::new(&config.database_url);
    let db = Pool::builder(db_config).build()?;
    info!("Connected to database");

    // Make sure we are migrated
    {
        const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
        let mut conn = db.get().await?;
        MIGRATIONS.run_pending_migrations(&mut conn).await?;
        info!("Migrations complete");
    }

    let email = Client::new(&config.local_smtp_host, config.local_smtp_port)?;

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(config.session_secure)
        .with_http_only(true);

    let origin = HeaderValue::from_str(&config.cors_origin)?;
    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_credentials(true);

    let router = Router::new()
        .route("/", get(async || StatusCode::OK))
        .route("/login", post(handlers::login))
        .route("/logout", post(handlers::logout))
        .route("/register", post(handlers::register))
        .route("/me", get(handlers::me))
        .route("/verify-email", get(handlers::verify_email))
        .route("/forgot-password", post(handlers::forgot_password))
        .route("/reset-password", post(handlers::reset_password))
        .route("/resend-verification", post(handlers::resend_verification))
        .route("/change-password", post(handlers::change_password))
        .layer(Extension(db))
        .layer(Extension(email))
        .layer(Extension(BaseUrl(config.base_url.clone())))
        .layer(Extension(ClientUrl(config.cors_origin.clone())))
        .layer(Extension(FromEmail(config.from_email.clone())))
        .layer(Extension(MinPasswordLength(config.min_password_length)))
        .layer(session_layer)
        .layer(cors);

    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    select! {
        _ = axum::serve(listener, router) => {}
        _ = tokio::signal::ctrl_c() => {}
    }

    info!("Shutting down");
    Ok(())
}