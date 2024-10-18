use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::{Component, ReflectComponent};
use bevy::ecs::system::Commands;
use bevy::hierarchy::ChildBuilder;
use bevy::reflect::Reflect;

mod builder;
mod constants;
mod helpers;
pub(super) mod systems;

/// Marks button as disabled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct DisableButton;

pub mod prelude {
    pub use super::helpers::*;
    pub use builder::{ButtonBuilder, ButtonRadius, ButtonSize, ButtonType};
}
