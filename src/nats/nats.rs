use std::{env, str::from_utf8};
use futures::StreamExt;

use async_nats::jetstream::{self, consumer::PullConsumer};

pub async fn get_nats_message() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://43.206.231.234:4222".to_string());

    let client = async_nats::connect(nats_url).await.unwrap();

    let mut vector = vec![];

    let jetstream = jetstream::new(client);

    let consumer: PullConsumer = jetstream.get_stream("mm_perp_summary_derive").await.unwrap()
    .create_consumer(jetstream::consumer::pull::Config {
        durable_name: Some("consumer".into()),
        ..Default::default()
    })
    .await.unwrap();

    let mut messages = consumer.messages().await.unwrap().take(1);

    // Iterate over messages.
    while let Some(message) = messages.next().await {
        let message = message.unwrap();
        vector.push(from_utf8(&message.payload).unwrap().to_string());
        message.ack().await.unwrap();
    }

    Ok(vector)
}