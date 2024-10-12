use std::convert::Infallible;
use std::str::FromStr;

// TODO: Set focus behavior
use bevy_color::Color;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_ecs::{
    query::{Changed, With},
    system::{Commands, Query},
};
use bevy_hierarchy::{BuildChildren, ChildBuild, ChildBuilder};
use bevy_reflect::Reflect;
use bevy_text::TextStyle;
use bevy_ui::widget::Text;
use bevy_ui::{
    prelude::{ButtonBundle, NodeBundle},
    widget::Button,
    AlignItems, BackgroundColor, BorderColor, BorderRadius, Interaction, Style, UiRect, Val,
};
use bevy_ui::{JustifyContent, PositionType};
use bevy_utils::default;

const SMALL_MEDIUM_FONT_SIZE: f32 = 10.0;
const LARGE_FONT_SIZE: f32 = 13.0;

const PRIMARY_TEXT_COLOR: Color = Color::srgb(0.97, 0.97, 0.98);
const SECONDARY_TEXT_COLOR: Color = Color::srgb(0.29, 0.31, 0.33);
const TERTIARY_TEXT_COLOR: Color = Color::srgb(0.97, 0.97, 0.98);

const NORMAL_BUTTON: Color = Color::srgb(0.19, 0.49, 0.71);
const NORMAL_SEC_BUTTON: Color = Color::srgb(0.93, 0.97, 1.);
const NORMAL_TER_BUTTON: Color = Color::srgb(0.11, 0.29, 0.42);
const HOVERED_BUTTON: Color = Color::srgb(0.51, 0.79, 1.);
const HOVERED_SEC_BUTTON: Color = Color::srgb(0.93, 0.97, 1.);
const HOVERED_TER_BUTTON: Color = Color::srgb(0.51, 0.79, 1.);
const PRESSED_BUTTON: Color = Color::srgb(0.11, 0.29, 0.42);
const PRESSED_SEC_BUTTON: Color = Color::srgb(0.51, 0.79, 1.);
const PRESSED_TER_BUTTON: Color = Color::srgb(0.19, 0.49, 0.71);
const DISABLED_BUTTON: Color = Color::srgb(0.8, 0.83, 0.85);

const HOVERED_BORDER: Color = Color::srgb(0.93, 0.97, 1.);
const SECONDARY_BORDER: Color = Color::srgb(0.51, 0.79, 1.);

/// Buttons can be classified accordingly to their height:
/// - small: height of 20px, padding of 16px x 8px, font size of 10px
/// - medium: height of 24px, padding of 20px x 12px, font size of 10px
/// - large: height of 30px, padding of 24px x 16px, font size of 13px
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    /// Font size for each button size
    pub const fn font_size(&self) -> f32 {
        match self {
            Self::Small => SMALL_MEDIUM_FONT_SIZE,
            Self::Medium => SMALL_MEDIUM_FONT_SIZE,
            Self::Large => LARGE_FONT_SIZE,
        }
    }

    /// Height for each button size
    pub const fn height(&self) -> Val {
        match self {
            Self::Small => Val::Px(20.),
            Self::Medium => Val::Px(24.),
            Self::Large => Val::Px(30.),
        }
    }

    /// Padding for each button size (both horizontal and vertical)
    pub fn padding(&self) -> UiRect {
        match self {
            Self::Small => UiRect::axes(Val::Px(16.), Val::Px(8.)),
            Self::Medium => UiRect::axes(Val::Px(20.), Val::Px(12.)),
            Self::Large => UiRect::axes(Val::Px(24.), Val::Px(16.)),
        }
    }
}

/// Buttons can be classified accordingly to their radius:
/// - squared: radius of 4px
/// - rounded: radius of 100% (i.e. circle)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonRadius {
    #[default]
    Squared,
    Rounded,
}

impl ButtonRadius {
    /// Radius for each button radius in bevy_ui::BorderRadius
    pub const fn radius(&self) -> BorderRadius {
        match self {
            Self::Squared => BorderRadius::all(Val::Px(4.)),
            Self::Rounded => BorderRadius::MAX,
        }
    }
}

