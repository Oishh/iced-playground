use std::sync::atomic::AtomicUsize;

use iced::advanced::widget::Id;

use iced::{
    widget::{column, container, row, text},
    Element, Length,
};
use iced_drop::droppable;

use crate::{highlight::Highlightable, theme, Message};

pub const NULL_TODO_LOC: TreeLocation = TreeLocation {
    slot: 0,
    element: TreeElement::Slot,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TreeLocation {
    slot: usize,
    element: TreeElement,
}

impl TreeLocation {
    fn new(slot: usize, element: TreeElement) -> Self {
        Self { slot, element }
    }

    pub fn element(&self) -> &TreeElement {
        &self.element
    }

    pub fn slot(&self) -> usize {
        self.slot
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeElement {
    Slot,
    List,
    Todo(usize),
}

/// Contains items organized by slots, and lists
pub struct TreeData {
    slots: Vec<Slot>,
}

impl TreeData {
    pub fn new(slots: Vec<Slot>) -> Self {
        Self { slots }
    }
    /// Convert the tree into an element that iced can render
    pub fn view(&self) -> Element<Message> {
        let children = self.slots.iter().enumerate().map(|(i, slot)| slot.view(i));
        row(children)
            .spacing(10.0)
            .padding(20.0)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn find(&self, id: &Id) -> Option<TreeLocation> {
        for (i, slot) in self.slots.iter().enumerate() {
            if slot.id == *id {
                return Some(TreeLocation::new(i, TreeElement::Slot));
            }
            if slot.list.id == *id {
                return Some(TreeLocation::new(i, TreeElement::List));
            }
        }
        None
    }

    pub fn slot_mut(&mut self, index: usize) -> &mut Slot {
        self.slots.get_mut(index).unwrap()
    }

    pub fn list_mut(&mut self, location: &TreeLocation) -> &mut List {
        let i = location.slot;
        match location.element {
            TreeElement::Slot => &mut self.slots[i].list,
            TreeElement::List => &mut self.slots[i].list,
            TreeElement::Todo(_) => &mut self.slots[i].list,
        }
    }

    pub fn swap_lists(&mut self, l1: &TreeLocation, l2: &TreeLocation) {
        let [s1, s2] = if let Ok(slots) = self.slots.get_many_mut([l1.slot, l2.slot]) {
            slots
        } else {
            return;
        };
        std::mem::swap(&mut s1.list, &mut s2.list);
    }

    /// Returns the widget Id of all the widgets wich a list can be dropped on
    pub fn list_options(&self) -> Vec<Id> {
        self.slots.iter().map(|slot| slot.id.clone()).collect()
    }
}

static NEXT_SLOT: AtomicUsize = AtomicUsize::new(0);

/// Some slot that a list can be dragged into
pub struct Slot {
    id: Id,
    list: List,
    c_id: iced::widget::container::Id,
    highlight: bool,
}

impl Highlightable for Slot {
    fn set_highlight(&mut self, highlight: bool) {
        self.highlight = highlight;
    }
}

impl Slot {
    /// Create a new slot with a list
    pub fn new(list: List) -> Self {
        let id = NEXT_SLOT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let c_id = iced::widget::container::Id::new(format!("slot_{}", id));
        Self {
            id: Id::from(c_id.clone()),
            c_id,
            list,
            highlight: false,
        }
    }

    /// Convert the slot into an element that iced can render
    fn view(&self, index: usize) -> Element<Message> {
        container(self.list.view(index))
            .id(self.c_id.clone())
            .style(if self.highlight {
                theme::container::active_slot
            } else {
                container::transparent
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(3.5)
            .into()
    }
}

static NEXT_LIST: AtomicUsize = AtomicUsize::new(0);

/// Some list that contains to-do tasks and can be dragged into a slot.
/// Tasks can also be dragged into a list.
pub struct List {
    id: Id,
    title: String,
    highlight: bool,
}

impl Highlightable for List {
    fn set_highlight(&mut self, highlight: bool) {
        self.highlight = highlight;
    }
}

impl List {
    /// Create a new list with a title
    pub fn new(title: &str) -> Self {
        let id = NEXT_LIST.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self {
            id: Id::new(format!("list_{}", id)),
            title: title.to_string(),
            highlight: false,
        }
    }

    pub fn id(&self) -> Id {
        self.id.clone()
    }

    /// Convert the list into an element that iced can render
    fn view(&self, slot_index: usize) -> Element<Message> {
        let name = text(self.title.clone())
            .size(20)
            .style(theme::text::list_name);
        let location = TreeLocation::new(slot_index, TreeElement::List);
        let content = container(column![name].spacing(20.0))
            .width(Length::Fill)
            .height(Length::Shrink)
            .padding(10.0)
            .style(if self.highlight {
                theme::container::active_list
            } else {
                theme::container::list
            });
        droppable(content)
            .id(self.id.clone())
            .on_drop(move |p, r| Message::DropList(location, p, r))
            .on_drag(move |p, r| Message::DragList(location, p, r))
            .on_cancel(Message::ListDropCanceled)
            .drag_hide(true)
            .into()
    }
}