use bevy::{asset::load_internal_binary_asset, ecs::system::SystemParam, prelude::*};
use constants::CURSOR_HANDLE;
use systems::*;

pub mod builder;
pub mod constants;
pub mod systems;

/// A Bevy `Plugin` providing the systems and assets required to make a [`TextInput`] work.
pub struct TextFieldPlugin;

/// Label for systems that update text inputs.
#[derive(Debug, PartialEq, Eq, Clone, Hash, SystemSet)]
pub struct TextInputSystem;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {
        // This is a special font with a zero-width `|` glyph.
        load_internal_binary_asset!(
            app,
            CURSOR_HANDLE,
            "../../../../assets/Cursor.ttf",
            |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
        );

        app.init_resource::<TextInputNavigationBindings>()
            .add_event::<TextInputSubmitEvent>()
            .add_observer(create)
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
                    .in_set(TextInputSystem),
            )
            .register_type::<TextInputSettings>()
            .register_type::<TextInputTextColor>()
            .register_type::<TextInputTextFont>()
            .register_type::<TextInputInactive>()
            .register_type::<TextInputCursorTimer>()
            .register_type::<TextInputInner>()
            .register_type::<TextInputValue>()
            .register_type::<TextInputPlaceholder>()
            .register_type::<TextInputCursorPos>();
    }
}

/// Marker component for a Text Input entity.
///
/// Add this to a Bevy `NodeBundle`. In addition to its [required components](TextInput#impl-Component-for-TextInput), some other
/// components may also be spawned with it: [`TextInputCursorPos`].
///
/// # Example
///
/// ```rust
/// # use bevy::prelude::*;
/// use bevy_simple_text_input::TextInput;
/// fn setup(mut commands: Commands) {
///     commands.spawn((NodeBundle::default(), TextInput));
/// }
/// ```
#[derive(Component)]
#[require(
    TextInputSettings,
    TextInputTextColor,
    TextInputTextFont,
    TextInputInactive,
    TextInputCursorTimer,
    TextInputValue,
    TextInputPlaceholder,
    Interaction
)]
pub struct TextInput;

/// The Bevy `TextFont` that will be used when creating the text input's inner Bevy `TextBundle`.
#[derive(Component, Default, Reflect)]
pub struct TextInputTextFont(pub TextFont);

/// The Bevy `TextColor` that will be used when creating the text input's inner Bevy `TextBundle`.
#[derive(Component, Default, Reflect)]
pub struct TextInputTextColor(pub TextColor);

/// If true, the text input does not respond to keyboard events and the cursor is hidden.
#[derive(Component, Default, Reflect)]
pub struct TextInputInactive(pub bool);

/// A component that manages the cursor's blinking.
#[derive(Component, Reflect)]
pub struct TextInputCursorTimer {
    /// The timer that blinks the cursor on and off, and resets when the user types.
    pub timer: Timer,
    should_reset: bool,
}

impl Default for TextInputCursorTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            should_reset: false,
        }
    }
}

/// A component containing the text input's settings.
#[derive(Component, Default, Reflect)]
pub struct TextInputSettings {
    /// If true, text is not cleared after pressing enter.
    pub retain_on_submit: bool,
    /// Mask text with the provided character.
    pub mask_character: Option<char>,
}

/// Text navigation actions that can be bound via `TextInputNavigationBindings`.
#[derive(Debug)]
pub enum TextInputAction {
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
/// All modifiers must be held when the primary key is pressed to perform the action.
/// The first matching action in the list will be performed, so a binding that is the same as another with additional
/// modifier keys should be earlier in the vector to be applied.
#[derive(Resource)]
pub struct TextInputNavigationBindings(pub Vec<(TextInputAction, TextInputBinding)>);

/// A combination of a key and required modifier keys that might trigger a `TextInputAction`.
pub struct TextInputBinding {
    /// Primary key
    key: KeyCode,
    /// Required modifier keys
    modifiers: Vec<KeyCode>,
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
impl Default for TextInputNavigationBindings {
    fn default() -> Self {
        use KeyCode::*;
        use TextInputAction::*;
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
impl Default for TextInputNavigationBindings {
    fn default() -> Self {
        use KeyCode::*;
        use TextInputAction::*;
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

/// A component containing the current value of the text input.
#[derive(Component, Default, Reflect)]
pub struct TextInputValue(pub String);

/// A component containing the placeholder text that is displayed when the text input is empty and not focused.
#[derive(Component, Default, Reflect)]
pub struct TextInputPlaceholder {
    /// The placeholder text.
    pub value: String,
    /// The color to use when rendering the placeholder text.
    ///
    /// If `None`, the text input color will be used with alpha value of `0.25`.
    pub text_color: Option<TextColor>,
}

#[derive(Component, Reflect)]
pub struct TextInputPlaceholderInner;

/// A component containing the current text cursor position.
#[derive(Component, Default, Reflect)]
pub struct TextInputCursorPos(pub usize);

#[derive(Component, Reflect)]
pub struct TextInputInner;

/// An event that is fired when the user presses the enter key.
#[derive(Event)]
pub struct TextInputSubmitEvent {
    /// The text input that triggered the event.
    pub entity: Entity,
    /// The string contained in the text input at the time of the event.
    pub value: String,
}

/// A convenience parameter for dealing with a text input's inner Bevy `Text` entity.
#[derive(SystemParam)]
pub struct InnerText<'w, 's> {
    text_query: Query<'w, 's, (), With<TextInputInner>>,
    children_query: Query<'w, 's, &'static Children>,
}
impl<'w, 's> InnerText<'w, 's> {
    fn inner_entity(&self, entity: Entity) -> Option<Entity> {
        self.children_query
            .iter_descendants(entity)
            .find(|descendant_entity| self.text_query.get(*descendant_entity).is_ok())
    }
}
