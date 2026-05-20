use iced::widget::{button, container};
use iced::{Background, Border, Color, Shadow, Theme};
use iced::theme::Palette;

pub const LIGHT_GREY: Color = Color::from_rgb(216.0 / 255.0, 223.0 / 255.0, 220.0 / 255.0);
pub const NEON_YELLOW: Color = Color::from_rgb(222.0 / 255.0, 1.0, 175.0 / 255.0);
pub const GREY: Color = Color::from_rgb(178.0 / 255.0, 178.0 / 255.0, 178.0 / 255.0);
pub const BEIGE: Color = Color::from_rgb(246.0 / 255.0, 246.0 / 255.0, 242.0 / 255.0);
pub const BLACK: Color = Color::BLACK;
pub const WHITE: Color = Color::WHITE;

pub fn living_models_theme() -> Theme {
    Theme::custom(
        "Living Models".to_string(),
        Palette {
            background: LIGHT_GREY,
            text: BLACK,
            primary: NEON_YELLOW,
            success: NEON_YELLOW,
            warning: NEON_YELLOW,
            danger: BLACK,
        },
    )
}

pub fn canvas(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(LIGHT_GREY)),
        text_color: Some(BLACK),
        ..Default::default()
    }
}

pub fn drop_zone_idle(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(BEIGE)),
        border: Border {
            color: GREY,
            width: 1.5,
            radius: 14.0.into(),
        },
        text_color: Some(BLACK),
        ..Default::default()
    }
}

pub fn drop_zone_hovered(_: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(NEON_YELLOW)),
        border: Border {
            color: BLACK,
            width: 1.5,
            radius: 14.0.into(),
        },
        text_color: Some(BLACK),
        ..Default::default()
    }
}

pub fn primary_button(_: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(NEON_YELLOW)),
        text_color: BLACK,
        border: Border {
            color: BLACK,
            width: 1.0,
            radius: 999.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(BLACK)),
            text_color: NEON_YELLOW,
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(BLACK)),
            text_color: NEON_YELLOW,
            ..base
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(GREY)),
            text_color: Color { a: 0.6, ..BLACK },
            border: Border {
                color: GREY,
                width: 1.0,
                radius: 999.0.into(),
            },
            ..base
        },
    }
}

pub fn secondary_button(_: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: Some(Background::Color(Color::TRANSPARENT)),
        text_color: BLACK,
        border: Border {
            color: BLACK,
            width: 1.0,
            radius: 999.0.into(),
        },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        button::Status::Active => base,
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(BLACK)),
            text_color: WHITE,
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(BLACK)),
            text_color: WHITE,
            ..base
        },
        button::Status::Disabled => button::Style {
            text_color: GREY,
            border: Border {
                color: GREY,
                width: 1.0,
                radius: 999.0.into(),
            },
            ..base
        },
    }
}
