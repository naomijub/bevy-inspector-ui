use crate::focus::Focus;

use super::constants::CURSOR_HANDLE;
use super::*;
use bevy::{
    ecs::event::EventCursor,
    input::keyboard::{Key, KeyboardInput},
    render::camera::RenderTarget,
    text::TextLayoutInfo,
    ui::FocusPolicy,
    window::{PrimaryWindow, WindowRef},
};

use builder::{ErrorValidationCallback, WarningValidationCallback};
use components::{
    numeric::{NumericDelta, NumericDeltaInitialValue, NumericField, NumericFieldValue},
    text::TextInputPlaceholderInner,
    AllowedCharSet,
};
use constants::{
    DEFAULT_BACKGROUND_COLOR, DISABLED_BACKGROUND_COLOR, ERROR_BACKGROUND_COLOR,
    ERROR_BORDER_COLOR, HOVERED_BACKGROUND_COLOR, SELECTED_BACKGROUND_COLOR, SELECTED_BORDER_COLOR,
    WARNING_BACKGROUND_COLOR, WARNING_BORDER_COLOR,
};

/// For custom numeric fields, you need to call this method after SystemSet [`InputFieldSystemSet`]
pub fn on_numeric_text_changed<T: NumericFieldValue>(
    mut text_input_query: Query<
        (&mut InputTextValue, &mut NumericField<T>),
        (Changed<InputTextValue>, With<AllowedCharSet>),
    >,
) {
    for (mut text, mut numeric) in text_input_query.iter_mut() {
        let current_numeric_value = numeric.value;
        if let Ok(numeric_value) = text.0.trim().parse() {
            numeric.set_value(numeric_value);
        } else {
            text.0 = current_numeric_value.to_string();
        }
    }
}

pub(super) fn keyboard(
    key_input: Res<ButtonInput<KeyCode>>,
    input_events: Res<Events<KeyboardInput>>,
    mut input_reader: Local<EventCursor<KeyboardInput>>,
    mut text_input_query: Query<
        (
            Entity,
            &InputFieldSettings,
            &InputInactive,
            &mut InputTextValue,
            &mut InputTextCursorPos,
            &mut InputCursorTimer,
            Option<&AllowedCharSet>,
        ),
        (Without<FixedTextLabel>, With<Focus>),
    >,
    mut submit_writer: EventWriter<InputFieldSubmitEvent>,
    navigation: Res<InputTextNavigationBindings>,
) {
    if input_reader.clone().read(&input_events).next().is_none() {
        return;
    }

    // collect actions that have all required modifiers held
    let valid_actions = navigation
        .0
        .iter()
        .filter(|(_, TextInputBinding { modifiers, .. })| {
            modifiers.iter().all(|m| key_input.pressed(*m))
        })
        .map(|(action, TextInputBinding { key, .. })| (*key, action));

    for (
        input_entity,
        settings,
        inactive,
        mut text_input,
        mut cursor_pos,
        mut cursor_timer,
        char_set,
    ) in &mut text_input_query
    {
        if inactive.0 {
            continue;
        }

        let mut submitted_value = None;

        for input in input_reader.clone().read(&input_events) {
            if !input.state.is_pressed() {
                continue;
            };

            let pos = cursor_pos.bypass_change_detection().0;

            if let Some((_, action)) = valid_actions
                .clone()
                .find(|(key, _)| *key == input.key_code)
            {
                use InputTextAction::*;
                let mut timer_should_reset = true;
                match action {
                    CharLeft => cursor_pos.0 = cursor_pos.0.saturating_sub(1),
                    CharRight => cursor_pos.0 = (cursor_pos.0 + 1).min(text_input.0.len()),
                    LineStart => cursor_pos.0 = 0,
                    LineEnd => cursor_pos.0 = text_input.0.len(),
                    WordLeft => {
                        cursor_pos.0 = text_input
                            .0
                            .char_indices()
                            .rev()
                            .skip(text_input.0.len() - cursor_pos.0 + 1)
                            .skip_while(|c| c.1.is_ascii_whitespace())
                            .find(|c| c.1.is_ascii_whitespace())
                            .map(|(ix, _)| ix + 1)
                            .unwrap_or(0)
                    }
                    WordRight => {
                        cursor_pos.0 = text_input
                            .0
                            .char_indices()
                            .skip(cursor_pos.0)
                            .skip_while(|c| !c.1.is_ascii_whitespace())
                            .find(|c| !c.1.is_ascii_whitespace())
                            .map(|(ix, _)| ix)
                            .unwrap_or(text_input.0.len())
                    }
                    DeletePrev => {
                        if pos > 0 {
                            cursor_pos.0 -= 1;
                            text_input.0 = remove_char_at(&text_input.0, cursor_pos.0);
                        }
                    }
                    DeleteNext => {
                        if pos < text_input.0.len() {
                            text_input.0 = remove_char_at(&text_input.0, cursor_pos.0);

                            // Ensure that the cursor isn't reset
                            cursor_pos.set_changed();
                        }
                    }
                    Submit => {
                        if settings.retain_on_submit {
                            submitted_value = Some(text_input.0.clone());
                        } else {
                            submitted_value = Some(std::mem::take(&mut text_input.0));
                            cursor_pos.0 = 0;
                        };
                        timer_should_reset = false;
                    }
                }

                cursor_timer.should_reset |= timer_should_reset;
                continue;
            }

            match input.logical_key {
                Key::Space => {
                    if char_set.map_or(true, |chars| !chars.has_invalid_char(' ')) {
                        let byte_pos = byte_pos(&text_input.0, pos);
                        text_input.0.insert(byte_pos, ' ');
                        cursor_pos.0 += 1;

                        cursor_timer.should_reset = true;
                    }
                }
                Key::Character(ref s) => {
                    if char_set.map_or(true, |chars| !chars.has_invalid_chars(s)) {
                        let byte_pos = byte_pos(&text_input.0, pos);
                        text_input.0.insert_str(byte_pos, s.as_str());

                        cursor_pos.0 += 1;

                        cursor_timer.should_reset = true;
                    }
                }
                _ => (),
            }
        }

        if let Some(value) = submitted_value {
            submit_writer.send(InputFieldSubmitEvent {
                entity: input_entity,
                value,
            });
        }
    }

    input_reader.clear(&input_events);
}

