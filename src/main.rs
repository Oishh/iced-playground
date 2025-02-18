use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;
use std::time::Instant;

use iced::alignment::Horizontal::Right;
use iced::border;
use iced::border::rounded;
use iced::daemon::Appearance;
use iced::keyboard;
use iced::mouse;
use iced::time;
use iced::widget::pane_grid;
use iced::widget::pane_grid::Axis;
use iced::widget::responsive;
use iced::widget::PaneGrid;
use iced::widget::{
    button, canvas, center, checkbox, column, container,
    horizontal_space, pick_list, row, scrollable,
    text,
};
use iced::Color;
use iced::Size;
use iced::{
    color, Center, Element, Fill, Font, Length, Point, Rectangle, Renderer,
    Subscription, Theme,
};
use nats::nats::get_nats_message;
use serde::Deserialize;
use serde::Serialize;
use ws_handler::connect;

mod nats;
mod ws_handler;

pub fn main() -> iced::Result {
    iced::application("Example - Iced", Layout::update, Layout::view)
        .subscription(Layout::subscription)
        .theme(Layout::theme)
        .run()
}

const PANE_ID_COLOR_UNFOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0xC7 as f32 / 255.0,
    0xC7 as f32 / 255.0,
);
const PANE_ID_COLOR_FOCUSED: Color = Color::from_rgb(
    0xFF as f32 / 255.0,
    0x47 as f32 / 255.0,
    0x47 as f32 / 255.0,
);

#[derive(Debug)]
struct Layout {
    explain: bool,
    boards: bool,
    stream_data: StreamData,
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
}