/// Buttons can be classified accordingly to their type:
/// - primary: text color is #F7F8F9 (white), background color is #307CB5 (blue)
/// - secondary: text color is #4B4F53 (dark gray), background color is #ECF7FF (white)
/// - tertiary: text color is #F7F8F9 (white), background color is #1D496B (dark blue)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum ButtonType {
    #[default]
    Primary,
    Secondary,
    Tertiary,
}

enum SubInteraction {
    Default,
    Hovered,
    Pressed,
    Disabled,
}

impl ButtonType {
    /// Default ont color for each button type
    pub const fn font_color(&self) -> Color {
        match self {
            Self::Primary => PRIMARY_TEXT_COLOR,
            Self::Secondary => SECONDARY_TEXT_COLOR,
            Self::Tertiary => TERTIARY_TEXT_COLOR,
        }
    }

    const fn border_width(&self, interaction: SubInteraction) -> UiRect {
        match (self, interaction) {
            (Self::Secondary, SubInteraction::Hovered) => UiRect::all(Val::Px(4.)),
            _ => UiRect::all(Val::Px(1.)),
        }
    }

    const fn border_color(&self, interaction: SubInteraction) -> Color {
        match (self, interaction) {
            (Self::Primary, SubInteraction::Default) => NORMAL_BUTTON,
            (Self::Primary, SubInteraction::Hovered) => HOVERED_BORDER,
            (Self::Primary, SubInteraction::Pressed) => PRESSED_BUTTON,
            (Self::Secondary, _) => SECONDARY_BORDER,
            (Self::Tertiary, SubInteraction::Default) => NORMAL_TER_BUTTON,
            (Self::Tertiary, SubInteraction::Hovered) => HOVERED_TER_BUTTON,
            (Self::Tertiary, SubInteraction::Pressed) => PRESSED_TER_BUTTON,
            _ => DISABLED_BUTTON,
        }
    }

    const fn background_color(&self, interaction: SubInteraction) -> Color {
        match (self, interaction) {
            (Self::Primary, SubInteraction::Default) => NORMAL_BUTTON,
            (Self::Secondary, SubInteraction::Default) => NORMAL_SEC_BUTTON,
            (Self::Tertiary, SubInteraction::Default) => NORMAL_TER_BUTTON,
            (Self::Primary, SubInteraction::Hovered) => HOVERED_BUTTON,
            (Self::Primary, SubInteraction::Pressed) => PRESSED_BUTTON,
            (Self::Primary, SubInteraction::Disabled) => DISABLED_BUTTON,
            (Self::Secondary, SubInteraction::Hovered) => HOVERED_SEC_BUTTON,
            (Self::Secondary, SubInteraction::Pressed) => PRESSED_SEC_BUTTON,
            (Self::Tertiary, SubInteraction::Hovered) => HOVERED_TER_BUTTON,
            (Self::Tertiary, SubInteraction::Pressed) => PRESSED_TER_BUTTON,
            _ => DISABLED_BUTTON,
        }
    }
}

/// Marks button as disabled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct DisableButton;

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

/// Builder for [`bevy_ui::Button`]
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ButtonBuilder {
    button_type: ButtonType,
    button_size: ButtonSize,
    button_radius: ButtonRadius,
    text: Option<String>,
    width: Option<Val>,
}

impl FromStr for ButtonBuilder {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            text: Some(s.to_string()),
            ..default()
        })
    }
}

impl ButtonBuilder {
    /// Create new button with text. use default for empty button.
    pub fn new(text: String) -> Self {
        Self {
            text: Some(text),
            ..default()
        }
    }

    /// Adds [`ButtonType`] to the builder.
    /// Default value is [`ButtonType::Primary`].
    /// - [`ButtonType::Primary`]
    /// - [`ButtonType::Secondary`]
    /// - [`ButtonType::Tertiary`]
    pub const fn with_type(mut self, button_type: ButtonType) -> Self {
        self.button_type = button_type;
        self
    }

