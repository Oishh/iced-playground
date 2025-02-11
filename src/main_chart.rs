use std::vec;

use iced::mouse::Cursor;
use iced::widget::canvas::{Cache, Frame, Geometry, Path, Program, Stroke};
use iced::widget::{button, column, container, text, Canvas};
use iced::{Center, Color, Element, Length, Point, Rectangle, Renderer, Theme};
use iced::Length::Fill;
use rand::Rng;

pub fn main() -> iced::Result {
    iced::application(CandlestickApp::title, CandlestickApp::update, CandlestickApp::view).run()
}

#[derive(Default)]
struct CandlestickApp {
    chart: CandlestickChart,
}

#[derive(Default)]
struct Candlestick {
    open: f32,
    high: f32,
    low: f32,
    close: f32,
}

pub struct CandlestickChart {
    data: Vec<Candlestick>,
    cache: Cache, // Cache to store drawn geometry
}

#[derive(Debug, Clone, Copy)]
enum Message {
    UpdateData,
}

impl CandlestickApp {
    fn update(&mut self, message: Message) {
        // match message {
        //     Message::Tick(local_time) => {
        //         let now = local_time;

        //         if now != self.now {
        //             self.now = now;
        //             self.clock.clear();
        //         }
        //     }
        // }
    }

    fn view(&self) -> Element<Message> {
        let canvas = Canvas::new(&self.chart)
            .width(Length::Fixed(800.0)) // Set fixed width
            .height(Length::Fixed(600.0)); // Set fixed height
    
        let container = container(canvas)
            .padding(20)
            .center_x(Length::Fill)
            .center_y(Length::Fill); // Center the chart
    
        container.into()
    }
    

    // fn view(&self) -> Element<Message> {
    //     let canvas = Canvas::new(&self.chart)
    //     .width(Length::Fill) // Fill the available space
    //     .height(Length::Fill); // Maintain aspect ratio

    //     let container = container(canvas)
    //         .padding(20)
    //         .width(Length::Fill)
    //         .height(Length::Fill); // Ensure it properly stretches

    //     container.into()
    // }

    fn title(&self) -> String {
        String::from("Candlestick Chart - Iced")
    }
}

// impl Default for CandlestickChart {
//     fn default() -> Self {
//         Self {
//             data: vec![
//                 Candlestick { open: -500.0, high: 1200.0, low: -1500.0, close: 800.0 },
//                 Candlestick { open: 800.0, high: 1800.0, low: 300.0, close: 1500.0 },
//                 Candlestick { open: 1500.0, high: 2500.0, low: 1000.0, close: -500.0 },
//                 Candlestick { open: -500.0, high: 700.0, low: -2000.0, close: -1000.0 },
//                 Candlestick { open: -1000.0, high: 1200.0, low: -1500.0, close: 500.0 },
//                 Candlestick { open: 500.0, high: 3000.0, low: 200.0, close: 2500.0 },
//                 Candlestick { open: 2500.0, high: 5000.0, low: 2000.0, close: 4500.0 },
//                 Candlestick { open: 4500.0, high: 7000.0, low: 4000.0, close: 6800.0 },
//                 Candlestick { open: 6800.0, high: 9000.0, low: 6000.0, close: 7500.0 },
//                 Candlestick { open: 7500.0, high: 11000.0, low: 5000.0, close: 10500.0 },
//             ],
//             cache: Cache::default(),
//         }
//     }
// }

impl Default for CandlestickChart {
    fn default() -> Self {
        let mut rng = rand::rng();
        let num_candlesticks = 100; // Adjust this value as needed

        let mut data = Vec::with_capacity(num_candlesticks);
        for _ in 0..num_candlesticks {
            let low = rng.random_range(0.0..1000.0);
            let high = rng.random_range(low..(low + 1000.0));
            let open = rng.random_range(low..high);
            let close = rng.random_range(low..high);

            data.push(Candlestick { open, high, low, close });
        }

        Self {
            data,
            cache: Cache::default(),
        }
    }
}

// impl Default for CandlestickChart {
//     fn default() -> Self {
//         let mut rng = rand::rng();
//         let num_candlesticks = 500; // Adjust as needed
//         let mut data = Vec::with_capacity(num_candlesticks);

//         let mut price = 1000.0; // Starting price
//         let trend_change = num_candlesticks / 2; // Midway point to change trend

//         for i in 0..num_candlesticks {
//             let trend_factor = if i < trend_change {
//                 -rng.random_range(1.0..10.0_f32) // Downtrend: small negative changes
//             } else {
//                 rng.random_range(1.0..10.0_f32) // Uptrend: small positive changes
//             };

//             price = (price + trend_factor).max(100.0_f32); // Keep price above 100 to avoid unrealistic drops

//             let low = price - rng.random_range(10.0..50.0_f32);
//             let high = price + rng.random_range(10.0..50.0_f32);
//             let open = rng.random_range(low..high);
//             let close = rng.random_range(low..high);

//             data.push(Candlestick { open, high, low, close });
//         }

//         Self {
//             data,
//             cache: Cache::default(),
//         }
//     }
// }

impl<Message> Program<Message> for CandlestickChart {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
            let chart = self.cache.draw(renderer, bounds.size(), |frame| {
            let width = bounds.width;
            let height = bounds.height;
            let candle_width = (width /  self.data.len() as f32);

            for (i, candle) in self.data.iter().enumerate() {
                let x = i as f32 * candle_width + candle_width / 2.0;
                let y_high = height - candle.high;
                let y_low = height - candle.low;
                let y_open = height - candle.open;
                let y_close = height - candle.close;

                let color = if candle.close >= candle.open {
                    Color::from_rgb(0.0, 1.0, 0.0) // Green for bullish
                } else {
                    Color::from_rgb(1.0, 0.0, 0.0) // Red for bearish
                };

                let wick_path = Path::line(Point::new(x, y_high), Point::new(x, y_low));
                frame.stroke(
                    &wick_path,
                    Stroke {
                        width: 1.0,
                        // color,
                        ..Default::default()
                    },
                );

                let body_height = (y_close - y_open).abs().max(1.0); // Ensure a minimum height
                let body_rect = Path::rectangle(Point::new(x - candle_width / 4.0, y_open), iced::Size::new(candle_width / 2.0, body_height));

                frame.fill(&body_rect, color);

            }
        });

        vec![chart]
    }
}