pub(super) fn update_value(
    mut input_query: Query<
        (
            Entity,
            Ref<InputTextValue>,
            &InputFieldSettings,
            &mut InputTextCursorPos,
        ),
        (
            Without<FixedTextLabel>,
            Or<(Changed<InputTextValue>, Changed<InputTextCursorPos>)>,
        ),
    >,
    inner_text: InnerText,
    mut writer: TextUiWriter,
) {
    for (entity, text_input, settings, mut cursor_pos) in &mut input_query {
        let Some(inner) = inner_text.inner_entity(entity) else {
            continue;
        };

        // Reset the cursor to the end of the input when the value is changed by
        // a user manipulating the value component.
        if text_input.is_changed() && !cursor_pos.is_changed() {
            cursor_pos.0 = text_input.0.chars().count();
        }

        if cursor_pos.is_changed() {
            cursor_pos.0 = cursor_pos.0.clamp(0, text_input.0.chars().count());
        }

        let values = get_section_values(
            &masked_value(&text_input.0, settings.mask_character),
            cursor_pos.0,
        );

        *writer.text(inner, 0) = values.0;
        *writer.text(inner, 1) = values.1;
        *writer.text(inner, 2) = values.2;
    }
}

pub(super) fn scroll_with_cursor(
    mut inner_text_query: Query<
        (&TextLayoutInfo, &mut Node, &Parent, Option<&TargetCamera>),
        (
            With<TextInputInner>,
            Without<FixedTextLabel>,
            Changed<TextLayoutInfo>,
        ),
    >,
    mut style_query: Query<&mut Node, (Without<FixedTextLabel>, Without<TextInputInner>)>,
    camera_query: Query<&Camera>,
    window_query: Query<&Window>,
    primary_window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (layout, mut child_style, parent, target_camera) in inner_text_query.iter_mut() {
        let Ok(mut parent_node) = style_query.get_mut(parent.get()) else {
            continue;
        };

        match layout.glyphs.last().map(|g| g.span_index) {
            // no text -> do nothing
            None => return,
            // if cursor is at the end, position at FlexEnd so newly typed text does not take a frame to move into view
            Some(1) => {
                child_style.left = Val::Auto;
                parent_node.justify_content = JustifyContent::FlexEnd;
                return;
            }
            _ => (),
        }

        // if cursor is in the middle, we use FlexStart + `left` px for consistent behaviour when typing the middle
        let child_size = child_style.width;
        let parent_size = parent_node.width;

        let Some(cursor_pos) = layout
            .glyphs
            .iter()
            .find(|g| g.span_index == 1)
            .map(|p| p.position.x)
        else {
            continue;
        };

        // glyph positions are not adjusted for scale factor so we do that here
        let window_ref = match target_camera {
            Some(target) => {
                let Ok(camera) = camera_query.get(target.0) else {
                    continue;
                };

                match camera.target {
                    RenderTarget::Window(window_ref) => Some(window_ref),
                    _ => None,
                }
            }
            None => Some(WindowRef::Primary),
        };

        let scale_factor = match window_ref {
            Some(window_ref) => {
                let window = match window_ref {
                    WindowRef::Entity(w) => window_query.get(w).ok(),
                    WindowRef::Primary => primary_window_query.get_single().ok(),
                };

                let Some(window) = window else {
                    continue;
                };

                window.scale_factor()
            }
            None => 1.0,
        };
        let cursor_pos = cursor_pos / scale_factor;

        let box_pos = match child_style.left {
            Val::Px(px) => -px,
            _ => match (child_size, parent_size) {
                (Val::Px(child), Val::Px(parent)) => child - parent,
                _ => 0.,
            },
        };

        let relative_pos = cursor_pos - box_pos;

        if let (Val::Px(parent_size), Val::Px(child_size)) = (parent_size, child_size) {
            if relative_pos < 0.0 || relative_pos > parent_size {
                let req_px = parent_size.mul_add(0.5, -cursor_pos);
                let req_px = req_px.clamp(parent_size - child_size, 0.0);
                child_style.left = Val::Px(req_px);
                parent_node.justify_content = JustifyContent::FlexStart;
            }
        }
    }
}

