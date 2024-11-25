use std::collections::BTreeSet;

use bevy::prelude::*;

use super::constants::*;
pub mod numeric;
pub mod text;

/// A wrapper for Bevy `TextFont` that will be used when creating the text input's inner Bevy `TextBundle`.
#[derive(Component, Default, Reflect)]
pub struct InputTextFont(pub TextFont);

/// A wrapper for Bevy `TextColor` that will be used when creating the text input's inner Bevy `TextBundle`.
#[derive(Component, Default, Reflect)]
pub struct InputTextColor(pub TextColor);

/// If true, the text input does not respond to keyboard events and the cursor is hidden.
/// This is different than disabled, as the value can be changed on selecting
#[derive(Component, Reflect)]
pub struct InputInactive(pub(crate) bool);

impl Default for InputInactive {
    fn default() -> Self {
        Self(true)
    }
}

impl InputInactive {
    /// Toggles the `TextInputInactive` component to be active
    pub fn active(&mut self) {
        self.0 = false;
    }

    /// Toggles the `TextInputInactive` component to be inactive
    pub fn inactive(&mut self) {
        self.0 = true;
    }
}

/// A component that manages the cursor's blinking.
#[derive(Component, Reflect)]
pub struct InputCursorTimer {
    /// The timer that blinks the cursor on and off, and resets when the user types.
    pub timer: Timer,
    pub(super) should_reset: bool,
}

impl Default for InputCursorTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            should_reset: false,
        }
    }
}

/// A component containing the text input's settings.
#[derive(Component, Reflect)]
pub struct InputFieldSettings {
    /// If true, text is not cleared after pressing enter. Defaults to true.
    pub retain_on_submit: bool,
    /// Mask text with the provided character. Defaults to `None`, when calling `.password()` it defaults to `Some('*')`.
    pub mask_character: Option<char>,
}

impl Default for InputFieldSettings {
    fn default() -> Self {
        Self {
            retain_on_submit: true,
            mask_character: None,
        }
    }
}

/// A component containing the current value of the text input.
#[derive(Component, Default, Reflect)]
pub struct InputTextValue(pub(crate) String);

/// A component containing the current text cursor position.
#[derive(Component, Default, Reflect)]
pub struct InputTextCursorPos(pub(crate) usize);

#[derive(Component, Reflect)]
pub(crate) struct TextInputInner;

/// An event that is fired when the user presses the enter key.
#[derive(Event, Debug, Reflect)]
pub struct InputFieldSubmitEvent {
    /// The text input that triggered the event.
    pub entity: Entity,
    /// The string contained in the text input at the time of the event.
    pub value: String,
}

/// Text fields can be classified accordingly to their height:
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component, Reflect)]
#[reflect(Component)]
pub enum InputFieldSize {
    /// small: font size: 10px, label font size: 8px, padding: 16px x 8px, field height: 28px, min width: 110px
    Small,
    #[default]
    /// medium: font size: 13px, label font size: 10px, padding: 16px x 8px, field height: 36px, min width: 200px
    Medium,
    /// large: font size: 13px, label font size: 10px, padding: 16px x 8px, field height: 42px, min width: 360px
    Large,
}

impl InputFieldSize {
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
pub enum InputFieldState {
    #[default]
    /// Default state, enabled but inactive
    Default,
    /// Selected state = ready to receive input
    Selected,
    /// Hover state
    Hovered,
    /// Warning state
    Warning,
    /// Error state = validation gone wrong
    Error,
    /// Disabled state, cannot receive input
    Disabled,
}

impl InputFieldState {
    pub(crate) const fn background_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_BACKGROUND_COLOR,
            Self::Selected => SELECTED_BACKGROUND_COLOR,
            Self::Warning => WARNING_BACKGROUND_COLOR,
            Self::Error => ERROR_BACKGROUND_COLOR,
            Self::Disabled => DISABLED_BACKGROUND_COLOR,
            Self::Hovered => HOVERED_BACKGROUND_COLOR,
        }
    }

    pub(crate) const fn border_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_BACKGROUND_COLOR,
            Self::Selected => SELECTED_BORDER_COLOR,
            Self::Warning => WARNING_BORDER_COLOR,
            Self::Error => ERROR_BORDER_COLOR,
            Self::Disabled => DISABLED_BACKGROUND_COLOR,
            Self::Hovered => HOVERED_BACKGROUND_COLOR,
        }
    }

    pub(crate) const fn hint_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_HINT_COLOR,
            Self::Selected => DEFAULT_HINT_COLOR,
            Self::Warning => WARNING_HINT_COLOR,
            Self::Error => ERROR_HINT_COLOR,
            Self::Disabled => DISABLED_HINT_COLOR,
            Self::Hovered => DEFAULT_HINT_COLOR,
        }
    }

    pub(crate) const fn label_color(&self) -> Color {
        match self {
            Self::Warning => WARNING_HINT_COLOR,
            Self::Error => ERROR_HINT_COLOR,
            _ => DISABLED_HINT_COLOR,
        }
    }

    /// Determines if the current state represents a validation issue.
    ///
    /// # Returns
    ///
    /// * `true` if the state is either `Warning` or `Error`.
    /// * `false` otherwise.
    pub const fn validation_state(&self) -> bool {
        matches!(self, Self::Warning | Self::Error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Component, Reflect)]
#[reflect(Component)]
pub struct AllowedCharSet(pub BTreeSet<char>);

impl AllowedCharSet {
    pub fn new(input: Vec<char>) -> Self {
        Self(input.iter().copied().collect())
    }
    pub fn remove_invalid_chars(&self, input: &str) -> String {
        input.chars().filter(|c| self.0.contains(c)).collect()
    }

    pub fn has_invalid_chars(&self, input: &str) -> bool {
        input.chars().any(|c| !self.0.contains(&c))
    }

    pub fn has_invalid_char(&self, input: char) -> bool {
        self.0.contains(&input)
    }
}
