use std::ops::{Bound, RangeBounds};

use super::{
    components::{
        numeric::{NumericField, NumericFieldValue},
        text::{Placeholder, TextInputDescriptions},
        AllowedCharSet, InputFieldSize, InputFieldState,
    },
    InputFieldSettings, InputTextColor, InputTextFont, InputTextValue, NumericInput, TextInput,
};
use bevy::prelude::*;

/// Text input validation callback
pub type ValidationCallback = fn(&str) -> bool;
// pub type ValidationCallback2 = dyn Fn(&str) -> bool;

/// Text input warning validation callback component
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct WarningValidationCallback {
    /// callback function
    pub func: ValidationCallback,
    /// previous input state helper
    pub(crate) original_state: Option<InputFieldState>,
}

impl WarningValidationCallback {
    /// Creates a new `WarningValidationCallback` from a callback function.
    ///
    /// The callback function will be called each time the text input changes.
    /// The callback should return `true` if the text input is valid and `false`
    /// otherwise. If the callback returns `false` the text input component will
    /// be marked as invalid and the `Warning` style will be applied.
    pub fn new(func: ValidationCallback) -> Self {
        Self {
            func,
            original_state: None,
        }
    }
}

/// Text input error validation callback component
#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct ErrorValidationCallback {
    /// callback function
    pub func: ValidationCallback,
    /// previous input state helper
    pub(crate) original_state: Option<InputFieldState>,
}

impl ErrorValidationCallback {
    /// Creates a new `ErrorValidationCallback` from a callback function.
    ///
    /// The callback function will be called each time the text input changes.
    /// The callback should return `true` if the text input is valid and `false`
    /// otherwise. If the callback returns `false` the text input component will
    /// be marked as invalid and the `Error` style will be applied.
    pub fn new(func: ValidationCallback) -> Self {
        Self {
            func,
            original_state: None,
        }
    }
}

/// Builder for [`TextInput`]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TextInputBuilder {
    size: InputFieldSize,
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
            size: InputFieldSize::Medium,
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
    pub const fn with_size(mut self, size: InputFieldSize) -> Self {
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
        InputFieldSettings,
        InputTextColor,
        InputTextFont,
        InputTextValue,
        InputFieldState,
        InputFieldSize,
        Placeholder,
        TextInputDescriptions,
    ) {
        let settings = InputFieldSettings {
            retain_on_submit: self.retain_on_submit,
            mask_character: self.mask,
        };
        let color = InputTextColor(self.size.default_text_color());
        let font = InputTextFont(self.size.default_text_font());
        let value = InputTextValue(self.value.clone());
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
            InputFieldState::Default.border_color().into(),
            InputFieldState::Default.background_color().into(),
            BorderRadius::all(Val::Px(8.0)),
            settings,
            color,
            font,
            value,
            InputFieldState::Default,
            self.size,
            placeholder,
            extras,
        )
    }
}

/// Numeric field Builder
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NumericFieldBuilder<T: NumericFieldValue> {
    /// Current value
    pub(crate) value: Option<T>,
    /// Minimum allowed value
    pub(crate) min: Option<T>,
    /// Maximum allowed value
    pub(crate) max: Option<T>,
    /// End inclusive range
    pub(crate) end_inclusive: bool,
    /// Value change per logical pixel during mouse drag
    pub(crate) drag_step: Option<T>,
    /// Numeric field size
    pub(crate) size: InputFieldSize,
    /// Max allowed width for component
    max_width: Option<f32>,
    /// Min allowed width for component
    min_width: Option<f32>,
    /// Initial width for component
    width: Option<f32>,
    mask: Option<char>,
    retain_on_submit: bool,
}

impl<T: NumericFieldValue> Default for NumericFieldBuilder<T> {
    fn default() -> Self {
        Self {
            retain_on_submit: true,
            value: None,
            min: None,
            max: None,
            end_inclusive: false,
            drag_step: None,
            size: InputFieldSize::Medium,
            mask: None,
            max_width: None,
            min_width: None,
            width: None,
        }
    }
}

impl<T: NumericFieldValue> NumericFieldBuilder<T> {
    /// Sets the initial value of the numeric field.
    ///
    /// This method assigns the provided value as the initial value of the numeric field.
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value to set for the numeric field.
    ///
    /// # Returns
    ///
    /// Returns an updated instance of `NumericFieldBuilder` with the initial value set.
    pub const fn with_initial_value(mut self, value: T) -> Self {
        self.value = Some(value);
        self
    }