pub(super) fn create_numeric_field(
    trigger: Trigger<OnAdd, NumericInput>,
    mut commands: Commands,
    query: Query<(
        Entity,
        &InputTextFont,
        &InputTextColor,
        &InputTextValue,
        Option<&InputTextCursorPos>,
        &InputInactive,
        &InputFieldSettings,
    )>,
) {
    if let Ok((entity, font, color, text_input, maybe_cursor_pos, inactive, settings)) =
        &query.get(trigger.entity())
    {
        #[expect(clippy::option_if_let_else)]
        // Internal mutation
        let cursor_pos = match maybe_cursor_pos {
            None => {
                let len = text_input.0.len();
                commands.entity(*entity).insert(InputTextCursorPos(len));
                len
            }
            Some(cursor_pos) => cursor_pos.0,
        };

        let values = get_section_values(
            &masked_value(&text_input.0, settings.mask_character),
            cursor_pos,
        );

        let text = commands
            .spawn((
                Text::default(),
                TextLayout::new_with_linebreak(LineBreak::NoWrap),
                Name::new("NumericInputInner"),
                TextInputInner,
            ))
            .with_children(|parent| {
                parent.spawn((TextSpan::new(values.0), font.0.clone(), color.0));

                parent.spawn((
                    TextSpan::new(values.1),
                    TextFont {
                        font: CURSOR_HANDLE,
                        ..font.0.clone()
                    },
                    if inactive.0 {
                        Color::NONE.into()
                    } else {
                        color.0
                    },
                ));

                parent.spawn((TextSpan::new(values.2), font.0.clone(), color.0));
            })
            .id();

        let overflow_container = commands
            .spawn((
                Node {
                    overflow: Overflow::clip_x(),
                    justify_content: JustifyContent::FlexEnd,
                    flex_direction: FlexDirection::Column,
                    max_width: Val::Percent(100.),
                    ..default()
                },
                FocusPolicy::Pass,
                PickingBehavior::IGNORE,
                Name::new("NumericInputOverflowContainer"),
            ))
            .id();

        commands.entity(overflow_container).add_child(text);
        commands
            .entity(trigger.entity())
            .add_child(overflow_container);
        // Prevent clicks from registering on UI elements underneath the text input.
        commands.entity(trigger.entity()).insert(FocusPolicy::Block);
    }
}

