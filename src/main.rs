use anyhow::Result;
use tokio::{
    net::TcpListener,
    select,
};

mod handlers;
use handlers::hotreload;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = format!("{}:{}", "127.0.0.1", "3000");
    let listener = TcpListener::bind(&addr).await?;

    let router = handlers::build_router();

    select! {
        _ = axum::serve(listener, router) => {}
        _ = tokio::signal::ctrl_c() => {
            println!("Shutdown signal received, notifying clients...");
            hotreload::broadcast_shutdown();
            // Give clients a moment to receive the shutdown event
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    Ok(())
}
