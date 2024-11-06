use bevy::prelude::*;

use crate::focus::Focus;

use super::{
    builder::{ButtonType, ButtonsText, SubInteraction},
    ButtonClickedEvent, DisableButton,
};

pub fn on_add_focus(
    trigger: Trigger<OnAdd, Focus>,
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Node,
            &ButtonType,
            Option<&DisableButton>,
        ),
        With<Button>,
    >,
) {
    let entity = trigger.entity();
    if let Ok((mut bg, mut border, mut node, button_type, None)) = interaction_query.get_mut(entity)
    {
        *bg = button_type.background_color(SubInteraction::Focus).into();
        border.0 = button_type.border_color(SubInteraction::Focus);
        node.border = button_type.border_width(SubInteraction::Focus);
    } else {
        commands.entity(entity).remove::<Focus>();
    }
}

pub(crate) fn button_system(
    mut interaction_query: Query<
        (
            Entity,
            &ButtonsText,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Node,
            &ButtonType,
            Option<&DisableButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut event_writer: EventWriter<ButtonClickedEvent>,
) {
    for (
        entity,
        button_text,
        interaction,
        mut color,
        mut border_color,
        mut node,
        button_type,
        is_disabled,
    ) in &mut interaction_query
    {
        if is_disabled.is_some() {
            *color = button_type
                .background_color(SubInteraction::Disabled)
                .into();
            border_color.0 = button_type.border_color(SubInteraction::Disabled);
            node.border = button_type.border_width(SubInteraction::Disabled);
        } else {
            match *interaction {
                Interaction::Pressed => {
                    *color = button_type.background_color(SubInteraction::Pressed).into();
                    border_color.0 = button_type.border_color(SubInteraction::Pressed);
                    node.border = button_type.border_width(SubInteraction::Pressed);
                    event_writer.send(ButtonClickedEvent {
                        entity,
                        value: button_text.0.clone(),
                    });
                }
                Interaction::Hovered => {
                    *color = button_type.background_color(SubInteraction::Hovered).into();
                    border_color.0 = button_type.border_color(SubInteraction::Hovered);
                    node.border = button_type.border_width(SubInteraction::Hovered);
                }
                Interaction::None => {
                    *color = button_type.background_color(SubInteraction::Default).into();
                    border_color.0 = button_type.border_color(SubInteraction::Default);
                    node.border = button_type.border_width(SubInteraction::Default);
                }
            }
        }
    }
}
