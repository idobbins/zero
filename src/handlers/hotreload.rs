use axum::{
    http::StatusCode,
    response::{Sse, sse::Event},
    Json,
};
use serde_json::json;
use std::convert::Infallible;
use std::sync::{Arc, Mutex, OnceLock};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::UnboundedReceiverStream, StreamExt};
use zero_macros::register_route;

// Use OnceLock for thread-safe initialization
static SHUTDOWN_SENDERS: OnceLock<Arc<Mutex<Vec<mpsc::UnboundedSender<String>>>>> = OnceLock::new();

fn get_shutdown_senders() -> &'static Arc<Mutex<Vec<mpsc::UnboundedSender<String>>>> {
    SHUTDOWN_SENDERS.get_or_init(|| Arc::new(Mutex::new(Vec::new())))
}

pub fn broadcast_shutdown() {
    let senders = get_shutdown_senders();
    let senders = senders.lock().unwrap();
    for sender in senders.iter() {
        let _ = sender.send("shutdown".to_string());
    }
}

#[register_route("/events", "GET")]
pub async fn events() -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    let (tx, rx) = mpsc::unbounded_channel::<String>();
    
    // Add this sender to our global list
    {
        let senders = get_shutdown_senders();
        let mut senders = senders.lock().unwrap();
        senders.push(tx);
    }
    
    let stream = UnboundedReceiverStream::new(rx)
        .map(|data| Ok(Event::default().data(data)));

    Sse::new(stream)
}

#[register_route("/health", "GET")]
pub async fn health() -> Result<Json<serde_json::Value>, StatusCode> {
    Ok(Json(json!({"status": "ok"})))
}
