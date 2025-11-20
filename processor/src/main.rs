use std::time::Duration;
use axum::http::StatusCode;
use log::info;
use rdkafka::config::ClientConfig;
use rdkafka::message::{Header, OwnedHeaders};
use rdkafka::producer::{FutureProducer, FutureRecord};
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init(); 

    let app = Router::new()
        .route("/recieve-test-messages", get(recieve_test_messages));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

async fn recieve_test_messages() -> StatusCode {
    let brokers = std::env::var("KAFKA_BROKERS").unwrap_or("localhost:9092".to_string());
    let topic = "test";
    let group_id = "1";

    info!("Connecting to {}", brokers);

    if let Err(e) = consume(&brokers, group_id, &[topic], None).await {
        eprintln!("Kafka error: {:?}", e);
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    StatusCode::OK
}

async fn consume(
    brokers: &str,
    group_id: &str,
    topics: &[&str],
    assignor: Option<&String>,
) -> anyhow::Result<()> {
    Ok(()) //TODO: Implement
}