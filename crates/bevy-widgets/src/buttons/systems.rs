use bevy::prelude::*;

use super::{
    builder::{ButtonType, SubInteraction},
    DisableButton,
};

pub(crate) fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Style,
            &ButtonType,
            Option<&DisableButton>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut border_color, mut style, button_type, is_disabled) in
        &mut interaction_query
    {
        if is_disabled.is_some() {
            *color = button_type
                .background_color(SubInteraction::Disabled)
                .into();
            border_color.0 = button_type.border_color(SubInteraction::Disabled);
            style.border = button_type.border_width(SubInteraction::Disabled);
        } else {
            match *interaction {
                Interaction::Pressed => {
                    *color = button_type.background_color(SubInteraction::Pressed).into();
                    border_color.0 = button_type.border_color(SubInteraction::Pressed);
                    style.border = button_type.border_width(SubInteraction::Pressed);
                }
                Interaction::Hovered => {
                    *color = button_type.background_color(SubInteraction::Hovered).into();
                    border_color.0 = button_type.border_color(SubInteraction::Hovered);
                    style.border = button_type.border_width(SubInteraction::Hovered);
                }
                Interaction::None => {
                    *color = button_type.background_color(SubInteraction::Default).into();
                    border_color.0 = button_type.border_color(SubInteraction::Default);
                    style.border = button_type.border_width(SubInteraction::Default);
                }
            }
        }
    }
}