pub(super) fn create_text_field(
    trigger: Trigger<OnAdd, TextInput>,
    mut commands: Commands,
    query: Query<(
        Entity,
        &InputTextFont,
        &InputTextColor,
        &InputTextValue,
        Option<&InputTextCursorPos>,
        &InputInactive,
        &InputFieldSettings,
        &Placeholder,
        &InputFieldSize,
        &InputFieldState,
        &TextInputDescriptions,
    )>,
) {
    if let Ok((
        entity,
        font,
        color,
        text_input,
        maybe_cursor_pos,
        inactive,
        settings,
        placeholder,
        text_input_size,
        text_state,
        extras,
    )) = &query.get(trigger.entity())
    {
        #[expect(clippy::option_if_let_else)]
        // Internal mutation
        let cursor_pos = match maybe_cursor_pos {
            None => {
                let len = text_input.0.len();
                commands.entity(*entity).insert(InputTextCursorPos(len));
                len
            }
            Some(cursor_pos) => cursor_pos.0,
        };

        let values = get_section_values(
            &masked_value(&text_input.0, settings.mask_character),
            cursor_pos,
        );

        let text = commands
            .spawn((
                Text::default(),
                TextLayout::new_with_linebreak(LineBreak::NoWrap),
                Name::new("TextInputInner"),
                TextInputInner,
            ))
            .with_children(|parent| {
                parent.spawn((TextSpan::new(values.0), font.0.clone(), color.0));

                parent.spawn((
                    TextSpan::new(values.1),
                    TextFont {
                        font: CURSOR_HANDLE,
                        ..font.0.clone()
                    },
                    if inactive.0 {
                        Color::NONE.into()
                    } else {
                        color.0
                    },
                ));

                parent.spawn((TextSpan::new(values.2), font.0.clone(), color.0));
            })
            .id();

        let placeholder_visible = inactive.0 && text_input.0.is_empty();

        let placeholder_text = commands
            .spawn((
                Text::new(&placeholder.0),
                TextLayout::new_with_linebreak(LineBreak::NoWrap),
                FocusPolicy::Pass,
                PickingBehavior::IGNORE,
                Placeholder::text_color(),
                Placeholder::text_font(text_input_size),
                Name::new("TextInputPlaceholderInner"),
                TextInputPlaceholderInner,
                if placeholder_visible {
                    Visibility::Inherited
                } else {
                    Visibility::Hidden
                },
                Node {
                    position_type: PositionType::Absolute,
                    bottom: text_input_size.padding(extras.label.is_some()).bottom,
                    ..default()
                },
            ))
            .id();

        let overflow_container = commands
            .spawn((
                Node {
                    overflow: Overflow::clip_x(),
                    justify_content: JustifyContent::FlexEnd,
                    flex_direction: FlexDirection::Column,
                    max_width: Val::Percent(100.),
                    ..default()
                },
                FocusPolicy::Pass,
                PickingBehavior::IGNORE,
                Name::new("TextInputOverflowContainer"),
            ))
            .id();

        commands.entity(overflow_container).add_child(text);
        commands
            .entity(trigger.entity())
            .add_children(&[overflow_container, placeholder_text]);
        // Prevent clicks from registering on UI elements underneath the text input.
        commands.entity(trigger.entity()).insert(FocusPolicy::Block);

        if let Some(hint) = &&extras.hint {
            let hint_id = commands
                .spawn((
                    Text::new(hint),
                    TextLayout::new_with_linebreak(LineBreak::NoWrap),
                    Name::new("TextInputHint"),
                    TextColor(text_state.hint_color()),
                    FixedTextLabel,
                    TextFont {
                        font_size: text_input_size.hint_font_size(),
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(0.),
                        top: Val::Px(
                            text_input_size.height() + text_input_size.hint_text_spacing(),
                        ),
                        ..default()
                    },
                ))
                .id();
            commands.entity(trigger.entity()).add_child(hint_id);
        };
        if let Some(label) = &&extras.label {
            let label_id = commands
                .spawn((
                    Text::new(label),
                    TextLayout::new_with_linebreak(LineBreak::NoWrap),
                    Name::new("TextInputLabel"),
                    TextColor(text_state.label_color()),
                    FixedTextLabel,
                    PickingBehavior::IGNORE,
                    FocusPolicy::Pass,
                    TextFont {
                        font_size: text_input_size.label_font_size(),
                        ..default()
                    },
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(16.),
                        top: Val::Px(if text_input_size.is_large() { 4. } else { 2. }),
                        ..default()
                    },
                ))
                .id();
            commands.entity(trigger.entity()).add_child(label_id);
        };
    }
}

