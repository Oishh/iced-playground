use std::{env, str::from_utf8};
use iced::futures::channel::mpsc;
use async_nats::jetstream::{self, consumer::PullConsumer};
use iced::stream;
use futures::stream::{Stream, StreamExt};

use crate::Message;

pub fn connect() -> impl Stream<Item = Message> {
    stream::channel(100, |mut output| async move {
        loop {
            let nats_url = env::var("NATS_URL").unwrap_or_else(|_| "nats://43.206.231.234:4222".to_string());

            let client = async_nats::connect(nats_url).await.unwrap();

            // let mut vector = vec![];

            let jetstream = jetstream::new(client);

            let consumer: PullConsumer = jetstream.get_stream("mm_perp_summary_derive").await.unwrap()
            .create_consumer(jetstream::consumer::pull::Config {
                durable_name: Some("consumer".into()),
                ..Default::default()
            })
            .await.unwrap();

            let mut messages = consumer.messages().await.unwrap().take(1);

            match messages.next().await {
                Some(msg) => {
                    let payload = msg.as_ref().unwrap().payload.clone();
                    output.try_send(Message::NatsMessageReceived { payload: payload.clone().to_vec() });
                },
                None => {},
            }

            // Iterate over messages.
            // while let Some(message) = messages.next().await {
            //     let message = message.unwrap();
            //     vector.push(from_utf8(&message.payload).unwrap().to_string());
            //     message.ack().await.unwrap();
            // }
        }
    })
}