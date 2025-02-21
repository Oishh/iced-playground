use iced::alignment::Alignment;
use iced::theme::{self, Theme};
use iced::widget::pane_grid::Pane;
use iced::widget::{
    button, checkbox, column, container, horizontal_space, pane_grid, pick_list, row, scrollable, text, text_input, vertical_space, Button, Column, Container, PaneGrid, PickList, Text
};
use iced::Alignment::Center;
use iced::{
    executor, keyboard, Application, Color, Element,
    Event, Font, Length, Settings, Size, Subscription,
};

pub fn main() -> iced::Result {
    iced::application("Pane Grid - Iced", Example::update, Example::view)
        .subscription(Example::subscription)
        .run()
}

#[derive(Clone)]
struct Example {
    panes: pane_grid::State<DefinedPane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    pane_contents: Vec<(Pane, PaneContent)>
}

#[derive(Debug, Clone, Copy)]
enum PaneContent {
    Text,
    Buttons,
    Image,
}

impl Default for Example {
    fn default() -> Self {
        Example::new()
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Maximize(pane_grid::Pane),
    Restore,
    Close(pane_grid::Pane),
    CloseFocused,
    TogglePin(pane_grid::Pane),
}

impl Example {
    fn new() -> Self {
        // Create the initial pane state
        let (mut state, first) = pane_grid::State::new(DefinedPane::new(0));

        // Split the first pane vertically
        let (second, _) = state
            .split(pane_grid::Axis::Vertical, first, DefinedPane::new(1))
            .unwrap();

        // Split the right pane horizontally
        let (third, _) = state
            .split(pane_grid::Axis::Horizontal, second, DefinedPane::new(2))
            .unwrap();

        Example {
            panes: state,
            panes_created: 3,
            focus: None,
            pane_contents: vec![
                (first, PaneContent::Text),
                (second, PaneContent::Buttons),
                (third, PaneContent::Image),
            ].into_iter().collect(),
        }
    }