    /// Adds [`ButtonSize`] to the builder.
    /// Default value is [`ButtonSize::Medium`].
    /// - [`ButtonSize::Small`] - 20px
    /// - [`ButtonSize::Medium`] - 24px
    /// - [`ButtonSize::Large`] - 30px
    pub const fn with_size(mut self, button_size: ButtonSize) -> Self {
        self.button_size = button_size;
        self
    }

    /// Adds [`ButtonRadius`] to the builder.
    /// Default value is [`ButtonRadius::Default`].
    /// - [`ButtonRadius::Default`] - 4px
    /// - [`ButtonRadius::Rounded`] - 100%
    ///
    pub const fn with_radius(mut self, button_radius: ButtonRadius) -> Self {
        self.button_radius = button_radius;
        self
    }

    /// Defines a fixed width for the button.
    ///
    /// # Alert
    /// Due to the way that buttons are drawn, there is a compromise on how long it takes to render the button.
    pub const fn with_fixed_width(mut self, width: Val) -> Self {
        self.width = Some(width);
        self
    }

    /// Spawns button from the builder returning its entity id
    pub fn build(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Relative,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                self.with_button(parent);
            })
            .id()
    }

    fn with_button(self, parent: &mut ChildBuilder<'_>) {
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: self.width.unwrap_or(Val::Auto),
                        height: self.button_size.height(),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: self.button_size.padding(),
                        ..default()
                    },
                    border_color: self
                        .button_type
                        .border_color(SubInteraction::Default)
                        .into(),
                    border_radius: self.button_radius.radius(),
                    background_color: self
                        .button_type
                        .background_color(SubInteraction::Default)
                        .into(),
                    ..default()
                },
                self.button_type,
            ))
            .with_children(|parent| {
                if let Some(text) = self.text {
                    parent.spawn((
                        Text::new(text),
                        TextStyle {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: self.button_size.font_size(),
                            color: self.button_type.font_color(),
                            ..default()
                        },
                    ));
                }
            });
    }

    pub fn child_build(self, commands: &mut ChildBuilder) -> Entity {
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Relative,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                self.with_button(parent);
            })
            .id()
    }
}

