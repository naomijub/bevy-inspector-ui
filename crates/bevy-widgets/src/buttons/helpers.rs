pub use super::*;
use crate::buttons::builder::*;

/// Defines button as `Primary`. Should be user as generic argument on trait `SpawnButton`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Primary;

/// Defines button as `Secondary`. Should be user as generic argument on trait `SpawnButton`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Secondary;

/// Defines button as `Tertiary`. Should be user as generic argument on trait `SpawnButton`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tertiary;

/// Defines button as `Small`. Should be user as generic argument on trait `SpawnButton`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Small;

/// Defines button as `Medium`. Should be user as generic argument on trait `SpawnButton`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Medium;

/// Defines button as `Large`. Should be user as generic argument on trait `SpawnButton`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Large;

/// Defines button as `Squared`. Should be user as generic argument on trait `SpawnButton`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Squared;

/// Defines button as `Rounded`. Should be user as generic argument on trait `SpawnButton`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rounded;

/// Defines button as `Default`. Should be user as generic argument on trait `SpawnButton`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Default;

/// Auxiliary trait to spawn buttons. It is used with its generic implementation.
/// There is a generic call order `(ButtonType, ButtonSize, ButtonRadius)`, so example calls are:
/// - `spawn_button("text",(Primary, Medium, Squared))` or `spawn_button("text", Default)`: Default button.
/// - `spawn_button("text",(Secondary, Rounded))`: Secondary rounded button.
/// - `spawn_button("text",Large)`: Large primary squared button.
///
pub trait SpawnButton<T> {
    fn spawn_button(&mut self, text: impl Into<String>, _: T) -> Entity;
}

impl SpawnButton<Default> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Default) -> Entity {
        ButtonBuilder::new(text.into()).child_build(self)
    }
}

impl SpawnButton<Primary> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Primary) -> Entity {
        ButtonBuilder::new(text.into()).child_build(self)
    }
}

impl SpawnButton<Secondary> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Secondary) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .child_build(self)
    }
}

impl SpawnButton<Tertiary> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Tertiary) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .child_build(self)
    }
}

impl SpawnButton<Small> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Small) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<Medium> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Medium) -> Entity {
        ButtonBuilder::new(text.into()).child_build(self)
    }
}

impl SpawnButton<Large> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Large) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Large)
            .child_build(self)
    }
}

impl SpawnButton<Squared> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Squared) -> Entity {
        ButtonBuilder::new(text.into()).child_build(self)
    }
}

impl SpawnButton<Rounded> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Rounded) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .child_build(self)
    }
}

impl SpawnButton<(Primary, Small)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Small)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Primary, Medium)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Medium)) -> Entity {
        ButtonBuilder::new(text.into()).child_build(self)
    }
}

impl SpawnButton<(Primary, Large)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Large)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Secondary, Small)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Small)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Secondary, Medium)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Medium)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .child_build(self)
    }
}

impl SpawnButton<(Secondary, Large)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Large)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Tertiary, Small)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Small)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Tertiary, Medium)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Medium)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .child_build(self)
    }
}

impl SpawnButton<(Tertiary, Large)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Large)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Primary, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Squared)) -> Entity {
        ButtonBuilder::new(text.into()).child_build(self)
    }
}

impl SpawnButton<(Primary, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .child_build(self)
    }
}

impl SpawnButton<(Secondary, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .child_build(self)
    }
}

impl SpawnButton<(Secondary, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_radius(ButtonRadius::Rounded)
            .child_build(self)
    }
}

impl SpawnButton<(Tertiary, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .child_build(self)
    }
}

impl SpawnButton<(Tertiary, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_radius(ButtonRadius::Rounded)
            .child_build(self)
    }
}

impl SpawnButton<(Small, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Small, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Small, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Small, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .with_radius(ButtonRadius::Rounded)
            .child_build(self)
    }
}

impl SpawnButton<(Medium, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Medium, Squared)) -> Entity {
        ButtonBuilder::new(text.into()).child_build(self)
    }
}

impl SpawnButton<(Medium, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Medium, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .child_build(self)
    }
}

impl SpawnButton<(Large, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Large, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Large)
            .child_build(self)
    }
}

impl SpawnButton<(Large, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Large, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Large)
            .with_radius(ButtonRadius::Rounded)
            .child_build(self)
    }
}

impl SpawnButton<(Primary, Small, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Small, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Primary, Small, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Small, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Primary, Medium, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Medium, Squared)) -> Entity {
        ButtonBuilder::new(text.into()).child_build(self)
    }
}

impl SpawnButton<(Primary, Medium, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Medium, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .child_build(self)
    }
}

impl SpawnButton<(Primary, Large, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Large, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Large)
            .child_build(self)
    }
}

impl SpawnButton<(Primary, Large, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Large, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Large)
            .child_build(self)
    }
}

impl SpawnButton<(Secondary, Small, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Small, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Secondary, Small, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Small, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Secondary, Medium, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Medium, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .child_build(self)
    }
}

impl SpawnButton<(Secondary, Medium, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Medium, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_radius(ButtonRadius::Rounded)
            .child_build(self)
    }
}

impl SpawnButton<(Secondary, Large, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Large, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_size(ButtonSize::Large)
            .child_build(self)
    }
}

impl SpawnButton<(Secondary, Large, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Large, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Large)
            .child_build(self)
    }
}

