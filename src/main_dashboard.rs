use iced::border;
use iced::mouse;
use iced::widget::pane_grid;
use iced::widget::responsive;
use iced::widget::Container;
use iced::widget::PaneGrid;
use iced::widget::Text;
use iced::widget::{
    button, canvas, center, checkbox, column, container,
    horizontal_space, row,
    text,
};
use iced::Color;
use iced::Padding;
use iced::Size;
use iced::{
    Center, Element, Fill, Font, Length, Point, Rectangle, Renderer,
    Subscription, Theme,
};
use serde::Deserialize;
use serde::Serialize;

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
struct Layout{
    explain: bool,
    boards: bool,
    stream_data: StreamData,
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    dragged: Option<usize>,
    items: Vec<String>,
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
            dragged: None,
            items: vec!["Perp Risk Metrics Module".to_string(), "Perp Performance Metrics Module".to_string(), "Positions Module".to_string()],
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
    Drag(usize, usize),
    Drop(usize, usize),
    Cancel(usize),
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
            dragged: None,
            items: Vec::new(),
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
            Message::Drag(key, _loc) => {
                self.dragged = Some(key);
            }
            Message::Drop(key, loc) => {
                self.drop_item(key, loc);
                self.dragged = None;
            }
            Message::Cancel(_key) => {
                self.dragged = None;
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(ws_handler::connect).map(|event| {
            // eprintln!("Received event from WebSocket: {:?}", event);
            event
        })
    }

    fn drop_item(&mut self, key: usize, mut loc: usize) {
        if key > self.items.len() || key == loc || key + 1 == loc {
            return;
        }
        if loc > key {
            loc -= 1;
        }
        let slot = self.items.remove(key);
        if loc < self.items.len() {
            self.items.insert(loc, slot);
        } else {
            self.items.push(slot);
        }
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
        // let pane_grid = create_pane_grid();
        // let pane_grid = text("Pane Grid").width(Fill).height(Fill).align_x(Center).align_y(Center).size(30).color(Color::WHITE);

        // let pane_grid = container(column![
        //     row![
        //         container(text("Pane 1").size(100).color(Color::WHITE)).style(container::rounded_box).width(Fill),
        //         horizontal_space(),
        //         container(text("Pane 2").size(100).color(Color::WHITE)).style(container::rounded_box).width(Fill),
        //         horizontal_space(),
        //         container(text("Pane 3").size(100).color(Color::WHITE)).style(container::rounded_box).width(Fill),
        //     ],
        //     horizontal_space(),
        //     row![
        //         container(text("Pane 4").size(100).color(Color::WHITE)).style(container::rounded_box).width(Fill),
        //         horizontal_space(),
        //         container(text("Pane 5").size(100).color(Color::WHITE)).style(container::rounded_box).width(Fill),
        //         horizontal_space(),
        //         container(text("Pane 6").size(100).color(Color::WHITE)).style(container::rounded_box).width(Fill),
        //     ],
        // ].spacing(20)).padding(20).align_x(Center).align_y(Center).width(Fill).height(Fill);

        const ITEM_PADDING: Padding = Padding {
            top: 5.0,
            right: 8.0,
            bottom: 5.0,
            left: 8.0,
        };

        let reorderable_items =
            iced_playground::Column::from_iter(self.items.iter().enumerate().map(|(index, item)| {
                let item_style = if Some(index) == self.dragged {
                    style::item_dragged
                } else {
                    style::item_idle
                };
                let content = Container::new(Text::new(item))
                    .align_x(Center)
                    .align_y(Center)
                    .height(Fill)
                    .width(Fill)
                    .padding(ITEM_PADDING)
                    .style(item_style);

                (index, content.into())
            }))
            .spacing(10)
            .padding(Padding::default())
            .on_drag(|key, index| Message::Drag(key, index))
            .on_drop(|key, index| Message::Drop(key, index))
            .on_cancel(|key| Message::Cancel(key))
            .drop_position_marker(true);


        let content = column!(reorderable_items).spacing(24).padding(12);

        let pane_grid = Container::new(content).center(Length::Fill);

        // let pane_grid = create_pane(&self);
        // ==========================================================

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
    let content = column!["TEST"].spacing(10).align_x(Center);


    let element: Element<Message> = center(content).padding(5).into();

    element.explain(iced::color!(0x0000ff))
}

// fn create_pane(layout: &Layout) -> PaneGrid<Message>{
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

//         pane_grid::Content::new(responsive(move |size| {
//             let content = view_content(id, layout.panes.len(), pane.is_pinned, size);

//             // row![content1].into()
//             content

//         }))
//         .title_bar(title_bar)
//         .style(if is_focused {
//             style::pane_focused
//         } else {
//             style::pane_active
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

// mod style {
//     use iced::widget::container;
//     use iced::{Border, Theme};

//     pub fn title_bar_active(theme: &Theme) -> container::Style {
//         let palette = theme.extended_palette();

//         container::Style {
//             text_color: Some(palette.background.strong.text),
//             background: Some(palette.background.strong.color.into()),
//             ..Default::default()
//         }
//     }

//     pub fn title_bar_focused(theme: &Theme) -> container::Style {
//         let palette = theme.extended_palette();

//         container::Style {
//             text_color: Some(palette.primary.strong.text),
//             background: Some(palette.primary.strong.color.into()),
//             ..Default::default()
//         }
//     }

//     pub fn pane_active(theme: &Theme) -> container::Style {
//         let palette = theme.extended_palette();

//         container::Style {
//             background: Some(palette.background.weak.color.into()),
//             border: Border {
//                 width: 2.0,
//                 color: palette.background.strong.color,
//                 ..Border::default()
//             },
//             ..Default::default()
//         }
//     }

//     pub fn pane_focused(theme: &Theme) -> container::Style {
//         let palette = theme.extended_palette();

//         container::Style {
//             background: Some(palette.background.weak.color.into()),
//             border: Border {
//                 width: 2.0,
//                 color: palette.primary.strong.color,
//                 ..Border::default()
//             },
//             ..Default::default()
//         }
//     }
// }

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

mod style {
    use iced::widget::container;
    use iced::Theme;

    pub fn item_idle(theme: &Theme) -> container::Style {
        container::Style {
            border: iced::Border {
                color: theme.extended_palette().secondary.weak.color.into(),
                width: 1.0,
                radius: 5.0.into(),
            },
            ..Default::default()
        }
    }

    pub fn item_dragged(theme: &Theme) -> container::Style {
        container::Style {
            border: iced::Border {
                color: theme.extended_palette().primary.strong.color.into(),
                width: 1.0,
                radius: 5.0.into(),
            },
            ..Default::default()
        }
    }
}