use crate::config::Config;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::Message;
use rdkafka::ClientConfig;
use tracing::{error, info};

pub async fn consume_messages(cfg: Config) -> anyhow::Result<()> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &cfg.kafka_brokers)
        .set("group.id", "rust-consumer-group")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest") 
        .create()?;

    consumer.subscribe(&[&cfg.kafka_topic])?;

    info!("Subscribed to topic: {}", cfg.kafka_topic);

    loop {
        match consumer.recv().await {
            Ok(msg) => {
                if let Some(payload) = msg.payload_view::<str>() {
                    match payload {
                        Ok(text) => info!("Received: {}", text),
                        Err(e) => error!("Error decoding message: {:?}", e),
                    }
                }
            }
            Err(e) => error!("Kafka error: {}", e),
        }
    }
}
