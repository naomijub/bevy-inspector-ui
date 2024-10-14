use bevy_color::Color;
use bevy_ecs::observer::Trigger;
use bevy_ecs::query::Changed;
use bevy_ecs::world::OnRemove;
use bevy_ecs::{
    query::{With, Without},
    system::{Commands, Query},
};
use bevy_hierarchy::{BuildChildren, Children, DespawnRecursiveExt};
use bevy_render::view::Visibility;
use bevy_ui::widget::Text;
use bevy_ui::{BorderColor, Node};

use crate::cursor::Cursor;
use crate::focus::{Clickable, Focus, GotFocus, LostFocus};

use super::{
    Placeholder, SingleLineTextField, TextField, TextFieldSize, DEFAULT_BACKGROUND_COLOR,
    SELECTED_BORDER_COLOR,
};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

pub fn focus_system(
    mut text_with_focus: Query<
        (&mut BorderColor, Option<&Focus>),
        (Changed<Focus>, With<SingleLineTextField>),
    >,
) {
    for (mut border_color, has_focus) in &mut text_with_focus {
        if has_focus.is_some() {
            border_color.0 = SELECTED_BORDER_COLOR;
        }
    }
}

pub fn react_on_removal(
    trigger: Trigger<OnRemove, Focus>,
    mut text_with_focus: Query<(&mut BorderColor, Option<&Focus>), With<SingleLineTextField>>,
) {
    let entity = trigger.entity();
    if let Ok((mut border_color, Some(_))) = text_with_focus.get_mut(entity) {
        border_color.0 = DEFAULT_BACKGROUND_COLOR;
    }
}

pub(crate) fn text_field_lost_focus(
    click: Trigger<LostFocus>,
    mut commands: Commands,
    text_fields: Query<(&Children, &TextFieldSize), (With<SingleLineTextField>, With<Clickable>)>,
    placeholder: Query<&Placeholder>,
    mut input_text: Query<
        (&mut Visibility, Option<&Text>),
        (With<TextField>, Without<Placeholder>),
    >,
) {
    let entity = click.entity();
    let Ok((children_entities, size)) = text_fields.get(entity) else {
        return;
    };

    let mut should_add_place_holder = false;
    for child in children_entities.iter() {
        if let Ok((mut input_visibility, input_text)) = input_text.get_mut(*child) {
            if input_text.is_none() || input_text.map(|text| text.is_empty()).unwrap_or(false) {
                *input_visibility = Visibility::Hidden;
                commands
                    .entity(*child)
                    .remove::<Cursor>()
                    .remove::<Text>()
                    .remove::<Node>();
                should_add_place_holder = true;
            }
        }
    }

    if let Ok(placeholder_text) = placeholder.get(entity) {
        if should_add_place_holder {
            commands.entity(entity).with_child((
                placeholder_text.clone(),
                Placeholder::text_style(size),
                Text::new(placeholder_text.0.to_string()),
            ));
        }
    }
}

pub(crate) fn text_field_on_focus(
    click: Trigger<GotFocus>,
    mut commands: Commands,
    text_fields: Query<&Children, (With<SingleLineTextField>, With<Clickable>)>,
    mut placeholder: Query<
        (&mut Visibility, Option<&Text>),
        (With<Placeholder>, Without<TextField>),
    >,
    mut input_text: Query<&mut Visibility, With<TextField>>,
) {
    let entity = click.entity();
    let Ok(children_entities) = text_fields.get(entity) else {
        return;
    };

    for child in children_entities.iter() {
        if let Ok((mut placeholder_visibility, placeholder_text)) = placeholder.get_mut(*child) {
            *placeholder_visibility = Visibility::Hidden;
            if placeholder_text.is_some() {
                commands.entity(*child).despawn_recursive();
            }
        }

        if let Ok(mut input_visibility) = input_text.get_mut(*child) {
            *input_visibility = Visibility::Visible;
            commands
                .entity(*child)
                .insert_if_new((Cursor::default(), Text::default()))
                .insert_if_new(Node::default());
        }
    }
}
