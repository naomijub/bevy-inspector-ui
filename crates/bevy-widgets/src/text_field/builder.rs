use super::{
    constants::*, TextInput, TextInputSettings, TextInputTextColor, TextInputTextFont,
    TextInputValue,
};
use bevy::prelude::*;

/// Textcomponent qualifying label and hint texts
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Component, Reflect)]
#[reflect(Component)]
pub struct TextInputDescriptions {
    pub(crate) label: Option<String>,
    pub(crate) hint: Option<String>,
}

/// Text fields can be classified accordingly to their height:
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component, Reflect)]
#[reflect(Component)]
pub enum TextInputSize {
    /// small: font size: 10px, label font size: 8px, padding: 16px x 8px, field height: 28px, min width: 110px
    Small,
    #[default]
    /// medium: font size: 13px, label font size: 10px, padding: 16px x 8px, field height: 36px, min width: 200px
    Medium,
    /// large: font size: 13px, label font size: 10px, padding: 16px x 8px, field height: 42px, min width: 360px
    Large,
}

impl TextInputSize {
    /// Default [`TextFont`] component for specific size
    pub fn default_text_font(&self) -> TextFont {
        TextFont {
            font_size: self.font_size(),
            ..default()
        }
    }

    /// Checks if the `TextInputSize` is `Large`.
    ///
    /// # Returns
    ///
    /// * `true` if the size is `Large`.
    /// * `false` otherwise.
    pub const fn is_large(&self) -> bool {
        match self {
            Self::Small => false,
            Self::Medium => false,
            Self::Large => true,
        }
    }

    /// Default [`TextColor`] component
    pub const fn default_text_color(&self) -> TextColor {
        TextColor(DEFAULT_FONT_COLOR)
    }

    /// Default font size for [`TextInputSize`]
    pub const fn font_size(&self) -> f32 {
        match self {
            Self::Small => SMALL_FONT_SIZE,
            _ => MEDIUM_LARGE_FONT_SIZE,
        }
    }

    /// Hint text font size
    pub const fn hint_font_size(&self) -> f32 {
        HINT_FONT_SIZE
    }

    /// Label font size for [`TextInputSize`]
    pub const fn label_font_size(&self) -> f32 {
        match self {
            Self::Small => LABEL_SMALL_FONT_SIZE,
            _ => LABEL_MEDIUM_LARGE_FONT_SIZE,
        }
    }

    /// Padding for TextInput
    pub fn padding(&self, has_label: bool) -> UiRect {
        let vertical = match (has_label, self) {
            (true, Self::Small) => 2.,
            (true, Self::Medium) => 4.,
            (true, Self::Large) => 4.,
            (false, _) => 8.,
        };
        UiRect::axes(Val::Px(16.), Val::Px(vertical))
    }

    /// Height for [`TextInputSize`]
    pub const fn height(&self) -> f32 {
        match self {
            Self::Small => 28.,
            Self::Medium => 36.,
            Self::Large => 42.,
        }
    }

    /// Hint text spacing for [`TextInputSize`]
    pub const fn hint_text_spacing(&self) -> f32 {
        match self {
            Self::Small => 2.,
            Self::Medium => 10.,
            Self::Large => 10.,
        }
    }

    /// Minimum width of the field for [`TextInputSize`]
    pub const fn min_width(&self) -> f32 {
        match self {
            Self::Small => 110.,
            Self::Medium => 200.,
            Self::Large => 360.,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component, Reflect)]
#[reflect(Component)]
/// Text input state
pub enum TextInputState {
    #[default]
    /// Default state, enabled but inactive
    Default,
    /// Selected state = ready to receive input
    Selected,
    /// Warning state
    Warning,
    /// Error state = validation gone wrong
    Error,
    /// Disabled state, cannot receive input
    Disabled,
}

impl TextInputState {
    pub(crate) const fn background_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_BACKGROUND_COLOR,
            Self::Selected => SELECTED_BACKGROUND_COLOR,
            Self::Warning => WARNING_BACKGROUND_COLOR,
            Self::Error => ERROR_BACKGROUND_COLOR,
            Self::Disabled => DISABLED_BACKGROUND_COLOR,
        }
    }

    pub(crate) const fn border_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_BACKGROUND_COLOR,
            Self::Selected => SELECTED_BORDER_COLOR,
            Self::Warning => WARNING_BORDER_COLOR,
            Self::Error => ERROR_BORDER_COLOR,
            Self::Disabled => DISABLED_BACKGROUND_COLOR,
        }
    }

    pub(crate) const fn hint_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_HINT_COLOR,
            Self::Selected => DEFAULT_HINT_COLOR,
            Self::Warning => WARNING_HINT_COLOR,
            Self::Error => ERROR_HINT_COLOR,
            Self::Disabled => DISABLED_HINT_COLOR,
        }
    }

    pub(crate) const fn label_color(&self) -> Color {
        match self {
            Self::Warning => WARNING_HINT_COLOR,
            Self::Error => ERROR_HINT_COLOR,
            _ => DISABLED_HINT_COLOR,
        }
    }
}

