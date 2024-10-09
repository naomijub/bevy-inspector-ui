use bevy_color::Color;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::{Component, ReflectComponent};
use bevy_ecs::{
    query::{Changed, With},
    system::{Commands, Query},
};
use bevy_hierarchy::{BuildChildren, ChildBuild};
use bevy_reflect::Reflect;
use bevy_text::TextStyle;
use bevy_ui::{
    prelude::{ButtonBundle, NodeBundle, TextBundle},
    widget::Button,
    AlignItems, BackgroundColor, BorderColor, BorderRadius, Interaction, JustifyContent, Style,
    UiRect, Val,
};
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
/// - default: radius of 4px
/// - rounded: radius of 100% (i.e. circle)
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonRadius {
    #[default]
    Default,
    Rounded,
}

impl ButtonRadius {
    /// Radius for each button radius in bevy_ui::BorderRadius
    pub const fn radius(&self) -> BorderRadius {
        match self {
            Self::Default => BorderRadius::all(Val::Px(4.)),
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
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ButtonBuilder {
    button_type: ButtonType,
    button_size: ButtonSize,
    button_radius: ButtonRadius,
    text: Option<String>,
}

impl ButtonBuilder {
    /// Create new button with text. use default for empty button.
    pub fn new(text: &str) -> Self {
        Self {
            text: Some(text.to_string()),
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

    /// Spawns button from the builder returning its entity id
    pub fn build(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                parent
                    .spawn((
                        ButtonBundle {
                            style: Style {
                                width: Val::Auto,
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
                            parent.spawn(TextBundle::from_section(
                                text,
                                TextStyle {
                                    // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: self.button_size.font_size(),
                                    color: self.button_type.font_color(),
                                    ..default()
                                },
                            ));
                        }
                    });
            }).id()
    }
}