pub mod prelude {
    pub use super::*;

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
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Small, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_size(ButtonSize::Small)
                .child_build(self)
        }
    }

    impl SpawnButton<(Primary, Small, Rounded)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Small, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_radius(ButtonRadius::Rounded)
                .with_size(ButtonSize::Small)
                .child_build(self)
        }
    }

    impl SpawnButton<(Primary, Medium, Squared)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Medium, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into()).child_build(self)
        }
    }

    impl SpawnButton<(Primary, Medium, Rounded)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Medium, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_radius(ButtonRadius::Rounded)
                .child_build(self)
        }
    }

    impl SpawnButton<(Primary, Large, Squared)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Large, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_size(ButtonSize::Large)
                .child_build(self)
        }
    }

    impl SpawnButton<(Primary, Large, Rounded)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Large, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_radius(ButtonRadius::Rounded)
                .with_size(ButtonSize::Large)
                .child_build(self)
        }
    }

    impl SpawnButton<(Secondary, Small, Squared)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Small, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .with_size(ButtonSize::Small)
                .child_build(self)
        }
    }

    impl SpawnButton<(Secondary, Small, Rounded)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Small, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .with_radius(ButtonRadius::Rounded)
                .with_size(ButtonSize::Small)
                .child_build(self)
        }
    }

    impl SpawnButton<(Secondary, Medium, Squared)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Medium, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .child_build(self)
        }
    }

    impl SpawnButton<(Secondary, Medium, Rounded)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Medium, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .with_radius(ButtonRadius::Rounded)
                .child_build(self)
        }
    }

    impl SpawnButton<(Secondary, Large, Squared)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Large, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .with_size(ButtonSize::Large)
                .child_build(self)
        }
    }

    impl SpawnButton<(Secondary, Large, Rounded)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Large, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .with_radius(ButtonRadius::Rounded)
                .with_size(ButtonSize::Large)
                .child_build(self)
        }
    }

    impl SpawnButton<(Tertiary, Small, Squared)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Small, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Tertiary)
                .with_size(ButtonSize::Small)
                .child_build(self)
        }
    }

    impl SpawnButton<(Tertiary, Small, Rounded)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Small, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Tertiary)
                .with_radius(ButtonRadius::Rounded)
                .with_size(ButtonSize::Small)
                .child_build(self)
        }
    }

    impl SpawnButton<(Tertiary, Medium, Squared)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Medium, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Tertiary)
                .child_build(self)
        }
    }

    impl SpawnButton<(Tertiary, Medium, Rounded)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Medium, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Tertiary)
                .with_radius(ButtonRadius::Rounded)
                .child_build(self)
        }
    }

    impl SpawnButton<(Tertiary, Large, Squared)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Large, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Tertiary)
                .with_size(ButtonSize::Large)
                .child_build(self)
        }
    }

    impl SpawnButton<(Tertiary, Large, Rounded)> for ChildBuilder<'_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Large, Rounded),
        ) -> Entity {
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
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Small, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_size(ButtonSize::Small)
                .build(self)
        }
    }

    impl SpawnButton<(Primary, Small, Rounded)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Small, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_radius(ButtonRadius::Rounded)
                .with_size(ButtonSize::Small)
                .build(self)
        }
    }

    impl SpawnButton<(Primary, Medium, Squared)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Medium, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into()).build(self)
        }
    }

    impl SpawnButton<(Primary, Medium, Rounded)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Medium, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_radius(ButtonRadius::Rounded)
                .build(self)
        }
    }

    impl SpawnButton<(Primary, Large, Squared)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Large, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_size(ButtonSize::Large)
                .build(self)
        }
    }

    impl SpawnButton<(Primary, Large, Rounded)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Primary, Large, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_radius(ButtonRadius::Rounded)
                .with_size(ButtonSize::Large)
                .build(self)
        }
    }

    impl SpawnButton<(Secondary, Small, Squared)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Small, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .with_size(ButtonSize::Small)
                .build(self)
        }
    }

    impl SpawnButton<(Secondary, Small, Rounded)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Small, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .with_radius(ButtonRadius::Rounded)
                .with_size(ButtonSize::Small)
                .build(self)
        }
    }

    impl SpawnButton<(Secondary, Medium, Squared)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Medium, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .build(self)
        }
    }

    impl SpawnButton<(Secondary, Medium, Rounded)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Medium, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .with_radius(ButtonRadius::Rounded)
                .build(self)
        }
    }

    impl SpawnButton<(Secondary, Large, Squared)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Large, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .with_size(ButtonSize::Large)
                .build(self)
        }
    }

    impl SpawnButton<(Secondary, Large, Rounded)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Secondary, Large, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Secondary)
                .with_radius(ButtonRadius::Rounded)
                .with_size(ButtonSize::Large)
                .build(self)
        }
    }

    impl SpawnButton<(Tertiary, Small, Squared)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Small, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Tertiary)
                .with_size(ButtonSize::Small)
                .build(self)
        }
    }

    impl SpawnButton<(Tertiary, Small, Rounded)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Small, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Tertiary)
                .with_radius(ButtonRadius::Rounded)
                .with_size(ButtonSize::Small)
                .build(self)
        }
    }

    impl SpawnButton<(Tertiary, Medium, Squared)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Medium, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Tertiary)
                .build(self)
        }
    }

    impl SpawnButton<(Tertiary, Medium, Rounded)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Medium, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Tertiary)
                .with_radius(ButtonRadius::Rounded)
                .build(self)
        }
    }

    impl SpawnButton<(Tertiary, Large, Squared)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Large, Squared),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Tertiary)
                .with_size(ButtonSize::Large)
                .build(self)
        }
    }

    impl SpawnButton<(Tertiary, Large, Rounded)> for Commands<'_, '_> {
        fn spawn_button(
            &mut self,
            text: impl Into<String>,
            _: (Tertiary, Large, Rounded),
        ) -> Entity {
            ButtonBuilder::new(text.into())
                .with_type(ButtonType::Tertiary)
                .with_radius(ButtonRadius::Rounded)
                .with_size(ButtonSize::Large)
                .build(self)
        }
    }
}
