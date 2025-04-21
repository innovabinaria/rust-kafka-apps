use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;
use tracing::{error, info};

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: &str, topic: &str) -> Result<Self, rdkafka::error::KafkaError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;
        Ok(Self {
            producer,
            topic: topic.to_string(),
        })
    }

    pub async fn send_message(&self, key: &str, payload: &str) -> Result<(), rdkafka::error::KafkaError> {
        let record = FutureRecord::to(&self.topic)
            .key(key)
            .payload(payload);
        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok(_) => {
                info!("Message sent to topic {} with key {}", self.topic, key);
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send message: {}", e);
                Err(e)
            }
        }
    }
}