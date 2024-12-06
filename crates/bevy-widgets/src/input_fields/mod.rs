use bevy::{asset::load_internal_binary_asset, ecs::system::SystemParam, prelude::*};
use builder::NumericFieldBuilder;
use components::{
    numeric::{NumericField, NumericFieldValue},
    text::{Placeholder, TextInputDescriptions},
    InputCursorTimer, InputFieldSettings, InputFieldState, InputInactive, InputTextColor,
    InputTextCursorPos, InputTextFont, InputTextValue, TextInputInner,
};
use constants::CURSOR_HANDLE;
use systems::*;

use crate::focus::Clickable;

/// Modelue containing auxiliary builder for text field widget
pub mod builder;
pub(crate) mod components;
pub(crate) mod constants;
mod systems;

pub use components::{InputFieldSize, InputFieldSubmitEvent};

/// A Bevy `Plugin` providing the systems and assets required to make a [`TextInput`] work.
pub struct InputFieldPlugin;

/// Label for systems that update text inputs.
#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct InputFieldSystemSet;

impl Plugin for InputFieldPlugin {
    fn build(&self, app: &mut App) {
        // This is a special font with a zero-width `|` glyph.
        load_internal_binary_asset!(
            app,
            CURSOR_HANDLE,
            "../../../../assets/Cursor.ttf",
            |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
        );

        app.init_resource::<InputTextNavigationBindings>()
            .add_event::<InputFieldSubmitEvent>()
            .add_observer(create_text_field)
            .add_observer(create_numeric_field)
            .add_observer(on_add_focus)
            .add_observer(on_remove_focus)
            .add_observer(mouse_over)
            .add_observer(mouse_out)
            .add_observer(mouse_move)
            .add_systems(
                Update,
                (
                    keyboard,
                    update_value.after(keyboard),
                    blink_cursor,
                    show_hide_cursor,
                    update_style,
                    show_hide_placeholder,
                    scroll_with_cursor,
                )
                    .in_set(InputFieldSystemSet),
            )
            .add_systems(PostUpdate, (on_error_validation, on_warning_validation))
            .add_systems(
                Update,
                (
                    on_numeric_text_changed::<i8>,
                    on_numeric_text_changed::<i16>,
                    on_numeric_text_changed::<i32>,
                    on_numeric_text_changed::<i64>,
                    on_numeric_text_changed::<i128>,
                    on_numeric_text_changed::<u8>,
                    on_numeric_text_changed::<u16>,
                    on_numeric_text_changed::<u32>,
                    on_numeric_text_changed::<u64>,
                    on_numeric_text_changed::<u128>,
                    on_numeric_text_changed::<f32>,
                    on_numeric_text_changed::<f64>,
                ),
            )
            .add_systems(
                Update,
                (
                    on_state_changed_text,
                    on_state_changed_numeric
                        .after(mouse_out)
                        .after(mouse_move)
                        .after(mouse_over),
                ),
            )
            .add_plugins(DragNumericPlugin)
            .register_type::<InputFieldSettings>()
            .register_type::<InputTextColor>()
            .register_type::<InputTextFont>()
            .register_type::<InputInactive>()
            .register_type::<InputCursorTimer>()
            .register_type::<TextInputInner>()
            .register_type::<InputTextValue>()
            .register_type::<InputFieldState>()
            .register_type::<TextInputDescriptions>()
            .register_type::<Placeholder>()
            .register_type::<InputFieldSize>()
            .register_type::<InputTextCursorPos>()
            .register_type::<NumericField<f32>>()
            .register_type::<NumericField<f64>>()
            .register_type::<NumericField<u8>>()
            .register_type::<NumericField<u16>>()
            .register_type::<NumericField<u32>>()
            .register_type::<NumericField<u64>>()
            .register_type::<NumericField<u128>>()
            .register_type::<NumericField<i8>>()
            .register_type::<NumericField<i16>>()
            .register_type::<NumericField<i32>>()
            .register_type::<NumericField<i64>>()
            .register_type::<NumericField<i128>>();
    }
}

