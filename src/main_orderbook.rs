use iced::{mouse::Button, widget::{button, canvas::Cache, Column, Container, Row, Text}, Element, Length};



#[derive(Debug, Clone)]
enum Message {
    Refresh,
}

#[derive(Debug, Clone, Default)]
struct Order {
    price: f64,
    volume: f64,
}

#[derive(Debug)]
struct OrderBook {
    bids: Vec<Order>,
    asks: Vec<Order>,
    cache: Cache,
}

#[derive(Debug, Default)]
struct OrderBookApp {
    orderbook: OrderBook,
}

impl OrderBookApp {
    fn update(&mut self, message: Message) {
        match message {
            Message::Refresh => {
                // For demonstration, we reverse the orders to simulate a change.
                self.orderbook.bids.reverse();
                self.orderbook.asks.reverse();
            }
        }
        // Command::none()
    }

    fn view(&self) -> Element<Message> {
        // let canvas = Canvas::new(&self.orderbook)
        //     .width(Length::Fixed(800.0)) // Set fixed width
        //     .height(Length::Fixed(600.0)); // Set fixed height
    
        // let container = container(canvas)
        //     .padding(20)
        //     .center_x(Length::Fill)
        //     .center_y(Length::Fill); // Center the chart
    
        // container.into()


        let bids_header = Row::new()
        .spacing(20)
        .push(Text::new("Price").size(20))
        .push(Text::new("Volume").size(20));

        let bids_column = Column::new()
            .spacing(10)
            .push(Text::new("Bids").size(24))
            .push(bids_header)
            .push(Column::with_children(
                self.orderbook
                    .bids
                    .iter()
                    .map(|order| {
                        Row::new()
                            .spacing(20)
                            .push(Text::new(format!("{:.2}", order.price)))
                            .push(Text::new(format!("{:.2}", order.volume)))
                            .into()
                    }).collect::<Vec<_>>(),
            ));

        // --- Build the "Asks" column ---
        let asks_header = Row::new()
            .spacing(20)
            .push(Text::new("Price").size(20))
            .push(Text::new("Volume").size(20));

        let asks_column = Column::new()
            .spacing(10)
            .push(Text::new("Asks").size(24))
            .push(asks_header)
            .push(Column::with_children(
                self.orderbook
                    .asks
                    .iter()
                    .map(|order| {
                        Row::new()
                            .spacing(20)
                            .push(Text::new(format!("{:.2}", order.price)))
                            .push(Text::new(format!("{:.2}", order.volume)))
                            .into()
                    })
                    .collect::<Vec<_>>(),
            ));

        // --- Combine both columns into one view ---
        let orderbook_view = Row::new()
            .spacing(50)
            .push(bids_column)
            .push(asks_column);

        // --- Add a refresh button ---
        let refresh_button = button("Refresh").on_press(Message::Refresh);

        // --- Build the overall layout ---
        let content = Column::new()
            .spacing(20)
            .padding(20)
            .push(orderbook_view)
            .push(refresh_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }

    fn title(&self) -> String {
        String::from("Order Book GUI")
    }
}

impl Default for OrderBook {
    fn default() -> Self {
        // let orderbook = OrderBook {
            let bids = vec![
                Order {
                    price: 99.5,
                    volume: 5.0,
                },
                Order {
                    price: 99.0,
                    volume: 10.0,
                },
                Order {
                    price: 98.5,
                    volume: 15.0,
                },
            ];
            let asks = vec![
                Order {
                    price: 100.5,
                    volume: 4.0,
                },
                Order {
                    price: 101.0,
                    volume: 8.0,
                },
                Order {
                    price: 101.5,
                    volume: 12.0,
                },
            ];
        // };

        Self {
            bids,
            asks,
            cache: Cache::default(),
        }
    }
}

fn main() -> iced::Result {
    iced::application(OrderBookApp::title, OrderBookApp::update, OrderBookApp::view).run()
    // OrderBookApp::run(Settings::default())
}