// Shows or hides the cursor based on the text input's [`TextInputInactive`] property.
pub(super) fn show_hide_cursor(
    mut input_query: Query<
        (
            Entity,
            &InputTextColor,
            &mut InputCursorTimer,
            &InputInactive,
        ),
        Changed<InputInactive>,
    >,
    inner_text: InnerText,
    mut writer: TextUiWriter,
) {
    for (entity, color, mut cursor_timer, inactive) in &mut input_query {
        let Some(inner) = inner_text.inner_entity(entity) else {
            continue;
        };

        *writer.color(inner, 1) = if inactive.0 {
            Color::NONE.into()
        } else {
            color.0
        };

        cursor_timer.timer.reset();
    }
}

// Blinks the cursor on a timer.
pub(super) fn blink_cursor(
    mut input_query: Query<(
        Entity,
        &InputTextColor,
        &mut InputCursorTimer,
        Ref<InputInactive>,
    )>,
    inner_text: InnerText,
    mut writer: TextUiWriter,
    time: Res<Time>,
) {
    for (entity, color, mut cursor_timer, inactive) in &mut input_query {
        if inactive.0 {
            continue;
        }

        if cursor_timer.is_changed() && cursor_timer.should_reset {
            cursor_timer.timer.reset();
            cursor_timer.should_reset = false;

            if let Some(inner) = inner_text.inner_entity(entity) {
                *writer.color(inner, 1) = color.0;
            };

            continue;
        }

        if !cursor_timer.timer.tick(time.delta()).just_finished() {
            continue;
        }

        let Some(inner) = inner_text.inner_entity(entity) else {
            continue;
        };

        if writer.color(inner, 1).0 != Color::NONE {
            *writer.color(inner, 1) = Color::NONE.into();
        } else {
            *writer.color(inner, 1) = color.0;
        }
    }
}

pub(super) fn show_hide_placeholder(
    input_query: Query<
        (&Children, &InputTextValue, &InputInactive),
        Or<(Changed<InputTextValue>, Changed<InputInactive>)>,
    >,
    mut vis_query: Query<&mut Visibility, With<TextInputPlaceholderInner>>,
) {
    for (children, text, inactive) in &input_query {
        let mut iter = vis_query.iter_many_mut(children);
        while let Some(mut inner_vis) = iter.fetch_next() {
            inner_vis.set_if_neq(if text.0.is_empty() && inactive.0 {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            });
        }
    }
}

pub(super) fn update_style(
    mut input_query: Query<
        (Entity, &InputTextFont, &InputTextColor, &InputInactive),
        (Changed<InputTextFont>, Changed<InputTextColor>),
    >,
    inner_text: InnerText,
    mut writer: TextUiWriter,
) {
    for (entity, font, color, inactive) in &mut input_query {
        let Some(inner) = inner_text.inner_entity(entity) else {
            continue;
        };

        *writer.color(inner, 0) = color.0;
        *writer.font(inner, 0) = font.0.clone();
        *writer.color(inner, 1) = if inactive.0 {
            Color::NONE.into()
        } else {
            color.0
        };
        *writer.font(inner, 1) = TextFont {
            font: CURSOR_HANDLE,
            ..font.0.clone()
        };
        *writer.color(inner, 2) = color.0;
        *writer.font(inner, 2) = font.0.clone();
    }
}

pub(super) fn get_section_values(value: &str, cursor_pos: usize) -> (String, String, String) {
    let before = value.chars().take(cursor_pos).collect();
    let after = value.chars().skip(cursor_pos).collect();

    // If the cursor is between two characters, use the zero-width cursor.
    let cursor = if cursor_pos >= value.chars().count() {
        "}".to_string()
    } else {
        "|".to_string()
    };

    (before, cursor, after)
}

