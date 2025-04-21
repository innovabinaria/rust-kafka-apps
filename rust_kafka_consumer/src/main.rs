mod config;
mod consumer;


use axum::{routing::get, serve, Router};
use config::Config;
use consumer::consume_messages;
use std::net::SocketAddr;
use tokio::{net::TcpListener, spawn};
use tracing::{info, error};
use tracing_subscriber::FmtSubscriber;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    dotenv().ok();
    info!("🚀 Iniciando el consumidor Kafka...");

    let cfg = Config::from_env().map_err(|e| anyhow::anyhow!("{}", e))?;

    info!("Kafka brokers: {}", cfg.kafka_brokers);
    info!("Kafka topic: {}", cfg.kafka_topic);
    info!("Server port: {}", cfg.server_port);

    let cfg_clone = cfg.clone();
    spawn(async move {
        if let Err(e) = consume_messages(cfg_clone).await {
            error!("❌ Error en el consumidor de Kafka: {}", e);
        }
    });

    let app = Router::new().route("/health", get(|| async { "OK" }));

    let addr = SocketAddr::from(([0, 0, 0, 0], cfg.server_port));
    info!("🌐 Servidor escuchando en http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    serve(listener, app).await?;

    Ok(())
}