impl SpawnButton<(Tertiary, Small, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Small, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Tertiary, Small, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Small, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Small)
            .child_build(self)
    }
}

impl SpawnButton<(Tertiary, Medium, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Medium, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .child_build(self)
    }
}

impl SpawnButton<(Tertiary, Medium, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Medium, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_radius(ButtonRadius::Rounded)
            .child_build(self)
    }
}

impl SpawnButton<(Tertiary, Large, Squared)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Large, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_size(ButtonSize::Large)
            .child_build(self)
    }
}

impl SpawnButton<(Tertiary, Large, Rounded)> for ChildBuilder<'_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Large, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Large)
            .child_build(self)
    }
}

impl SpawnButton<Default> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Default) -> Entity {
        ButtonBuilder::new(text.into()).build(self)
    }
}

impl SpawnButton<Primary> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Primary) -> Entity {
        ButtonBuilder::new(text.into()).build(self)
    }
}

impl SpawnButton<Secondary> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Secondary) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .build(self)
    }
}

impl SpawnButton<Tertiary> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Tertiary) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .build(self)
    }
}

impl SpawnButton<Small> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Small) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<Medium> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Medium) -> Entity {
        ButtonBuilder::new(text.into()).build(self)
    }
}

impl SpawnButton<Large> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Large) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Large)
            .build(self)
    }
}

impl SpawnButton<Squared> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Squared) -> Entity {
        ButtonBuilder::new(text.into()).build(self)
    }
}

impl SpawnButton<Rounded> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: Rounded) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .build(self)
    }
}

impl SpawnButton<(Primary, Small)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Small)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Primary, Medium)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Medium)) -> Entity {
        ButtonBuilder::new(text.into()).build(self)
    }
}

impl SpawnButton<(Primary, Large)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Large)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Secondary, Small)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Small)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Secondary, Medium)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Medium)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .build(self)
    }
}

impl SpawnButton<(Secondary, Large)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Large)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Tertiary, Small)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Small)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Tertiary, Medium)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Medium)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .build(self)
    }
}

impl SpawnButton<(Tertiary, Large)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Large)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Primary, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Squared)) -> Entity {
        ButtonBuilder::new(text.into()).build(self)
    }
}

impl SpawnButton<(Primary, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .build(self)
    }
}

impl SpawnButton<(Secondary, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .build(self)
    }
}

impl SpawnButton<(Secondary, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_radius(ButtonRadius::Rounded)
            .build(self)
    }
}

impl SpawnButton<(Tertiary, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .build(self)
    }
}

impl SpawnButton<(Tertiary, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_radius(ButtonRadius::Rounded)
            .build(self)
    }
}

impl SpawnButton<(Small, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Small, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Small, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Small, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .with_radius(ButtonRadius::Rounded)
            .build(self)
    }
}

impl SpawnButton<(Medium, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Medium, Squared)) -> Entity {
        ButtonBuilder::new(text.into()).build(self)
    }
}

impl SpawnButton<(Medium, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Medium, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .build(self)
    }
}

impl SpawnButton<(Large, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Large, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Large)
            .build(self)
    }
}

impl SpawnButton<(Large, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Large, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Large)
            .with_radius(ButtonRadius::Rounded)
            .build(self)
    }
}

impl SpawnButton<(Primary, Small, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Small, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Primary, Small, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Small, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Primary, Medium, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Medium, Squared)) -> Entity {
        ButtonBuilder::new(text.into()).build(self)
    }
}

impl SpawnButton<(Primary, Medium, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Medium, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .build(self)
    }
}

impl SpawnButton<(Primary, Large, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Large, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_size(ButtonSize::Large)
            .build(self)
    }
}

impl SpawnButton<(Primary, Large, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Primary, Large, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Large)
            .build(self)
    }
}

impl SpawnButton<(Secondary, Small, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Small, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Secondary, Small, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Small, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Secondary, Medium, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Medium, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .build(self)
    }
}

impl SpawnButton<(Secondary, Medium, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Medium, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_radius(ButtonRadius::Rounded)
            .build(self)
    }
}

impl SpawnButton<(Secondary, Large, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Large, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_size(ButtonSize::Large)
            .build(self)
    }
}

impl SpawnButton<(Secondary, Large, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Secondary, Large, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Secondary)
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Large)
            .build(self)
    }
}

impl SpawnButton<(Tertiary, Small, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Small, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Tertiary, Small, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Small, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Small)
            .build(self)
    }
}

impl SpawnButton<(Tertiary, Medium, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Medium, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .build(self)
    }
}

impl SpawnButton<(Tertiary, Medium, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Medium, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_radius(ButtonRadius::Rounded)
            .build(self)
    }
}

impl SpawnButton<(Tertiary, Large, Squared)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Large, Squared)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_size(ButtonSize::Large)
            .build(self)
    }
}

impl SpawnButton<(Tertiary, Large, Rounded)> for Commands<'_, '_> {
    fn spawn_button(&mut self, text: impl Into<String>, _: (Tertiary, Large, Rounded)) -> Entity {
        ButtonBuilder::new(text.into())
            .with_type(ButtonType::Tertiary)
            .with_radius(ButtonRadius::Rounded)
            .with_size(ButtonSize::Large)
            .build(self)
    }
}
