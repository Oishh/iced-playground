use std::vec;

use iced::mouse::Cursor;
use iced::widget::canvas::{Cache, Frame, Geometry, Path, Program, Stroke};
use iced::widget::{button, column, container, text, Canvas};
use iced::{Center, Color, Element, Length, Point, Rectangle, Renderer, Theme};
use iced::Length::Fill;
use plotly::{Candlestick, Plot};
use rand::Rng;

pub fn main() {
    // iced::application(CandlestickApp::title, CandlestickApp::update, CandlestickApp::view).run()
    let chart = simple_candlestick_chart(true);

    std::fs::create_dir_all("./out").unwrap();
    let name = "simple_candlestick_chart";
    let html = chart.to_inline_html(Some(name));

    std::fs::write(format!("./out/{}.html", name), html).unwrap();

    // chart
}

fn simple_candlestick_chart(show: bool) -> Plot {
    let x = vec![
        "2017-01-04",
        "2017-01-05",
        "2017-01-06",
        "2017-01-09",
        "2017-01-10",
        "2017-01-11",
        "2017-01-12",
        "2017-01-13",
        "2017-01-17",
        "2017-01-18",
        "2017-01-19",
        "2017-01-20",
        "2017-01-23",
        "2017-01-24",
        "2017-01-25",
        "2017-01-26",
        "2017-01-27",
        "2017-01-30",
        "2017-01-31",
        "2017-02-01",
        "2017-02-02",
        "2017-02-03",
        "2017-02-06",
        "2017-02-07",
        "2017-02-08",
        "2017-02-09",
        "2017-02-10",
        "2017-02-13",
        "2017-02-14",
        "2017-02-15",
    ];
    let open = vec![
        115.849998, 115.919998, 116.779999, 117.949997, 118.769997, 118.739998, 118.900002,
        119.110001, 118.339996, 120.0, 119.400002, 120.449997, 120.0, 119.550003, 120.419998,
        121.669998, 122.139999, 120.93, 121.150002, 127.029999, 127.980003, 128.309998, 129.130005,
        130.539993, 131.350006, 131.649994, 132.460007, 133.080002, 133.470001, 135.520004,
    ];
    let high = vec![
        116.510002, 116.860001, 118.160004, 119.43, 119.379997, 119.93, 119.300003, 119.620003,
        120.239998, 120.5, 120.089996, 120.449997, 120.809998, 120.099998, 122.099998, 122.440002,
        122.349998, 121.629997, 121.389999, 130.490005, 129.389999, 129.190002, 130.5, 132.089996,
        132.220001, 132.449997, 132.940002, 133.820007, 135.089996, 136.270004,
    ];
    let low = vec![
        115.75, 115.809998, 116.470001, 117.940002, 118.300003, 118.599998, 118.209999, 118.809998,
        118.220001, 119.709999, 119.370003, 119.730003, 119.769997, 119.5, 120.279999, 121.599998,
        121.599998, 120.660004, 120.620003, 127.010002, 127.779999, 128.160004, 128.899994,
        130.449997, 131.220001, 131.119995, 132.050003, 132.75, 133.25, 134.619995,
    ];
    let close = vec![
        116.019997, 116.610001, 117.910004, 118.989998, 119.110001, 119.75, 119.25, 119.040001,
        120.0, 119.989998, 119.779999, 120.0, 120.080002, 119.970001, 121.879997, 121.940002,
        121.949997, 121.629997, 121.349998, 128.75, 128.529999, 129.080002, 130.289993, 131.529999,
        132.039993, 132.419998, 132.119995, 133.289993, 135.020004, 135.509995,
    ];

    let trace1 = Candlestick::new(x, open, high, low, close);

    let mut plot = Plot::new();
    plot.add_trace(trace1);

    if show {
        plot.show();
    }
    plot
}

// #[derive(Default)]
// struct CandlestickApp {
//     chart: CandlestickChart,
// }

// #[derive(Default)]
// struct Candlestick {
//     open: f32,
//     high: f32,
//     low: f32,
//     close: f32,
// }

// pub struct CandlestickChart {
//     data: Vec<Candlestick>,
//     cache: Cache, // Cache to store drawn geometry
// }

// #[derive(Debug, Clone, Copy)]
// enum Message {
//     UpdateData,
// }

// impl CandlestickApp {
//     fn update(&mut self, message: Message) {
//         // match message {
//         //     Message::Tick(local_time) => {
//         //         let now = local_time;

//         //         if now != self.now {
//         //             self.now = now;
//         //             self.clock.clear();
//         //         }
//         //     }
//         // }
//     }

//     fn view(&self) -> Element<Message> {
//         let canvas = Canvas::new(&self.chart)
//             .width(Length::Fixed(800.0)) // Set fixed width
//             .height(Length::Fixed(600.0)); // Set fixed height
    
//         let container = container(canvas)
//             .padding(20)
//             .center_x(Length::Fill)
//             .center_y(Length::Fill); // Center the chart
    
//         container.into()
//     }
    