impl Default for Layout {
    fn default() -> Self {
        let (panes, _) = pane_grid::State::new(Pane::new(0));

        Self {
            explain: false,
            boards: true,
            stream_data: StreamData::default(),
            panes,
            panes_created: 0,
            focus: None,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Pane {
    id: usize,
    pub is_pinned: bool,
}

impl Pane {
    fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}


#[derive(Default, Debug, Serialize, Deserialize)]
struct StreamData {
    name: String,
    currency: String,
    index_price: f64,
    unrealized_pnl: f64,
    spread: f64,
    long_ave_entry_price: f64,
    short_ave_entry_price: f64,
    long_size: f64,
    long_usd_notional: f64,
    short_size: f64,
    short_usd_notional: f64,
    dtd_pnl: f64,
    wtd_pnl: f64,
    mtd_pnl: f64,
    dtd_volume_maker: f64,
    dtd_volume_taker: f64,
    wtd_volume_maker: f64,
    wtd_volume_taker: f64,
    mtd_volume_maker: f64,
    mtd_volume_taker: f64,
}



#[derive(Debug, Clone)]
pub enum Message {
    // Next,
    // Previous,
    ExplainToggled(bool),
    ToggleBoards,
    NatsMessageReceived { payload: Vec<u8> },
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    Close(pane_grid::Pane),
    CloseFocused,
    Maximize(pane_grid::Pane),
    Restore,
}

impl Layout {
    fn new() -> Self {
        let (panes, _) = pane_grid::State::new(Pane::new(0));

        Layout {
            explain: false,
            boards: false,
            stream_data: StreamData::default(),
            panes,
            panes_created: 0,
            focus: None,
        }
    }

    fn title(&self) -> String {
        format!("Example - Layout - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            // Message::Next => {
            //     self.example = self.example.next("Button".to_string()); // CHANGE NEXT FUNCTION TO CATER TO A CHANGE IN PAGE
            // }
            // Message::Previous => {
            //     self.example = self.example.previous();
            // }
            Message::ExplainToggled(explain) => {
                self.explain = explain;
            }
            Message::ToggleBoards => {
                self.boards = !self.boards;
            }
            Message::NatsMessageReceived { payload } => {
                self.stream_data = serde_json::from_slice(&payload).unwrap();
                // println!("Stream Data: {:?}", self.stream_data);
            }
            Message::Clicked(pane) => {
                self.focus = Some(pane);
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::Dragged(pane_grid::DragEvent::Dropped {
                pane,
                target,
            }) => {
                self.panes.drop(pane, target);
            }
            Message::Dragged(_) => {}
            Message::Split(axis, pane) => {
                let result =
                    self.panes.split(axis, pane, Pane::new(self.panes_created));

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1;
            }
            Message::SplitFocused(axis) => {
                if let Some(pane) = self.focus {
                    let result = self.panes.split(
                        axis,
                        pane,
                        Pane::new(self.panes_created),
                    );

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;
                }
            }
            Message::Close(pane) => {
                if let Some((_, sibling)) = self.panes.close(pane) {
                    self.focus = Some(sibling);
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.focus {
                    if let Some(Pane { is_pinned, .. }) = self.panes.get(pane) {
                        if !is_pinned {
                            if let Some((_, sibling)) = self.panes.close(pane) {
                                self.focus = Some(sibling);
                            }
                        }
                    }
                }
            }
            Message::Maximize(pane) => self.panes.maximize(pane),
            Message::Restore => {
                self.panes.restore();
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(ws_handler::connect).map(|event| {
            // eprintln!("Received event from WebSocket: {:?}", event);
            event
        })

        // let message_clone = Arc::clone(&self.stream_data);

        // std::thread::spawn(move || {
        //     // Ideally, use an existing runtime if possible
        //     let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            
        //     match runtime.block_on(nats::nats::get_nats_message()) {
        //         Ok(messages) => {
        //             let mut messages_lock = message_clone.lock().unwrap();
        //             *messages_lock = messages;
        //         }
        //         Err(e) => eprintln!("Error fetching NATS message: {:?}", e),
        //     }
        // });


        // use keyboard::key;

        // keyboard::on_key_release(|key, _modifiers| match key {
        //     keyboard::Key::Named(key::Named::ArrowLeft) => {
        //         Some(Message::Previous)
        //     }
        //     keyboard::Key::Named(key::Named::ArrowRight) => Some(Message::Next),
        //     _ => None,
        // })
    }

    fn view(&self) -> Element<Message> {
        let header = row![
            text("Example Header").size(20).font(Font::MONOSPACE),
            horizontal_space(),
            button("Board Management").on_press(Message::ToggleBoards),
            checkbox("Explain", self.explain)
                .on_toggle(Message::ExplainToggled),
        ]
        .spacing(20)
        .align_y(Center);

        // ===================== CONTENT =====================

        // let content = container(
        //     scrollable(
        //         column![
        //             "Content goes here",
        //         ]
        //         .spacing(40)
        //         // .align_x(Center)
        //         // .width(Fill),
        //     )
        //     // .height(Fill),
        // )
        // .padding(10)
        // // .style(container::rounded_box)
        // .style(container::bordered_box);
    
        // let content: Element<Message> = column![content].into();

        // let focus = self.focus;
        // let total_panes = self.panes.len();

        // let pane = create_pane(self.focus, self.panes.len(),&self.panes);
        let pane_grid = create_pane(&self);

        // let pane_grid = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
        //     let is_focused = focus == Some(id);

        //     let title = row![
        //         "Pane",
        //         text(pane.id.to_string()).color(if is_focused {
        //             PANE_ID_COLOR_FOCUSED
        //         } else {
        //             PANE_ID_COLOR_UNFOCUSED
        //         }),
        //     ]
        //     .spacing(5);

        //     let title_bar = pane_grid::TitleBar::new(title)
        //         .controls(pane_grid::Controls::dynamic(
        //             view_controls(
        //                 id,
        //                 total_panes,
        //                 pane.is_pinned,
        //                 is_maximized,
        //             ),
        //             button(text("X").size(14))
        //                 .style(button::danger)
        //                 .padding(3)
        //                 .on_press_maybe(
        //                     if total_panes > 1 && !pane.is_pinned {
        //                         Some(Message::Close(id))
        //                     } else {
        //                         None
        //                     },
        //                 ),
        //         ))
        //         .padding(10)
        //         .style(if is_focused {
        //             style::title_bar_focused
        //         } else {
        //             style::title_bar_active
        //     });

        //     pane_grid::Content::new(responsive(move |size| {
        //         view_content(id, total_panes, pane.is_pinned, size)
        //     }))
        //     .title_bar(title_bar)
        //     .style(if is_focused {
        //         style::pane_focused
        //     } else {
        //         style::pane_active
        //     })
        // })
        // .width(Fill)
        // .height(Fill)
        // .spacing(10)
        // .on_click(Message::Clicked)
        // .on_drag(Message::Dragged)
        // .on_resize(10, Message::Resized);

        // ==========================================================

        // let example = center(if self.explain {
        //     content.explain(color!(0x0000ff))
        // } else {
        //     content
        // })
        // .style(|theme| {
        //     let palette = theme.extended_palette();

        //     container::Style::default()
        //         .border(border::color(palette.background.strong.color).width(4))
        // })
        // .padding(4);

        let sidebar = container(
            column!["Sidebar!", square(50), square(50)]
                .spacing(40)
                .padding(10)
                .width(200)
                .align_x(Center),
        ).center_y(Fill);
        // .style(container::rounded_box);

        let content = container(
            if self.boards {
                row![pane_grid, sidebar].spacing(20)
            } else {
                row![pane_grid].spacing(20)
            }
        ).style(|theme| {
            let palette = theme.extended_palette();

            container::Style::default()
                .border(border::color(palette.background.strong.color).width(4))
        });

        // let content = row![example, sidebar].style(|theme| {
        //     let palette = theme.extended_palette();

        //     container::Style::default()
        //         .border(border::color(palette.background.strong.color).width(4))
        // });


        // let controls = row([
        //     (!self.example.is_first()).then_some(
        //         button("← Previous")
        //             .padding([5, 10])
        //             .on_press(Message::Previous)
        //             .into(),
        //     ),
        //     Some(horizontal_space().into()),
        //     (!self.example.is_last()).then_some(
        //         button("Next →")
        //             .padding([5, 10])
        //             .on_press(Message::Next)
        //             .into(),
        //     ),
        // ]
        // .into_iter()
        // .flatten());

        column![row![header], content]
            .spacing(10)
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::TokyoNight
    }
}

fn square<'a>(size: impl Into<Length> + Copy) -> Element<'a, Message> {
    struct Square;

    impl canvas::Program<Message> for Square {
        type State = ();

        fn draw(
            &self,
            _state: &Self::State,
            renderer: &Renderer,
            theme: &Theme,
            bounds: Rectangle,
            _cursor: mouse::Cursor,
        ) -> Vec<canvas::Geometry> {
            let mut frame = canvas::Frame::new(renderer, bounds.size());

            let palette = theme.extended_palette();

            frame.fill_rectangle(
                Point::ORIGIN,
                bounds.size(),
                palette.background.strong.color,
            );

            vec![frame.into_geometry()]
        }
    }

    canvas(Square).width(size).height(size).into()
}

fn test_view<'a>() -> Element<'a, Message> {
    let header = container(
        row![
            square(40),
            horizontal_space(),
            "Header!",
            horizontal_space(),
            button("Board Management").on_press(Message::ToggleBoards),
        ]
        .padding(10)
        .align_y(Center),
    )
    .style(|theme| {
        let palette = theme.extended_palette();

        container::Style::default()
            .border(border::color(palette.background.strong.color).width(1))
    });

    column![header].into()
}

fn view_content<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
) -> Element<'a, Message> {
    // let button = |label, message| {
    //     button(text(label).width(Fill).align_x(Center).size(16))
    //         .width(Fill)
    //         .padding(8)
    //         .on_press(message)
    // };

    // let controls: iced::widget::Column<'_, Message> = column![
    //     button(
    //         "Split horizontally",
    //         Message::Split(pane_grid::Axis::Horizontal, pane),
    //     ),
    //     button(
    //         "Split vertically",
    //         Message::Split(pane_grid::Axis::Vertical, pane),
    //     )
    // ]
    // .push_maybe(if total_panes > 1 && !is_pinned {
    //     Some(button("Close", Message::Close(pane)).style(button::danger))
    // } else {
    //     None
    // })
    // .spacing(5)
    // .max_width(160);

    // // let board_container = container("This text is centered inside a rounded box!")
    // // .padding(10)
    // // .center(800)
    // // .style(container::rounded_box);

    // let board_container: Element<'a, Message> = column![
    //     "A column can be used to",
    //     "lay out widgets vertically.",
    //     "The amount of space between",
    //     "elements can be configured!",
    // ]
    // .height(Fill)
    // .spacing(40)
    // .into();

    // // button("Manage Board", Message::BoardManagement).style(button::danger)
    // //         .width(Fill)
    // //         .padding(8)
    // //         .on_press(Message::BoardManagement)

    // let content =
    //     column![text!("{}x{}", size.width, size.height).size(24), controls]
    //         .spacing(10)
    //         .align_x(Center);

    // // let content =
    // //     column![text!("{}x{}", size.width, size.height).size(24)]
    // //         .spacing(10)
    // //         .align_x(Center);

    // // let content = row![content_1].spacing(5);

    let content = column!["TEST"].spacing(10).align_x(Center);


    let element: Element<Message> = center(content).padding(5).into();

    element.explain(iced::color!(0x0000ff))
}

fn create_pane(layout: &Layout) -> PaneGrid<Message>{
    let pane_grid = PaneGrid::new(&layout.panes, |id, pane, is_maximized| {
        let is_focused = layout.focus == Some(id);

        let title = row![
            "Pane",
            text(pane.id.to_string()).color(if is_focused {
                PANE_ID_COLOR_FOCUSED
            } else {
                PANE_ID_COLOR_UNFOCUSED
            }),
        ]
        .spacing(5);

        let title_bar = pane_grid::TitleBar::new(title)
            .controls(pane_grid::Controls::dynamic(
                view_controls(
                    id,
                    layout.panes.len(),
                    pane.is_pinned,
                    is_maximized,
                ),
                button(text("X").size(14))
                    .style(button::danger)
                    .padding(3)
                    .on_press_maybe(
                        if layout.panes.len() > 1 && !pane.is_pinned {
                            Some(Message::Close(id))
                        } else {
                            None
                        },
                    ),
            ))
            .padding(10)
            .style(if is_focused {
                style::title_bar_focused
            } else {
                style::title_bar_active
        });

        pane_grid::Content::new(responsive(move |size| {
            let content = view_content(id, layout.panes.len(), pane.is_pinned, size);

            // row![content1].into()
            content

        }))
        .title_bar(title_bar)
        .style(if is_focused {
            style::pane_focused
        } else {
            style::pane_active
        })
    })
    .width(Fill)
    .height(Fill)
    .spacing(10)
    .on_click(Message::Clicked)
    .on_drag(Message::Dragged)
    .on_resize(10, Message::Resized);

    pane_grid
}



fn view_controls<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    is_maximized: bool,
) -> Element<'a, Message> {
    let row = row![].spacing(5).push_maybe(if total_panes > 1 {
        let (content, message) = if is_maximized {
            ("Restore", Message::Restore)
        } else {
            ("Maximize", Message::Maximize(pane))
        };

        Some(
            button(text(content).size(14))
                .style(button::secondary)
                .padding(3)
                .on_press(message),
        )
    } else {
        None
    });



    let close = button(text("Close").size(14))
        .style(button::danger)
        .padding(3)
        .on_press_maybe(if total_panes > 1 && !is_pinned {
            Some(Message::Close(pane))
        } else {
            None
        });

    let buttons = row![row![close].spacing(5)];


    row.push(buttons).into()
}

mod style {
    use iced::widget::container;
    use iced::{Border, Theme};

    pub fn title_bar_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.background.strong.text),
            background: Some(palette.background.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn title_bar_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            text_color: Some(palette.primary.strong.text),
            background: Some(palette.primary.strong.color.into()),
            ..Default::default()
        }
    }

    pub fn pane_active(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.background.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }

    pub fn pane_focused(theme: &Theme) -> container::Style {
        let palette = theme.extended_palette();

        container::Style {
            background: Some(palette.background.weak.color.into()),
            border: Border {
                width: 2.0,
                color: palette.primary.strong.color,
                ..Border::default()
            },
            ..Default::default()
        }
    }
}

// TO TRY NEXT TIME:

// fn create_pane(layout: &Layout) -> PaneGrid<Message> {
//     let pane_grid = PaneGrid::new(&layout.panes, |id, pane, is_maximized| {
//         let is_focused = layout.focus == Some(id);

//         let title = row![
//             "Pane",
//             text(pane.id.to_string()).color(if is_focused {
//                 PANE_ID_COLOR_FOCUSED
//             } else {
//                 PANE_ID_COLOR_UNFOCUSED
//             }),
//         ]
//         .spacing(5);

//         let title_bar = pane_grid::TitleBar::new(title)
//             .controls(pane_grid::Controls::dynamic(
//                 view_controls(
//                     id,
//                     layout.panes.len(),
//                     pane.is_pinned,
//                     is_maximized,
//                 ),
//                 button(text("X").size(14))
//                     .style(button::danger)
//                     .padding(3)
//                     .on_press_maybe(
//                         if layout.panes.len() > 1 && !pane.is_pinned {
//                             Some(Message::Close(id))
//                         } else {
//                             None
//                         },
//                     ),
//             ))
//             .padding(10)
//             .style(if is_focused {
//                 style::title_bar_focused
//             } else {
//                 style::title_bar_active
//         });

//         // === Multiple Sub-Panes Inside This One Pane ===
//         let sub_pane1 = column![
//             pane_grid::TitleBar::new(text("Sub Pane 1"))
//                 .padding(5)
//                 .style(style::title_bar_active),
//             container(text("This is content inside sub-pane 1"))
//                 .padding(10)
//                 .center_x()
//         ]
//         .spacing(10);

//         let sub_pane2 = column![
//             pane_grid::TitleBar::new(text("Sub Pane 2"))
//                 .padding(5)
//                 .style(style::title_bar_active),
//             container(text("This is content inside sub-pane 2"))
//                 .padding(10)
//                 .center_x()
//         ]
//         .spacing(10);

//         let content = column![sub_pane1, sub_pane2].spacing(15);

//         pane_grid::Content::new(content)
//             .title_bar(title_bar)
//             .style(if is_focused {
//                 style::pane_focused
//             } else {
//                 style::pane_active
//         })
//     })
//     .width(Fill)
//     .height(Fill)
//     .spacing(10)
//     .on_click(Message::Clicked)
//     .on_drag(Message::Dragged)
//     .on_resize(10, Message::Resized);

//     pane_grid
// }