/// Builder for [`TextInput`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextInputBuilder {
    size: TextInputSize,
    label: Option<String>,
    placeholder: Option<String>,
    hint_text: Option<String>,
    mask: Option<char>,
    retain_on_submit: bool,
    value: String,
}

impl Default for TextInputBuilder {
    fn default() -> Self {
        Self {
            size: TextInputSize::Medium,
            label: None,
            placeholder: None,
            hint_text: None,
            mask: None,
            retain_on_submit: true,
            value: String::new(),
        }
    }
}

impl TextInputBuilder {
    /// Sets to clear text field on input
    pub const fn clear_on_submit(mut self) -> Self {
        self.retain_on_submit = false;
        self
    }

    /// Sets mask to password
    pub const fn password(mut self) -> Self {
        self.mask = Some('*');
        self
    }

    /// Sets the mask character
    pub const fn with_mask(mut self, mask: char) -> Self {
        self.mask = Some(mask);
        self
    }

    /// Sets the [`TextInputSize`]
    pub const fn with_size(mut self, size: TextInputSize) -> Self {
        self.size = size;
        self
    }

    /// Adds a label to the text field
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    /// Adds a initial value to the text field
    pub fn with_initial_value(mut self, value: String) -> Self {
        self.value = value;
        self
    }

    /// Adds a placeholder to the text field
    pub fn with_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Adds a hint text to the text field
    pub fn with_hint_text(mut self, hint_text: String) -> Self {
        self.hint_text = Some(hint_text);
        self
    }

    /// Builds the text field
    pub fn build(
        self,
    ) -> (
        TextInput,
        Node,
        BorderColor,
        BackgroundColor,
        BorderRadius,
        TextInputSettings,
        TextInputTextColor,
        TextInputTextFont,
        TextInputValue,
        TextInputState,
        TextInputSize,
        Placeholder,
        TextInputDescriptions,
    ) {
        let settings = TextInputSettings {
            retain_on_submit: self.retain_on_submit,
            mask_character: self.mask,
        };
        let color = TextInputTextColor(self.size.default_text_color());
        let font = TextInputTextFont(self.size.default_text_font());
        let value = TextInputValue(self.value.clone());
        let placeholder = Placeholder(self.placeholder.unwrap_or_default());
        let extras = TextInputDescriptions {
            label: self.label,
            hint: self.hint_text,
        };

        (
            TextInput,
            Node {
                height: Val::Px(self.size.height()),
                min_width: Val::Px(self.size.min_width()),
                border: UiRect::all(Val::Px(2.0)),
                padding: self.size.padding(extras.label.is_some()),
                ..default()
            },
            TextInputState::Default.border_color().into(),
            TextInputState::Default.background_color().into(),
            BorderRadius::all(Val::Px(8.0)),
            settings,
            color,
            font,
            value,
            TextInputState::Default,
            self.size,
            placeholder,
            extras,
        )
    }
}

/// Marks Text Field as disabled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct DisableTextInput;

/// Marks Text Field as warning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct WarningTextInput;

/// Marks Text Field as error
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ErrorTextInput;

/// Marks Text Field placeholder
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct Placeholder(pub String);

impl Placeholder {
    /// Placeholder font size
    pub fn text_font(size: &TextInputSize) -> TextFont {
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
