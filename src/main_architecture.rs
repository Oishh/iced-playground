use iced::{widget::{button, column, container, row, text}, Element, Length, Task};

pub fn main() -> iced::Result {
    iced::application("Pane Grid - Iced", Main::update, Main::view)
        .run()
}

pub struct Main {
    screen: Screen
}

impl Main {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(screen) => {
                self.screen = screen;
                Task::none()
            }
            Message::Contacts(msg) => {
                if let Screen::Contacts(contacts) = &mut self.screen {
                    contacts.update(msg);
                }
                Task::none()
            }
            Message::Conversation(msg) => {
                if let Screen::Conversation(conversation) = &mut self.screen {
                    conversation.update(msg);
                }
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<Message> {
        let sidebar = container(
            column![
                button("Items")
                    .on_press(Message::Navigate(Screen::Contacts(contacts::Contacts::default())))
                    .width(Length::Fill)
                    .style(button::secondary),
                button("Item Groups")
                    .on_press(Message::Navigate(Screen::Conversation(conversation::Conversation::default())))
                    .width(Length::Fill)
                    .style(button::secondary),
            ]
            .spacing(5)
            .padding(10)
        )
        .width(Length::Fixed(200.0))
        .height(Length::Fill)
        .style(container::rounded_box);

        let content = match &self.screen {
            Screen::Contacts(contacts) => Element::from(
                contacts.view().map(|_| Message::Navigate(self.screen.clone())),
            ),
            Screen::Conversation(conversation) => Element::from(
                conversation.view().map(|_| Message::Navigate(self.screen.clone())),
            ),
        };

        let app_view = row![
            sidebar,
            container(content)
                .width(Length::Fill)
                .padding(20),
        ];
        
        app_view.into()
    }
}

impl Default for Main {
    fn default() -> Self {
        Self {
            screen: Screen::default()
        }
    }
}

#[derive(Clone, Debug)]
pub enum Screen {
    Contacts(contacts::Contacts),
    Conversation(conversation::Conversation),
}

impl Default for Screen {
    fn default() -> Self {
        Self::Contacts(contacts::Contacts::default())
    }
}

#[derive(Debug, Clone)]
enum Message {
   Contacts(contacts::Message),
   Conversation(conversation::Message),
   Navigate(Screen),
}



mod contacts {
    use iced::{widget::row, Element};

    #[derive(Clone, Debug)]
    pub struct Contacts;

    #[derive(Debug, Clone)]
    pub enum Message {
        ContactMessage
    }

    impl Default for Contacts {
        fn default() -> Self {
            Self
        }
    }

    impl Contacts {
        pub fn update(&self, _message: Message) {

        }
        pub fn view(&self) -> Element<Message> {
            row!["Contacts Message!"].into()
        }
    }
}

mod conversation {
    use iced::{widget::row, Element};

    #[derive(Clone, Debug)]
    pub struct Conversation;

    #[derive(Debug, Clone)]
    pub enum Message {
        ConversationMessage
    }

    impl Default for Conversation {
        fn default() -> Self {
            Self
        }
    }

    impl Conversation {
        pub fn update(&self, _message: Message) {

        }
        pub fn view(&self) -> Element<Message> {
            row!["Conversation Message!"].into()
        }
    }
}