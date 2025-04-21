mod config;
mod error;
mod handler;
mod kafka_producer;
mod models;


use config::Config;
use handler::create_routes;
use kafka_producer::KafkaProducer;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;
use dotenv::dotenv;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Inicializar logging
    tracing_subscriber::fmt()
        .with_env_filter("info,kafka_service=debug")
        .init();

        // Cargar variables de entorno desde el archivo .env
    dotenv().ok();
    tracing::info!("Start the program");

    // Cargar configuración
    let config = Config::from_env()?;

    // ✅ Verificación por consola de los valores leídos
    tracing::info!("Kafka brokers: {}", config.kafka_brokers);
    tracing::info!("Kafka topic: {}", config.kafka_topic);
    tracing::info!("Server port: {}", config.server_port);

    // Inicializar productor de Kafka
    let producer = KafkaProducer::new(&config.kafka_brokers, &config.kafka_topic)?;

    // Crear rutas con el productor inyectado
    let app = create_routes(producer);

    // Configurar servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server_port));
    info!("Server starting on http://{}", addr);

     // Crear el listener TCP
     let listener = TcpListener::bind(addr).await?;

    // Iniciar servidor Axum
    axum::serve(listener, app).await?;

    Ok(())
}