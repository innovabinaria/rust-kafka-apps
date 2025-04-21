use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub kafka_brokers: String,
    pub kafka_topic: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn Error>> {
        Ok(Config {
            kafka_brokers: env::var("APP_KAFKA_BROKERS")?,
            kafka_topic: env::var("APP_KAFKA_TOPIC")?,
            server_port: env::var("APP_SERVER_PORT")?.parse()?,
        })
    }
}