pub(crate) fn remove_char_at(input: &str, index: usize) -> String {
    input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if i != index { Some(c) } else { None })
        .collect()
}

pub(crate) fn byte_pos(input: &str, char_pos: usize) -> usize {
    let mut char_indices = input.char_indices();
    char_indices
        .nth(char_pos)
        .map(|(pos, _)| pos)
        .unwrap_or(input.len())
}

pub(super) fn masked_value(value: &str, mask: Option<char>) -> String {
    mask.map_or_else(
        || value.to_string(),
        |c| value.chars().map(|_| c).collect::<String>(),
    )
}

pub(super) fn placeholder_color(color: &TextColor) -> TextColor {
    let color = color.with_alpha(color.alpha() * 0.25);
    TextColor(color)
}

pub(super) fn on_add_focus(
    trigger: Trigger<OnAdd, Focus>,
    mut commands: Commands,
    mut interaction_query: Query<
        (&mut InputInactive, &mut InputFieldState),
        Or<(With<TextInput>, With<NumericInput>)>,
    >,
) {
    let entity = trigger.entity();
    if let Ok((mut inactive, mut state)) = interaction_query.get_mut(entity) {
        inactive.active();
        *state = InputFieldState::Selected;
    } else {
        commands.entity(entity).remove::<Focus>();
    }
}

pub(super) fn on_remove_focus(
    trigger: Trigger<OnRemove, Focus>,
    mut commands: Commands,
    mut interaction_query: Query<
        (&mut InputInactive, &mut InputFieldState),
        Or<(With<TextInput>, With<NumericInput>)>,
    >,
) {
    let entity = trigger.entity();
    if let Ok((mut inactive, mut state)) = interaction_query.get_mut(entity) {
        inactive.inactive();
        *state = InputFieldState::Default;
    } else {
        commands.entity(entity).remove::<Focus>();
    }
}

pub(super) fn on_state_changed_text(
    mut interaction_query: Query<
        (
            &InputInactive,
            &InputFieldState,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<InputFieldState>, With<TextInput>),
    >,
) {
    for (inactive, state, mut bg, mut border) in &mut interaction_query {
        match (state, inactive.0) {
            (InputFieldState::Default, true) => {
                *bg = DEFAULT_BACKGROUND_COLOR.into();
                *border = DEFAULT_BACKGROUND_COLOR.into();
            }
            (InputFieldState::Selected, false) => {
                *bg = SELECTED_BACKGROUND_COLOR.into();
                *border = SELECTED_BORDER_COLOR.into();
            }
            (InputFieldState::Warning, _) => {
                *bg = WARNING_BACKGROUND_COLOR.into();
                *border = WARNING_BORDER_COLOR.into();
            }
            (InputFieldState::Error, _) => {
                *bg = ERROR_BACKGROUND_COLOR.into();
                *border = ERROR_BORDER_COLOR.into();
            }
            (InputFieldState::Disabled, _) => {
                *bg = DISABLED_BACKGROUND_COLOR.into();
                *border = DISABLED_BACKGROUND_COLOR.into();
            }
            _ => {}
        }
    }
}

pub(super) fn on_state_changed_numeric(
    mut interaction_query: Query<
        (
            &InputInactive,
            &InputFieldState,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<InputFieldState>, With<NumericInput>),
    >,
) {
    for (inactive, state, mut bg, mut border) in &mut interaction_query {
        match (state, inactive.0) {
            (InputFieldState::Default, true) => {
                *bg = DEFAULT_BACKGROUND_COLOR.into();
                *border = DEFAULT_BACKGROUND_COLOR.into();
            }
            (InputFieldState::Selected, false) => {
                *bg = SELECTED_BACKGROUND_COLOR.into();
                *border = SELECTED_BORDER_COLOR.into();
            }
            (InputFieldState::Disabled, _) => {
                *bg = DISABLED_BACKGROUND_COLOR.into();
                *border = DISABLED_BACKGROUND_COLOR.into();
            }
            (InputFieldState::Hovered, true) => {
                *bg = HOVERED_BACKGROUND_COLOR.into();
                *border = HOVERED_BACKGROUND_COLOR.into();
            }
            _ => {}
        }
    }
}

