use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let topic = "test";
    let key = "key";
    let value = "Hello kafka from rust";

    match producer.send(
        FutureRecord::to(topic)
                .payload(value)
                .key(key),
        Duration::from_secs(0),)
        .await {
            Ok(delivery) => println!("Sent: {:?}", delivery),   
            Err((e, _)) => println!("Error: {:?}", e)
        }
    
    sleep(Duration::from_secs(1)).await;
}
