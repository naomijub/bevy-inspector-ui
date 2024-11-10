use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::{Component, ReflectComponent};
use bevy::ecs::system::Commands;
use bevy::hierarchy::ChildBuilder;
use bevy::prelude::Event;
use bevy::reflect::Reflect;

mod builder;
mod constants;
mod helpers;
pub(super) mod systems;

/// Marks button as disabled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct DisableButton;

/// Prelude containing all the commonly used components and builders for buttons
pub mod prelude {
    pub use super::helpers::*;
    pub use super::ButtonClickedEvent;
    pub use builder::{ButtonBuilder, ButtonRadius, ButtonSize, ButtonType};
}

/// An event that is fired when the user presses the button.
#[derive(Event, Debug, Reflect)]
pub struct ButtonClickedEvent {
    /// The entity that triggered the event.
    pub entity: Entity,
    /// The string contained in the button.
    pub value: String,
}