/// Marker component for a Text Input entity.
///
/// Due to the amount of underlying configuration, shoulb be created with [`TextInputBuilder`]:
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_widgets::{input_fields::{*, builder::*}, WidgetsPlugin};
/// fn setup(mut commands: Commands) {
///     commands
///         .spawn(Node {
///             width: Val::Percent(100.0),
///             height: Val::Percent(100.0),
///             align_items: AlignItems::Center,
///             justify_content: JustifyContent::Center,
///             ..default()
///         })
///         .with_children(|parent| {
///             parent.spawn(
///                 TextInputBuilder::default()
///                     .with_size(InputFieldSize::Medium)
///                     .with_placeholder("placeholder".to_string())
///                     .with_hint_text("hint text".to_string())
///                     .with_label("label".to_string())
///                     .clear_on_submit()
///                     .build(),
///             );
///         });
/// }
/// ```
#[derive(Component)]
#[require(
    InputFieldSettings,
    InputTextColor,
    InputTextFont,
    InputInactive,
    InputCursorTimer,
    InputTextValue,
    InputFieldState,
    InputFieldSize,
    Placeholder,
    Clickable,
    TextInputDescriptions,
    Interaction
)]
pub struct TextInput;

/// Marker component for a Numeric Input entity.
///
/// Due to the amount of underlying configuration, shoulb be created with [`NumericFieldBuilder`]:
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_widgets::{input_fields::{*, builder::*}, WidgetsPlugin};
/// fn setup(mut commands: Commands) {
///     commands
///         .spawn(Node{
///             width: Val::Percent(100.0),
///             height: Val::Percent(100.0),
///             align_items: AlignItems::Center,
///             justify_content: JustifyContent::Center,
///             ..default()
///         })
///         .with_children(|parent| {
///            parent.spawn(
///                NumericFieldBuilder::default()
///                    .with_size(InputFieldSize::Medium)
///                    .with_initial_value(7)
///                    .with_range(0..10)
///                    .clear_on_submit()
///                    .build(),
///            );
///       });
/// }
/// ```
#[derive(Component)]
#[require(
    InputFieldSettings,
    InputTextColor,
    InputTextFont,
    InputInactive,
    InputCursorTimer,
    InputTextValue,
    InputFieldState,
    InputFieldSize,
    Clickable,
    Interaction
)]
pub struct NumericInput;

/// Marker component for the text input's label.
#[derive(Component, Default, Reflect)]
pub struct FixedTextLabel;

/// Text navigation actions that can be bound via `TextInputNavigationBindings`.
#[derive(Debug)]
pub enum InputTextAction {
    /// Moves the cursor one char to the left.
    CharLeft,
    /// Moves the cursor one char to the right.
    CharRight,
    /// Moves the cursor to the start of line.
    LineStart,
    /// Moves the cursor to the end of line.
    LineEnd,
    /// Moves the cursor one word to the left.
    WordLeft,
    /// Moves the cursor one word to the right.
    WordRight,
    /// Removes the char left of the cursor.
    DeletePrev,
    /// Removes the char right of the cursor.
    DeleteNext,
    /// Triggers a `TextInputSubmitEvent`, optionally clearing the text input.
    Submit,
}

/// A resource in which key bindings can be specified. Bindings are given as a tuple of (`TextInputAction`, `TextInputBinding`).
///
/// All modifiers must be held when the key is pressed to perform the action.
/// The first matching action in the list will be performed, so a binding that is the same as another with additional
/// modifier keys should be earlier in the vector to be applied.
#[derive(Resource)]
pub struct InputTextNavigationBindings(pub Vec<(InputTextAction, TextInputBinding)>);

/// A combination of a key and required modifier that might trigger a `TextInputAction`.
pub struct TextInputBinding {
    /// Key
    pub(crate) key: KeyCode,
    /// Required modifier
    pub(crate) modifiers: Vec<KeyCode>,
}

impl TextInputBinding {
    /// Creates a new `TextInputBinding` from a key and required modifiers.
    pub fn new(key: KeyCode, modifiers: impl Into<Vec<KeyCode>>) -> Self {
        Self {
            key,
            modifiers: modifiers.into(),
        }
    }
}

