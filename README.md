# Rust Kafka Projects

This repository contains two Rust projects:

## 1. Kafka Producer API

A REST API built with Axum that allows sending messages to a Kafka topic.

### Features:
- REST endpoint to publish messages.
- Kafka integration using `rdkafka`.
- Configuration via environment variables.
- Logging with `tracing`.

### How to Run:
1. Start Kafka and Zookeeper (e.g., using Docker).
2. Set the environment variables in a `.env` file:
```
APP_KAFKA_BROKERS=localhost:9092
APP_KAFKA_TOPIC=my-topic
APP_SERVER_PORT=3000
```
3. Run the service:
```bash
cargo run
```

## 2. Kafka Consumer Service

A background service that consumes messages from Kafka and prints them to the console.

### Features:
- Consumes from Kafka in real-time.
- Runs a basic HTTP server with a `/health` endpoint.
- Configuration via `.env`.

### How to Run:
1. Set the environment variables in a `.env` file:
```
APP_KAFKA_BROKERS=localhost:9092
APP_KAFKA_TOPIC=my-topic
APP_SERVER_PORT=3001
```
2. Run the service:
```bash
cargo run
```

---

## 🚀 Requirements

- [Rust](https://www.rust-lang.org/) 1.8 or higher
- [Docker](https://www.docker.com/) with Kafka running locally (port 9092)
- Kafka topic created: `my-topic`

---

## 🔧 Setup

1. Clone the repository:

```bash
git clone https://github.com/TU_USUARIO/rust-kafka-apps.git
cd rust-kafka-apps
```

### 📬 Send a Message

```bash
curl -X POST http://localhost:3000/send \
  -H "Content-Type: application/json" \
  -d '{"id":"123","content":"Hello from Rust!"}'
```

## 📚 Technologies Used

- [Rust](https://www.rust-lang.org/)
- [Axum](https://docs.rs/axum)
- [Tokio](https://tokio.rs/)
- [rdkafka](https://docs.rs/rdkafka) (Rust client for Apache Kafka)
- [dotenv](https://docs.rs/dotenv)

---

## ✨ Future Plans

- Añadir métricas Prometheus
- Soporte para múltiples topics y particiones
- Persistencia y procesamiento adicional

---


## 🧠 Author 
Developed by **Victor Aguayo Seminario**