use bevy::prelude::*;

use super::InputFieldSize;

#[derive(Component, Reflect)]
pub(crate) struct TextInputPlaceholderInner;

/// Textcomponent qualifying label and hint texts
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Component, Reflect)]
#[reflect(Component)]
pub struct TextInputDescriptions {
    pub(crate) label: Option<String>,
    pub(crate) hint: Option<String>,
}

/// Marks Text Field placeholder
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Placeholder(pub String);

impl Placeholder {
    /// Placeholder font size
    pub fn text_font(size: &InputFieldSize) -> TextFont {
        TextFont {
            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: size.font_size(),
            ..default()
        }
    }

    /// Placeholder text color
    pub const fn text_color() -> TextColor {
        TextColor(Color::srgba(0.29, 0.31, 0.33, 0.87))
    }
}