pub(super) fn on_error_validation(
    mut interaction_query: Query<
        (
            &Children,
            &mut ErrorValidationCallback,
            &InputTextValue,
            &mut InputFieldState,
        ),
        (Changed<InputFieldState>, With<TextInput>),
    >,
) {
    for (_, mut callback, new_value, mut state) in interaction_query.iter_mut() {
        let s = &new_value.0;
        if (callback.func)(s) {
            if callback.original_state.is_none() {
                callback.original_state = Some(*state);
            }
            *state = InputFieldState::Error;
        } else if let (Some(original_state), false) =
            (callback.original_state, state.validation_state())
        {
            *state = original_state;
        }
    }
}

pub(super) fn on_warning_validation(
    mut interaction_query: Query<
        (
            &Children,
            &mut WarningValidationCallback,
            &InputTextValue,
            &mut InputFieldState,
        ),
        (Changed<InputFieldState>, With<TextInput>),
    >,
) {
    for (_, mut callback, new_value, mut state) in interaction_query.iter_mut() {
        let s = &new_value.0;
        if (callback.func)(s) {
            if callback.original_state.is_none() {
                callback.original_state = Some(*state);
            }
            *state = InputFieldState::Warning;
        } else if let (Some(original_state), false) =
            (callback.original_state, state.validation_state())
        {
            *state = original_state;
        }
    }
}

#[derive(Component)]
pub(super) struct PreviousInputState(InputFieldState);

pub(super) fn mouse_over(
    mut click: Trigger<Pointer<Over>>,
    mut commands: Commands,
    clickable_entities: Query<Entity, With<Clickable>>,
    mut interaction_query: Query<(&mut InputFieldState, &InputInactive), Without<Focus>>,
) {
    let entity = click.entity();
    if clickable_entities.contains(entity) {
        click.propagate(false);

        if let Ok((mut state, &InputInactive(true))) = interaction_query.get_mut(entity) {
            if *state != InputFieldState::Hovered {
                commands.entity(entity).insert(PreviousInputState(*state));
            }
            *state = InputFieldState::Hovered;
        }
    }
}

pub(super) fn mouse_move(
    mut click: Trigger<Pointer<Move>>,
    mut commands: Commands,
    clickable_entities: Query<Entity, With<Clickable>>,
    mut interaction_query: Query<(&mut InputFieldState, &InputInactive), Without<Focus>>,
) {
    let entity = click.entity();
    if clickable_entities.contains(entity) {
        click.propagate(false);

        if let Ok((mut state, &InputInactive(true))) = interaction_query.get_mut(entity) {
            if *state != InputFieldState::Hovered {
                commands.entity(entity).insert(PreviousInputState(*state));
            } else {
                *state = InputFieldState::Hovered;
            }
        }
    }
}

pub(super) fn mouse_out(
    mut click: Trigger<Pointer<Out>>,
    mut commands: Commands,
    clickable_entities: Query<Entity, With<Clickable>>,
    mut interaction_query: Query<(&mut InputFieldState, Option<&PreviousInputState>)>,
) {
    let entity = click.entity();
    if clickable_entities.contains(entity) {
        click.propagate(false);

        if let Ok((mut state, previous_state)) = interaction_query.get_mut(entity) {
            if let Some(previous_state) = previous_state {
                *state = previous_state.0;
                commands.entity(entity).remove::<PreviousInputState>();
            } else {
                *state = InputFieldState::Default;
            }
        }
    }
}

pub fn on_drag<T: NumericFieldValue>(
    trigger: Trigger<Pointer<Drag>>,
    mut q_fields: Query<(&mut NumericField<T>, &mut NumericDelta, &mut InputTextValue)>,
) {
    let event_delta = trigger.delta.normalize();
    let entity = trigger.entity();
    if let Ok((mut field, mut delta, mut value)) = q_fields.get_mut(entity) {
        if let Some(drag_delta) = field.drag_step.unwrap_or_default().to_f64() {
            delta.accumulated_delta +=
                drag_delta.mul_add(event_delta.x as f64, drag_delta * event_delta.y as f64);
            let new_value = field.value
                - field.drag_step.unwrap_or_default() * T::from(event_delta.y).unwrap_or_default()
                + field.drag_step.unwrap_or_default() * T::from(event_delta.x).unwrap_or_default();
            field.set_value(new_value);
            value.0 = field.value.to_string();
        }
    }
}

