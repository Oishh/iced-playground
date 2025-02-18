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

    fn title(&self) -> String {
        String::from("Candlestick Chart - Iced")
    }
}

impl Default for CandlestickChart {
    fn default() -> Self {
        // let mut rng = rand::rng();
        // let num_candlesticks = 100; // Adjust this value as needed

        // let mut data = Vec::with_capacity(num_candlesticks);
        // for _ in 0..num_candlesticks {
        //     let low = rng.random_range(0.0..1000.0);
        //     let high = rng.random_range(low..(low + 1000.0));
        //     let open = rng.random_range(low..high);
        //     let close = rng.random_range(low..high);

        //     data.push(Candlestick { open, high, low, close });
        // }
        let data = vec![
            Candlestick {
                open: 100.0,
                high: 105.0,
                low: 95.0,
                close: 102.0,
            },
            Candlestick {
                open: 102.0,
                high: 108.0,
                low: 101.0,
                close: 107.0,
            },
            Candlestick {
                open: 107.0,
                high: 110.0,
                low: 104.0,
                close: 105.0,
            },
            Candlestick {
                open: 105.0,
                high: 106.0,
                low: 100.0,
                close: 101.0,
            },
            Candlestick {
                open: 101.0,
                high: 103.0,
                low: 97.0,
                close: 99.0,
            },
            Candlestick {
                open: 99.0,
                high: 101.0,
                low: 95.0,
                close: 100.0,
            },
            Candlestick {
                open: 100.0,
                high: 104.0,
                low: 98.0,
                close: 103.0,
            },
            Candlestick {
                open: 103.0,
                high: 107.0,
                low: 102.0,
                close: 106.0,
            },
        ];

        Self {
            data,
            cache: Cache::default(),
        }
    }
}

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
        //     let chart = self.cache.draw(renderer, bounds.size(), |frame| {
        //     let width = bounds.width;
        //     let height = bounds.height;
        //     let candle_width = (width /  self.data.len() as f32);

        //     for (i, candle) in self.data.iter().enumerate() {
        //         let x = i as f32 * candle_width + candle_width / 2.0;
        //         let y_high = height - candle.high;
        //         let y_low = height - candle.low;
        //         let y_open = height - candle.open;
        //         let y_close = height - candle.close;

        //         let color = if candle.close >= candle.open {
        //             Color::from_rgb(0.0, 1.0, 0.0) // Green for bullish
        //         } else {
        //             Color::from_rgb(1.0, 0.0, 0.0) // Red for bearish
        //         };

        //         let wick_path = Path::line(Point::new(x, y_high), Point::new(x, y_low));
        //         frame.stroke(
        //             &wick_path,
        //             Stroke {
        //                 width: 1.0,
        //                 // color,
        //                 ..Default::default()
        //             },
        //         );

        //         let body_height = (y_close - y_open).abs().max(1.0); // Ensure a minimum height
        //         let body_rect = Path::rectangle(Point::new(x - candle_width / 4.0, y_open), iced::Size::new(candle_width / 2.0, body_height));

        //         frame.fill(&body_rect, color);

        //     }
        // });
                    // Use our cache to avoid redrawing if nothing has changed.
        let content = self.cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            // Set margins inside the canvas.
            let margin = 20.0;
            let width = bounds.width - 2.0 * margin;
            let height = bounds.height - 2.0 * margin;

        let num_candles = self.data.len();
        if num_candles == 0 {
            return;
        }

        // Determine the min and max prices among all candlesticks.
        let mut min_price = self.data[0].low;
        let mut max_price = self.data[0].high;
        for candle in &self.data {
            if candle.low < min_price {
                min_price = candle.low;
            }
            if candle.high > max_price {
                max_price = candle.high;
            }
        }
        // Add some padding.
        let price_padding = (max_price - min_price) * 0.1;
        min_price -= price_padding;
        max_price += price_padding;

        let price_range = max_price - min_price;

        // Calculate horizontal spacing.
        let x_step = width / num_candles as f32;
        let candle_width = x_step * 0.6; // Width for the candle body.

        // A helper closure: convert a price to a y coordinate (inverted so higher prices are toward the top)
        let price_to_y = |price: f32| {
            margin + height - ((price - min_price) / price_range * height)
        };

        // Draw each candlestick.
        for (i, candle) in self.data.iter().enumerate() {
            // Center position for this candle.
            let x_center = margin + x_step * (i as f32 + 0.5);

            let y_high = price_to_y(candle.high);
            let y_low = price_to_y(candle.low);
            let y_open = price_to_y(candle.open);
            let y_close = price_to_y(candle.close);

            // Draw the wick as a vertical line.
            let wick = Path::line(Point::new(x_center, y_high), Point::new(x_center, y_low));
            frame.stroke(
                &wick,
                Stroke {
                    width: 2.0,
                    style: iced::widget::canvas::Style::Solid(Color::BLACK),
                    ..Stroke::default()
                },
            );

            // Determine the color of the candle body.
            let body_color = if candle.close >= candle.open {
                Color::from_rgb(0.0, 0.8, 0.0) // Bullish (green)
            } else {
                Color::from_rgb(0.8, 0.0, 0.0) // Bearish (red)
            };

            // Compute top and bottom of the candle body.
            let body_top = y_open.min(y_close);
            let body_bottom = y_open.max(y_close);

            // Draw the candle body as a rectangle.
            let rect = Path::rectangle(
                Point::new(x_center - candle_width / 2.0, body_top),
                iced::Size::new(candle_width, body_bottom - body_top),
            );
            frame.fill(&rect, body_color);
            frame.stroke(
                &rect,
                Stroke {
                    width: 1.0,
                    style: iced::widget::canvas::Style::Solid(Color::BLACK),
                    ..Stroke::default()
                },
            );
        }
    });

        vec![content]
    }
}