#[cfg(not(target_os = "macos"))]
impl Default for InputTextNavigationBindings {
    fn default() -> Self {
        use InputTextAction::*;
        use KeyCode::*;
        Self(vec![
            (LineStart, TextInputBinding::new(Home, [])),
            (LineEnd, TextInputBinding::new(End, [])),
            (WordLeft, TextInputBinding::new(ArrowLeft, [ControlLeft])),
            (WordLeft, TextInputBinding::new(ArrowLeft, [ControlRight])),
            (WordRight, TextInputBinding::new(ArrowRight, [ControlLeft])),
            (WordRight, TextInputBinding::new(ArrowRight, [ControlRight])),
            (CharLeft, TextInputBinding::new(ArrowLeft, [])),
            (CharRight, TextInputBinding::new(ArrowRight, [])),
            (DeletePrev, TextInputBinding::new(Backspace, [])),
            (DeletePrev, TextInputBinding::new(NumpadBackspace, [])),
            (DeleteNext, TextInputBinding::new(Delete, [])),
            (Submit, TextInputBinding::new(Enter, [])),
            (Submit, TextInputBinding::new(NumpadEnter, [])),
        ])
    }
}

#[cfg(target_os = "macos")]
impl Default for InputTextNavigationBindings {
    fn default() -> Self {
        use InputTextAction::*;
        use KeyCode::*;
        Self(vec![
            (LineStart, TextInputBinding::new(ArrowLeft, [SuperLeft])),
            (LineStart, TextInputBinding::new(ArrowLeft, [SuperRight])),
            (LineEnd, TextInputBinding::new(ArrowRight, [SuperLeft])),
            (LineEnd, TextInputBinding::new(ArrowRight, [SuperRight])),
            (WordLeft, TextInputBinding::new(ArrowLeft, [AltLeft])),
            (WordLeft, TextInputBinding::new(ArrowLeft, [AltRight])),
            (WordRight, TextInputBinding::new(ArrowRight, [AltLeft])),
            (WordRight, TextInputBinding::new(ArrowRight, [AltRight])),
            (CharLeft, TextInputBinding::new(ArrowLeft, [])),
            (CharRight, TextInputBinding::new(ArrowRight, [])),
            (DeletePrev, TextInputBinding::new(Backspace, [])),
            (DeletePrev, TextInputBinding::new(NumpadBackspace, [])),
            (DeleteNext, TextInputBinding::new(Delete, [])),
            (Submit, TextInputBinding::new(Enter, [])),
            (Submit, TextInputBinding::new(NumpadEnter, [])),
        ])
    }
}

/// A convenience parameter for dealing with a text input's inner Bevy `Text` entity.
#[derive(SystemParam)]
pub struct InnerText<'w, 's> {
    text_query: Query<'w, 's, (), With<TextInputInner>>,
    children_query: Query<'w, 's, &'static Children>,
}
impl InnerText<'_, '_> {
    fn inner_entity(&self, entity: Entity) -> Option<Entity> {
        self.children_query
            .iter_descendants(entity)
            .find(|descendant_entity| self.text_query.get(*descendant_entity).is_ok())
    }
}

/// A trait for spawning constrained numeric field.
pub trait SpawnNumericField<T> {
    /// Spawns a numeric field with the provided initial value and range.
    fn spawn_numeric_field(
        &mut self,
        initial_value: T,
        range: impl std::ops::RangeBounds<T>,
    ) -> Entity;
}

impl<T: NumericFieldValue> SpawnNumericField<T> for Commands<'_, '_> {
    fn spawn_numeric_field(
        &mut self,
        initial_value: T,
        range: impl std::ops::RangeBounds<T>,
    ) -> Entity {
        self.spawn(
            NumericFieldBuilder::default()
                .with_size(InputFieldSize::Medium)
                .with_initial_value(initial_value)
                .with_range(range)
                .clear_on_submit()
                .build(),
        )
        .id()
    }
}

impl<T: NumericFieldValue> SpawnNumericField<T> for ChildBuilder<'_> {
    fn spawn_numeric_field(
        &mut self,
        initial_value: T,
        range: impl std::ops::RangeBounds<T>,
    ) -> Entity {
        self.spawn(
            NumericFieldBuilder::default()
                .with_size(InputFieldSize::Medium)
                .with_initial_value(initial_value)
                .with_range(range)
                .clear_on_submit()
                .build(),
        )
        .id()
    }
}
