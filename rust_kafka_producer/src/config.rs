use config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server_port: u16,
    pub kafka_brokers: String,
    pub kafka_topic: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::with_prefix("APP"))
            .set_default("SERVER_PORT", 8080)?
            .set_default("KAFKA_BROKERS", "localhost:9092")?
            .set_default("KAFKA_TOPIC", "messages")?
            .build()?;
        cfg.try_deserialize()
    }
}