//     // fn view(&self) -> Element<Message> {
//     //     let canvas = Canvas::new(&self.chart)
//     //     .width(Length::Fill) // Fill the available space
//     //     .height(Length::Fill); // Maintain aspect ratio

//     //     let container = container(canvas)
//     //         .padding(20)
//     //         .width(Length::Fill)
//     //         .height(Length::Fill); // Ensure it properly stretches

//     //     container.into()
//     // }

//     fn title(&self) -> String {
//         String::from("Candlestick Chart - Iced")
//     }
// }

// // impl Default for CandlestickChart {
// //     fn default() -> Self {
// //         Self {
// //             data: vec![
// //                 Candlestick { open: -500.0, high: 1200.0, low: -1500.0, close: 800.0 },
// //                 Candlestick { open: 800.0, high: 1800.0, low: 300.0, close: 1500.0 },
// //                 Candlestick { open: 1500.0, high: 2500.0, low: 1000.0, close: -500.0 },
// //                 Candlestick { open: -500.0, high: 700.0, low: -2000.0, close: -1000.0 },
// //                 Candlestick { open: -1000.0, high: 1200.0, low: -1500.0, close: 500.0 },
// //                 Candlestick { open: 500.0, high: 3000.0, low: 200.0, close: 2500.0 },
// //                 Candlestick { open: 2500.0, high: 5000.0, low: 2000.0, close: 4500.0 },
// //                 Candlestick { open: 4500.0, high: 7000.0, low: 4000.0, close: 6800.0 },
// //                 Candlestick { open: 6800.0, high: 9000.0, low: 6000.0, close: 7500.0 },
// //                 Candlestick { open: 7500.0, high: 11000.0, low: 5000.0, close: 10500.0 },
// //             ],
// //             cache: Cache::default(),
// //         }
// //     }
// // }

// impl Default for CandlestickChart {
//     fn default() -> Self {
//         let mut rng = rand::rng();
//         let num_candlesticks = 100; // Adjust this value as needed

//         let mut data = Vec::with_capacity(num_candlesticks);
//         for _ in 0..num_candlesticks {
//             let low = rng.random_range(0.0..1000.0);
//             let high = rng.random_range(low..(low + 1000.0));
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

// // impl Default for CandlestickChart {
// //     fn default() -> Self {
// //         let mut rng = rand::rng();
// //         let num_candlesticks = 500; // Adjust as needed
// //         let mut data = Vec::with_capacity(num_candlesticks);

// //         let mut price = 1000.0; // Starting price
// //         let trend_change = num_candlesticks / 2; // Midway point to change trend

// //         for i in 0..num_candlesticks {
// //             let trend_factor = if i < trend_change {
// //                 -rng.random_range(1.0..10.0_f32) // Downtrend: small negative changes
// //             } else {
// //                 rng.random_range(1.0..10.0_f32) // Uptrend: small positive changes
// //             };

// //             price = (price + trend_factor).max(100.0_f32); // Keep price above 100 to avoid unrealistic drops

// //             let low = price - rng.random_range(10.0..50.0_f32);
// //             let high = price + rng.random_range(10.0..50.0_f32);
// //             let open = rng.random_range(low..high);
// //             let close = rng.random_range(low..high);

// //             data.push(Candlestick { open, high, low, close });
// //         }

// //         Self {
// //             data,
// //             cache: Cache::default(),
// //         }
// //     }
// // }

// impl<Message> Program<Message> for CandlestickChart {
//     type State = ();

//     fn draw(
//         &self,
//         _state: &Self::State,
//         renderer: &Renderer,
//         theme: &Theme,
//         bounds: Rectangle,
//         _cursor: Cursor,
//     ) -> Vec<Geometry> {
//             let chart = self.cache.draw(renderer, bounds.size(), |frame| {
//             let width = bounds.width;
//             let height = bounds.height;
//             let candle_width = (width /  self.data.len() as f32);

//             for (i, candle) in self.data.iter().enumerate() {
//                 let x = i as f32 * candle_width + candle_width / 2.0;
//                 let y_high = height - candle.high;
//                 let y_low = height - candle.low;
//                 let y_open = height - candle.open;
//                 let y_close = height - candle.close;

//                 let color = if candle.close >= candle.open {
//                     Color::from_rgb(0.0, 1.0, 0.0) // Green for bullish
//                 } else {
//                     Color::from_rgb(1.0, 0.0, 0.0) // Red for bearish
//                 };

//                 let wick_path = Path::line(Point::new(x, y_high), Point::new(x, y_low));
//                 frame.stroke(
//                     &wick_path,
//                     Stroke {
//                         width: 1.0,
//                         // color,
//                         ..Default::default()
//                     },
//                 );

//                 let body_height = (y_close - y_open).abs().max(1.0); // Ensure a minimum height
//                 let body_rect = Path::rectangle(Point::new(x - candle_width / 4.0, y_open), iced::Size::new(candle_width / 2.0, body_height));

//                 frame.fill(&body_rect, color);

//             }
//         });

//         vec![chart]
//     }
// }