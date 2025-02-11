use std::sync::{Arc, Mutex};

use iced::widget::{button, column, container, text, Button, Column, Container, Text};
use iced::{alignment, Center, Color, Element, Length};
use iced::Length::Fill;
mod nats;

pub fn main() -> iced::Result {
    iced::application("NATS Message", Nats::update, Nats::view)
    .run()
}

#[derive(Default)]
struct Nats {
    message: Arc<Mutex<Vec<String>>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OnMessage,
}

impl Nats {
    fn update(&mut self, message: Message) {
        match message {
            Message::OnMessage => {
                let message_clone = Arc::clone(&self.message);
                std::thread::spawn(move || {
                    // Ideally, use an existing runtime if possible
                    let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
                    
                    match runtime.block_on(nats::nats::get_nats_message()) {
                        Ok(messages) => {
                            let mut messages_lock = message_clone.lock().unwrap();
                            *messages_lock = messages;
                        }
                        Err(e) => eprintln!("Error fetching NATS message: {:?}", e),
                    }
                });
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let message = self.message.lock().unwrap();
    
        // Join the Vec<String> into a single String.
        let message_text = message.iter().cloned().collect::<Vec<_>>()
            .join(" ");  // Adjust the separator as needed, here it's a space.
        container(
            column![
                button("Call Nats").on_press(Message::OnMessage),
                text(message_text).size(50),
            ]
            .spacing(10)
        )
        .padding(10)
        .center_x(Fill)
        .center_y(Fill)
        .align_x(Center)
        .into()
    }
}