pub fn on_drag_start<T: NumericFieldValue>(
    trigger: Trigger<Pointer<DragStart>>,
    mut commands: Commands,
    q_fields: Query<&NumericField<T>, (With<NumericDelta>, With<InputTextValue>)>,
) {
    let entity = trigger.entity();
    if let Ok(field) = q_fields.get(entity) {
        let initial_value = NumericDeltaInitialValue {
            initial_value: field.value,
        };
        commands.entity(entity).insert(initial_value);
    }
}

pub fn on_drag_end<T: NumericFieldValue>(
    trigger: Trigger<Pointer<DragEnd>>,
    mut commands: Commands,
    mut q_fields: Query<
        &mut NumericDelta,
        (
            With<NumericDeltaInitialValue<T>>,
            With<NumericField<T>>,
            With<InputTextValue>,
        ),
    >,
) {
    let entity = trigger.entity();
    if let Ok(mut delta) = q_fields.get_mut(entity) {
        delta.accumulated_delta = 0.0;
        commands
            .entity(entity)
            .remove::<NumericDeltaInitialValue<T>>();
    }
}

pub fn on_drag_exit<T: NumericFieldValue>(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut q_fields: Query<(
        Entity,
        &mut NumericField<T>,
        &mut NumericDelta,
        &mut InputTextValue,
        &NumericDeltaInitialValue<T>,
    )>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        for (entity, mut field, mut delta, mut text, initial_value) in q_fields.iter_mut() {
            delta.accumulated_delta = 0.0;
            commands
                .entity(entity)
                .remove::<NumericDeltaInitialValue<T>>();
            field.set_value(initial_value.initial_value);
            text.0 = field.value.to_string();
        }
    }
}

pub struct DragNumericPlugin;
impl Plugin for DragNumericPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                on_drag_exit::<f32>,
                on_drag_exit::<f64>,
                on_drag_exit::<u8>,
                on_drag_exit::<u16>,
                on_drag_exit::<u32>,
                on_drag_exit::<u64>,
                on_drag_exit::<u128>,
                on_drag_exit::<i8>,
                on_drag_exit::<i16>,
                on_drag_exit::<i32>,
                on_drag_exit::<i64>,
                on_drag_exit::<i128>,
            ),
        )
        .add_observer(on_drag::<f32>)
        .add_observer(on_drag_start::<f32>)
        .add_observer(on_drag_end::<f32>)
        .add_observer(on_drag::<f64>)
        .add_observer(on_drag_start::<f64>)
        .add_observer(on_drag_end::<f64>)
        .add_observer(on_drag::<u8>)
        .add_observer(on_drag_start::<u8>)
        .add_observer(on_drag_end::<u8>)
        .add_observer(on_drag::<u16>)
        .add_observer(on_drag_start::<u16>)
        .add_observer(on_drag_end::<u16>)
        .add_observer(on_drag::<u32>)
        .add_observer(on_drag_start::<u32>)
        .add_observer(on_drag_end::<u32>)
        .add_observer(on_drag::<u64>)
        .add_observer(on_drag_start::<u64>)
        .add_observer(on_drag_end::<u64>)
        .add_observer(on_drag::<u128>)
        .add_observer(on_drag_start::<u128>)
        .add_observer(on_drag_end::<u128>)
        .add_observer(on_drag::<i8>)
        .add_observer(on_drag_start::<i8>)
        .add_observer(on_drag_end::<i8>)
        .add_observer(on_drag::<i16>)
        .add_observer(on_drag_start::<i16>)
        .add_observer(on_drag_end::<i16>)
        .add_observer(on_drag::<i32>)
        .add_observer(on_drag_start::<i32>)
        .add_observer(on_drag_end::<i32>)
        .add_observer(on_drag::<i64>)
        .add_observer(on_drag_start::<i64>)
        .add_observer(on_drag_end::<i64>)
        .add_observer(on_drag::<i128>)
        .add_observer(on_drag_start::<i128>)
        .add_observer(on_drag_end::<i128>);
    }
}
