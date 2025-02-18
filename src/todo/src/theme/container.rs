use iced::widget::container::Style;
use iced::{color, Border, Theme};

pub fn active_slot(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(202, 233, 255).into()),
        border: Border {
            color: color!(202, 233, 255),
            radius: 10.0.into(),
            width: 5.0,
        },
        ..Default::default()
    }
}

pub fn title(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(131, 122, 117).into()),
        ..Default::default()
    }
}

pub fn list(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(172, 193, 138).into()),
        border: Border {
            radius: 10.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn active_list(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(172, 193, 138).into()),
        border: Border {
            color: color!(202, 233, 255),
            radius: 10.0.into(),
            width: 5.0,
        },
        ..Default::default()
    }
}

pub fn background(_theme: &Theme) -> Style {
    Style {
        background: Some(color!(96, 91, 86).into()),
        ..Default::default()
    }
}
