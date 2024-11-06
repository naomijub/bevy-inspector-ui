use std::{convert::Infallible, str::FromStr};

use crate::{buttons::constants::*, focus::Clickable};
use bevy::prelude::*;

/// A helper container for button text
#[derive(Debug, Clone, Component, Reflect, PartialEq, Eq, Hash, Default)]
#[reflect(Component)]
pub struct ButtonsText(pub String);

/// Buttons can be classified accordingly to their height:
/// - small: height of 20px, padding of 16px x 8px, font size of 10px
/// - medium: height of 24px, padding of 20px x 12px, font size of 10px
/// - large: height of 30px, padding of 24px x 16px, font size of 13px
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ButtonSize {
    /// small: height of 20px, padding of 16px x 8px, font size of 10px
    Small,
    #[default]
    /// medium: height of 24px, padding of 20px x 12px, font size of 10px. the default button size
    Medium,
    /// large: height of 30px, padding of 24px x 16px, font size of 13px
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
    /// Border radius of 4px, soft square.
    #[default]
    Squared,
    /// Border radius of 100%, completely round border
    Rounded,
}

impl ButtonRadius {
    /// Radius for each button radius in bevy::ui::BorderRadius
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
    /// primary: text color is #F7F8F9 (white), background color is #307CB5 (blue).
    Primary,
    /// secondary: text color is #4B4F53 (dark gray), background color is #ECF7FF (white)
    Secondary,
    /// tertiary: text color is #F7F8F9 (white), background color is #1D496B (dark blue)
    Tertiary,
}

pub(crate) enum SubInteraction {
    Default,
    Hovered,
    Pressed,
    Disabled,
    Focus,
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

    pub(super) const fn border_width(&self, interaction: SubInteraction) -> UiRect {
        match (self, interaction) {
            (Self::Secondary, SubInteraction::Hovered) => UiRect::all(Val::Px(4.)),
            _ => UiRect::all(Val::Px(1.)),
        }
    }

    pub(super) const fn border_color(&self, interaction: SubInteraction) -> Color {
        match (self, interaction) {
            (Self::Primary, SubInteraction::Default) => NORMAL_BUTTON,
            (Self::Primary, SubInteraction::Hovered) => HOVERED_BORDER,
            (Self::Primary, SubInteraction::Pressed) => PRESSED_BUTTON,
            (Self::Primary, SubInteraction::Focus) => FOCUS_BORDER_BUTTON,
            (Self::Secondary, SubInteraction::Focus) => FOCUS_BORDER_SEC_BUTTON,
            (Self::Secondary, _) => SECONDARY_BORDER,
            (Self::Tertiary, SubInteraction::Default) => NORMAL_TER_BUTTON,
            (Self::Tertiary, SubInteraction::Hovered) => HOVERED_TER_BUTTON,
            (Self::Tertiary, SubInteraction::Pressed) => PRESSED_TER_BUTTON,
            (Self::Tertiary, SubInteraction::Focus) => FOCUS_BORDER_TER_BUTTON,
            _ => DISABLED_BUTTON,
        }
    }

    pub(super) const fn background_color(&self, interaction: SubInteraction) -> Color {
        match (self, interaction) {
            (Self::Primary, SubInteraction::Default) => NORMAL_BUTTON,
            (Self::Secondary, SubInteraction::Default) => NORMAL_SEC_BUTTON,
            (Self::Tertiary, SubInteraction::Default) => NORMAL_TER_BUTTON,
            (Self::Primary, SubInteraction::Focus) => FOCUS_BG_BUTTON,
            (Self::Secondary, SubInteraction::Focus) => FOCUS_BG_SEC_BUTTON,
            (Self::Tertiary, SubInteraction::Focus) => FOCUS_BG_TER_BUTTON,
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

/// Builder for [`bevy::ui::Button`]
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
            .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Relative,
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
                Clickable,
                ButtonsText(self.text.clone().unwrap_or_default()),
                Button,
                Node {
                    width: self.width.unwrap_or(Val::Auto),
                    height: self.button_size.height(),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: self.button_size.padding(),
                    ..default()
                },
                Into::<BorderColor>::into(self.button_type.border_color(SubInteraction::Default)),
                self.button_radius.radius(),
                Into::<BackgroundColor>::into(
                    self.button_type.background_color(SubInteraction::Default),
                ),
                self.button_type,
            ))
            .with_children(|parent| {
                if let Some(text) = self.text {
                    parent.spawn((
                        Text::new(text),
                        TextColor(self.button_type.font_color()),
                        TextFont {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: self.button_size.font_size(),
                            ..default()
                        },
                    ));
                }
            });
    }

    pub(crate) fn child_build(self, commands: &mut ChildBuilder) -> Entity {
        commands
            .spawn(Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Relative,
                ..default()
            })
            .with_children(|parent| {
                self.with_button(parent);
            })
            .id()
    }
}
