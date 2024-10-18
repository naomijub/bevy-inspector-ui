use super::constants::*;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum TextFieldSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl TextFieldSize {
    pub fn default_text_font(&self) -> TextFont {
        TextFont {
            font_size: self.font_size(),
            ..default()
        }
    }

    pub fn default_text_color(&self) -> TextColor {
        TextColor(DEFAULT_FONT_COLOR)
    }

    pub const fn font_size(&self) -> f32 {
        match self {
            Self::Small => SMALL_FONT_SIZE,
            _ => MEDIUM_LARGE_FONT_SIZE,
        }
    }

    pub const fn hint_font_size(&self) -> f32 {
        HINT_FONT_SIZE
    }

    pub const fn label_font_size(&self) -> f32 {
        match self {
            Self::Small => LABEL_SMALL_FONT_SIZE,
            _ => LABEL_MEDIUM_LARGE_FONT_SIZE,
        }
    }

    pub fn padding(&self) -> UiRect {
        UiRect::axes(Val::Px(16.), Val::Px(8.))
    }

    pub const fn height(&self) -> f32 {
        match self {
            Self::Small => 28.,
            Self::Medium => 36.,
            Self::Large => 42.,
        }
    }

    pub const fn min_width(&self) -> f32 {
        match self {
            Self::Small => 110.,
            Self::Medium => 200.,
            Self::Large => 360.,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TextState {
    #[default]
    Default,
    Selected,
    Warning,
    Error,
    Disabled,
}

impl TextState {
    pub const fn background_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_BACKGROUND_COLOR,
            Self::Selected => SELECTED_BACKGROUND_COLOR,
            Self::Warning => WARNING_BACKGROUND_COLOR,
            Self::Error => ERROR_BACKGROUND_COLOR,
            Self::Disabled => DISABLED_BACKGROUND_COLOR,
        }
    }

    pub const fn border_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_BACKGROUND_COLOR,
            Self::Selected => SELECTED_BORDER_COLOR,
            Self::Warning => WARNING_BORDER_COLOR,
            Self::Error => ERROR_BORDER_COLOR,
            Self::Disabled => DISABLED_BACKGROUND_COLOR,
        }
    }

    pub const fn hint_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_HINT_COLOR,
            Self::Selected => DEFAULT_HINT_COLOR,
            Self::Warning => WARNING_HINT_COLOR,
            Self::Error => ERROR_HINT_COLOR,
            Self::Disabled => DISABLED_HINT_COLOR,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextFieldBuilder {
    size: TextFieldSize,
    label: Option<String>,
    placeholder: Option<String>,
    hint_text: Option<String>,
}

impl TextFieldBuilder {
    pub const fn with_size(mut self, size: TextFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn with_hint_text(mut self, hint_text: String) -> Self {
        self.hint_text = Some(hint_text);
        self
    }
}

/// Marks Text Field as disabled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct DisableTextField;

/// Marks Text Field as warning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct WarningTextField;

/// Marks Text Field as error
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ErrorTextField;

/// Marks Text Field placeholder
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct Placeholder(pub String);

impl Placeholder {
    pub fn text_font(size: &TextFieldSize) -> TextFont {
        TextFont {
            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: size.font_size(),
            ..default()
        }
    }

    pub const fn text_color() -> TextColor {
        TextColor(Color::srgba(0.29, 0.31, 0.33, 0.87))
    }
}
