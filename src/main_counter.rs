use iced::widget::{button, column, container, row, text, Column};
use iced::{Center, Element, Theme};
use iced::Length::Fill;

pub fn main() -> iced::Result {
    iced::application("A cool counter", Counter::update, Counter::view)
    .run()
}

#[derive(Default)]
struct Counter {
    value: i64,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }

    // fn view(&self) -> Column<Message> {
    fn view(&self) -> Element<Message> {
        container(
            column![
                button("Increment").on_press(Message::Increment),
                text(self.value).size(50),
                button("Decrement").on_press(Message::Decrement)
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