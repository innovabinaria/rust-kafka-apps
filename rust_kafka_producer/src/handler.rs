use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use std::sync::Arc;
use tracing::info;

use crate::error::AppError;
use crate::kafka_producer::KafkaProducer;
use crate::models::Message;

pub fn create_routes(producer: KafkaProducer) -> Router {
    let shared_producer = Arc::new(producer);
    Router::new()
        .route("/send", post(send_message))
        .with_state(shared_producer)
}

async fn send_message(
    State(producer): State<Arc<KafkaProducer>>,
    Json(message): Json<Message>,
) -> Result<(StatusCode, String), AppError> {
    let payload = serde_json::to_string(&message)?;
    producer
        .send_message(&message.id, &payload)
        .await?;
    info!("Processed message with id: {}", message.id);
    Ok((StatusCode::OK, format!("Message {} sent", message.id)))
}