    fn title(&self) -> String {
        String::from("Pane grid - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Split(axis, pane) => {
                let result = self.panes.split(
                    axis,
                    pane,
                    DefinedPane::new(self.panes_created),
                );

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
                        DefinedPane::new(self.panes_created),
                    );

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;
                }
            }
            Message::FocusAdjacent(direction) => {
                if let Some(pane) = self.focus {
                    if let Some(adjacent) =
                        self.panes.adjacent(pane, direction)
                    {
                        self.focus = Some(adjacent);
                    }
                }
            }
            Message::Clicked(pane) => {
                self.focus = Some(pane);
            }
            Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            Message::Dragged(_) => {}
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::Maximize(pane) => {
                self.panes.maximize(pane);
            }
            Message::Restore => {
                self.panes.restore();
            }
            Message::Close(pane) => {
                if let Some((_, sibling)) = self.panes.close(pane) {
                    self.focus = Some(sibling);
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.focus {
                    if let Some(DefinedPane { is_pinned, .. }) = self.panes.get(pane) {
                        if !is_pinned {
                            if let Some((_, sibling)) = self.panes.close(pane) {
                                self.focus = Some(sibling);
                            }
                        }
                    }
                }
            }
            Message::TogglePin(pane) => {
                if let Some(DefinedPane { is_pinned, .. }) = self.panes.get_mut(pane) {
                    *is_pinned = !*is_pinned;
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key_code, modifiers| {
            if !modifiers.command() {
                return None;
            }

            handle_hotkey(key_code)
        })
    }

    fn view(&self) -> Element<Message> {
        let focus = self.focus;
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, is_maximized| {
            let is_focused = focus == Some(id);

            let title = pane_grid::TitleBar::new(
                row![
                    text("Pane").size(16),
                    text(pane.id.to_string())
                        .size(16)
                        .color(if is_focused {
                            PANE_ID_COLOR_FOCUSED
                        } else {
                            PANE_ID_COLOR_UNFOCUSED
                        }),
                ]
                .spacing(5)
                .align_y(Alignment::Center),
            )
            .style(if is_focused {
                style::title_bar_focused
            } else {
                style::title_bar_active
            })
            .padding(10);

            let is_pinned = pane.is_pinned;

            let content = self.pane_contents
            .iter()
            .find(|(p, _)| *p == id)
            .map(|(_, content)| content)
            .cloned()
            .unwrap_or(PaneContent::Text);
            

            pane_grid::Content::new(responsive(move |size| {
                match content {
                    PaneContent::Text => view_text_content(id, total_panes, is_pinned, size),
                    PaneContent::Buttons => view_buttons_content(id, total_panes, is_pinned, size),
                    PaneContent::Image => view_image_content(id, total_panes, is_pinned, size),
                }
            }))
            .title_bar(title)
            .style(if is_focused {
                style::pane_focused
            } else {
                style::pane_active
            })
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        container(pane_grid).padding(10).into()
    }
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

fn handle_hotkey(key: keyboard::Key) -> Option<Message> {
    use keyboard::key::{self, Key};
    use pane_grid::{Axis, Direction};

    match key.as_ref() {
        Key::Character("v") => Some(Message::SplitFocused(Axis::Vertical)),
        Key::Character("h") => Some(Message::SplitFocused(Axis::Horizontal)),
        Key::Character("w") => Some(Message::CloseFocused),
        Key::Named(key) => {
            let direction = match key {
                key::Named::ArrowUp => Some(Direction::Up),
                key::Named::ArrowDown => Some(Direction::Down),
                key::Named::ArrowLeft => Some(Direction::Left),
                key::Named::ArrowRight => Some(Direction::Right),
                _ => None,
            };

            direction.map(Message::FocusAdjacent)
        }
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
struct DefinedPane {
    id: usize,
    pub is_pinned: bool,
}

impl DefinedPane {
    fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}

fn view_content<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
) -> Element<'a, Message> {
    let button = |label, message| {
        Button::new(
            Text::new(label)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .size(16),
        )
        .width(Length::Fill)
        .padding(8)
        .on_press(message)
    };

    let mut controls = Column::new()
        .push(button(
            "Split horizontally",
            Message::Split(pane_grid::Axis::Horizontal, pane),
        ))
        .push(button(
            "Split vertically",
            Message::Split(pane_grid::Axis::Vertical, pane),
        ))
        .spacing(5)
        .max_width(160);

    if total_panes > 1 && !is_pinned {
        controls = controls.push(
            button("Close", Message::Close(pane))
                .style(button::danger),
        );
    }

    let content = Column::new()
        .push(Text::new(format!("{}x{}", size.width, size.height)).size(24))
        .push(controls)
        .spacing(10)
        .align_x(Alignment::Center);

    Container::new(scrollable(content))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(Center)
        .padding(5)
        .into()
}

fn responsive(f: impl Fn(Size) -> Element<'static, Message> + 'static) -> Element<'static, Message> {
    Container::new(iced::widget::responsive(f))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
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

fn view_text_content<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
) -> Element<'a, Message> {
    // Implement text content view
    Column::new()
        .push(Text::new("Text Content").size(24))
        .push(Text::new(format!("Pane size: {}x{}", size.width, size.height)))
        .into()
}

fn view_buttons_content<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
) -> Element<'a, Message> {
    // Implement buttons content view
    Column::new()
        .push(Text::new("Buttons Content").size(24))
        .push(Button::new("Button 1").on_press(Message::Clicked(pane)))
        .push(Button::new("Button 2").on_press(Message::Clicked(pane)))
        .into()
}

fn view_image_content<'a>(
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
) -> Element<'a, Message> {
    // Implement image content view
    // Note: You'll need to add an actual image here
    Column::new()
        .push(Text::new("Image Content").size(24))
        .push(Text::new("(Placeholder for actual image)"))
        .into()
}