    /// Sets to clear numeric field on enter
    pub const fn clear_on_submit(mut self) -> Self {
        self.retain_on_submit = false;
        self
    }

    /// Sets the mask character
    pub const fn with_mask(mut self, mask: char) -> Self {
        self.mask = Some(mask);
        self
    }

    /// Sets the bounds of the numeric field, allowing you to specify a range of values which can be input into the field.
    ///
    /// The `range` parameter should be a type which implements the `RangeBounds` trait, such as `std::ops::Range<T>`
    /// or `std::ops::RangeInclusive<T>`.
    ///
    /// # Example
    /// - 0..=100
    /// - 0..100
    /// - ..=100
    /// - ..100
    /// - 0..
    /// - ..
    pub fn with_range(mut self, range: impl RangeBounds<T>) -> Self {
        self.min = match range.start_bound() {
            Bound::Excluded(min) | Bound::Included(min) => Some(*min),
            Bound::Unbounded => None,
        };
        self.max = match range.end_bound() {
            Bound::Excluded(max) | Bound::Included(max) => Some(*max),
            Bound::Unbounded => None,
        };
        self.end_inclusive = match range.end_bound() {
            Bound::Excluded(_) => false,
            Bound::Included(_) | Bound::Unbounded => true,
        };

        self
    }

    /// Sets the amount of change to the value when the user drags the numeric field (click and drag up or down).
    ///
    /// This is useful for creating a finer-grained control over the numeric field's value.
    pub const fn with_drag_step(mut self, drag_step: T) -> Self {
        self.drag_step = Some(drag_step);
        self
    }

    /// Sets the size of the numeric field.
    ///
    /// The size determines the font size, height and minimum width of the numeric field.
    pub const fn with_size(mut self, size: InputFieldSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the maximum allowed width of the numeric field component.
    ///
    /// If None is set, the component will have a maximum width of `3 * InputFieldSize`.
    pub const fn with_max_width(mut self, width: f32) -> Self {
        self.max_width = Some(width);
        self
    }

    /// Sets the minimum allowed width of the numeric field component.
    ///
    /// If None is set, the component will have a minimum width of `52.`.
    pub const fn with_min_width(mut self, width: f32) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Sets the initial width of the numeric field component.
    ///
    /// If None is set, the component will have the same initial width as the `self.min_width | 52`.
    pub const fn with_initial_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

/// Sets a fixed width for the numeric field component.
///
/// This method assigns the specified width as the fixed width, minimum width, and maximum width
/// for the numeric field component. This ensures that the component's width remains constant
/// regardless of other constraints or layout settings.
/// 
/// # Arguments
///
/// * `width` - The width to set for the numeric field component.
/// 
/// > Useful for when you have know boundaries.
    pub const fn with_fixed_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self.min_width = Some(width);
        self.max_width = Some(width);
        self
    }


    /// Builds the numeric field
    pub fn build(
        self,
    ) -> (
        NumericInput,
        AllowedCharSet,
        NumericField<T>,
        Node,
        BorderRadius,
        BorderColor,
        BackgroundColor,
        InputFieldSettings,
        InputTextColor,
        InputTextFont,
        InputTextValue,
        InputFieldSize,
        InputFieldState,
    ) {
        let numeric_field: NumericField<T> = self.into();
        let field_size = self.size;
        let state = InputFieldState::default();
        let settings = InputFieldSettings {
            retain_on_submit: self.retain_on_submit,
            mask_character: self.mask,
        };
        let color = InputTextColor(self.size.default_text_color());
        let font = InputTextFont(self.size.default_text_font());
        let value = InputTextValue(self.value.unwrap_or_default().to_string());
        let min_width = Val::Px(self.min_width.unwrap_or(52.));

        (
            NumericInput,
            T::allowed_chars(),
            numeric_field,
            Node {
                height: Val::Px(field_size.height()),
                min_width,
                max_width: Val::Px(
                    self.max_width
                        .unwrap_or_else(|| 3. * field_size.min_width()),
                ),
                width: self.width.map_or(min_width, Val::Px),
                border: UiRect::all(Val::Px(1.0)),
                padding: field_size.padding(false),
                justify_content: JustifyContent::End,
                ..default()
            },
            BorderRadius::all(Val::Px(8.0)),
            state.border_color().into(),
            state.background_color().into(),
            settings,
            color,
            font,
            value,
            field_size,
            state,
        )
    }
}
