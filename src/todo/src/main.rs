#![feature(get_many_mut)]
#![feature(hash_raw_entry)]

use highlight::{should_update_droppable, zone_update, Highlight, Highlightable, ZoneUpdate};
use iced::{
    advanced::widget::Id,
    widget::{column, container, text},
    Element, Length, Point, Rectangle, Task,
};
use iced_drop::find_zones;
use iced_drop::widget::droppable::State as DroppableState;
use operation::swap_modify_states;
use tree::{List, Slot, TreeData, TreeLocation};

mod highlight;
mod operation;
mod theme;
mod tree;

const HEADER_HEIGHT: f32 = 80.0;

fn main() -> iced::Result {
    iced::application(TodoBoard::title, TodoBoard::update, TodoBoard::view)
        .theme(TodoBoard::theme)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    // Drag/drop lists
    #[allow(dead_code)]
    DragList(TreeLocation, Point, Rectangle),
    HandleListZones(Vec<(Id, Rectangle)>),
    #[allow(dead_code)]
    DropList(TreeLocation, Point, Rectangle),
    ListDropCanceled,
}

struct TodoBoard {
    tree: TreeData,
    todos_highlight: highlight::Highlight,
    lists_highlight: highlight::Highlight,
}

impl Default for TodoBoard {
    fn default() -> Self {
        Self {
            tree: TreeData::new(vec![
                Slot::new(List::new("Todo")),
                Slot::new(List::new("Doing")),
                Slot::new(List::new("Done")),
            ]),
            todos_highlight: Highlight::default(),
            lists_highlight: Highlight::default(),
        }
    }
}

impl TodoBoard {
    fn title(&self) -> String {
        "Todo".to_string()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::CatppuccinFrappe
    }

    fn view(&self) -> Element<'_, Message> {
        let header = container(text("TODO Board").size(30).style(theme::text::title))
            .padding(10.0)
            .width(Length::Fill)
            .height(Length::Fixed(HEADER_HEIGHT))
            .style(theme::container::title);
        container(
            column![header, self.tree.view()]
                .height(Length::Fill)
                .width(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(theme::container::background)
        .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // List drag/drop
            Message::DragList(l_loc, _, l_bounds) => {
                let new_highlight =
                    highlight::dragged(&self.lists_highlight, l_loc.clone(), l_bounds);
                if should_update_droppable(&self.lists_highlight, &new_highlight, &l_loc) {
                    self.tree.list_mut(&l_loc).set_highlight(false);
                }
                self.lists_highlight = new_highlight;
                return find_zones(
                    Message::HandleListZones,
                    move |zone_bounds| zone_bounds.intersects(&l_bounds),
                    Some(self.tree.list_options()),
                    None,
                );
            }
            Message::HandleListZones(zones) => {
                let new_info =
                    highlight::zones_found(&self.lists_highlight, &map_zones(&self.tree, zones));
                let highlight_update = zone_update(&self.lists_highlight, &new_info);
                highlight_update.update(&mut self.tree, &self.lists_highlight, &new_info);
                self.lists_highlight = new_info;

                if highlight_update == ZoneUpdate::Replace {
                    if let Some(d_loc) = &self.lists_highlight.dragging {
                        if let Some(h_loc) = &self.lists_highlight.hovered {
                            return move_list_to_zone(&mut self.tree, &d_loc.0, &h_loc);
                        }
                    }
                }
            }
            Message::DropList(l_loc, _, _) => {
                self.tree.list_mut(&l_loc).set_highlight(false);
                if let Some(s_loc) = &self.lists_highlight.hovered {
                    self.tree.slot_mut(s_loc.slot()).set_highlight(false);
                }
                self.todos_highlight = highlight::dropped();
            }
            Message::ListDropCanceled => {
                if let Some(d_loc) = &self.lists_highlight.dragging {
                    self.tree.list_mut(&d_loc.0).set_highlight(false);
                    self.tree.slot_mut(d_loc.0.slot()).set_highlight(false);
                }
                self.lists_highlight = highlight::dropped();
            }
        }
        Task::none()
    }
}

fn map_zones(tree: &TreeData, zones: Vec<(Id, Rectangle)>) -> Vec<(TreeLocation, Rectangle)> {
    zones
        .into_iter()
        .filter_map(|(id, rect)| {
            if let Some(loc) = tree.find(&id) {
                Some((loc, rect))
            } else {
                None
            }
        })
        .collect()
}

fn move_list_to_zone(
    tree: &mut TreeData,
    d_loc: &TreeLocation,
    h_loc: &TreeLocation,
) -> Task<Message> {
    let l1 = tree.list_mut(d_loc).id();
    let l2 = tree.list_mut(h_loc).id();
    tree.swap_lists(d_loc, h_loc);
    return swap_modify_states(l1, l2, |_old: &DroppableState, new: &DroppableState| {
        new.clone()
    });
}
