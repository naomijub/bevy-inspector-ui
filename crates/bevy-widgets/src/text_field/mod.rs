use bevy_app::{App, Plugin, Update};
use bevy_color::Color;
use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::ReflectComponent;
use bevy_ecs::{component::Component, system::Commands};
use bevy_hierarchy::{BuildChildren, ChildBuild, ChildBuilder};
use bevy_reflect::Reflect;
use bevy_render::view::Visibility;
use bevy_text::{TextLayout, TextStyle};
use bevy_transform::components::Transform;
use bevy_ui::prelude::NodeBundle;
use bevy_ui::widget::{Label, Text, TextNodeFlags};
use bevy_ui::{
    AlignItems, BorderRadius, ContentSize, FlexDirection, FocusPolicy, JustifyContent, Node,
    PositionType, Style, UiRect, Val, ZIndex,
};
use bevy_utils::default;
use systems::{focus_system, react_on_removal, text_field_lost_focus, text_field_on_focus};

use crate::focus::Clickable;

pub struct TextFieldPlugin;

impl Plugin for TextFieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, focus_system)
            .add_observer(text_field_on_focus)
            .add_observer(text_field_lost_focus)
            .add_observer(react_on_removal);
    }
}

pub(crate) mod systems;

const HINT_FONT_SIZE: f32 = 8.0;
const LABEL_SMALL_FONT_SIZE: f32 = 8.0;
const LABEL_MEDIUM_LARGE_FONT_SIZE: f32 = 10.0;
const SMALL_FONT_SIZE: f32 = 10.0;
const MEDIUM_LARGE_FONT_SIZE: f32 = 13.0;

const DEFAULT_HINT_COLOR: Color = Color::srgb(0.19, 0.49, 0.71);
const WARNING_HINT_COLOR: Color = Color::srgb(0.91, 0.71, 0.);
const ERROR_HINT_COLOR: Color = Color::srgb(0.91, 0., 0.);
const DISABLED_HINT_COLOR: Color = Color::srgb(0.49, 0.53, 0.55);

pub(super) const DEFAULT_BACKGROUND_COLOR: Color = Color::srgb(0.93, 0.97, 1.0);
pub(super) const SELECTED_BORDER_COLOR: Color = Color::srgb(0.51, 0.79, 1.);
pub(super) const SELECTED_BACKGROUND_COLOR: Color = Color::srgb(0.93, 0.97, 1.0);
pub(super) const WARNING_BORDER_COLOR: Color = Color::srgb(1., 0.78, 0.);
pub(super) const WARNING_BACKGROUND_COLOR: Color = Color::srgb(1., 0.98, 0.9);
pub(super) const ERROR_BORDER_COLOR: Color = Color::srgb(1.0, 0.0, 0.);
pub(super) const ERROR_BACKGROUND_COLOR: Color = Color::srgb(1., 0.9, 0.9);
pub(super) const DISABLED_BACKGROUND_COLOR: Color = Color::srgb(0.8, 0.83, 0.85);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub enum TextFieldSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl TextFieldSize {
    pub const fn font_size(&self) -> f32 {
        match self {
            Self::Small => SMALL_FONT_SIZE,
            _ => MEDIUM_LARGE_FONT_SIZE,
        }
    }

    pub const fn hint_font_size(&self) -> f32 {
        HINT_FONT_SIZE
    }

    pub const fn label_font_size(&self) -> f32 {
        match self {
            Self::Small => LABEL_SMALL_FONT_SIZE,
            _ => LABEL_MEDIUM_LARGE_FONT_SIZE,
        }
    }

    pub fn padding(&self) -> UiRect {
        UiRect::axes(Val::Px(16.), Val::Px(8.))
    }

    pub const fn height(&self) -> f32 {
        match self {
            Self::Small => 28.,
            Self::Medium => 36.,
            Self::Large => 42.,
        }
    }

    pub const fn min_width(&self) -> f32 {
        match self {
            Self::Small => 110.,
            Self::Medium => 200.,
            Self::Large => 360.,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum TextState {
    #[default]
    Default,
    Selected,
    Warning,
    Error,
    Disabled,
}

impl TextState {
    pub const fn background_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_BACKGROUND_COLOR,
            Self::Selected => SELECTED_BACKGROUND_COLOR,
            Self::Warning => WARNING_BACKGROUND_COLOR,
            Self::Error => ERROR_BACKGROUND_COLOR,
            Self::Disabled => DISABLED_BACKGROUND_COLOR,
        }
    }

    pub const fn border_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_BACKGROUND_COLOR,
            Self::Selected => SELECTED_BORDER_COLOR,
            Self::Warning => WARNING_BORDER_COLOR,
            Self::Error => ERROR_BORDER_COLOR,
            Self::Disabled => DISABLED_BACKGROUND_COLOR,
        }
    }

    pub const fn hint_color(&self) -> Color {
        match self {
            Self::Default => DEFAULT_HINT_COLOR,
            Self::Selected => DEFAULT_HINT_COLOR,
            Self::Warning => WARNING_HINT_COLOR,
            Self::Error => ERROR_HINT_COLOR,
            Self::Disabled => DISABLED_HINT_COLOR,
        }
    }
}

/// Marks Text Field Node
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component, Debug)]
#[require(
    TextLayout,
    TextStyle,
    TextNodeFlags,
    Node,
    Style, // TODO: Remove when Node uses required components.
    ContentSize, // TODO: Remove when Node uses required components.
    FocusPolicy, // TODO: Remove when Node uses required components.
    ZIndex, // TODO: Remove when Node uses required components.
    Visibility, // TODO: Remove when Node uses required components.
    Transform // TODO: Remove when Node uses required components.
)]
pub struct TextField;

/// Marks Text Field as disabled
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct DisableTextField;

/// Marks Text Field as warning
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct WarningTextField;

/// Marks Text Field as error
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct ErrorTextField;

/// Marks Text Field placeholder
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
#[require(
    TextLayout,
    TextStyle,
    TextNodeFlags,
    Node,
    Style, // TODO: Remove when Node uses required components.
    ContentSize, // TODO: Remove when Node uses required components.
    FocusPolicy, // TODO: Remove when Node uses required components.
    ZIndex, // TODO: Remove when Node uses required components.
    Visibility, // TODO: Remove when Node uses required components.
    Transform // TODO: Remove when Node uses required components.
)]
pub struct Placeholder(pub String);

impl Placeholder {
    pub fn text_style(size: &TextFieldSize) -> TextStyle {
        TextStyle {
            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: size.font_size(),
            color: Color::srgba(0.29, 0.31, 0.33, 0.87),
            ..default()
        }
    }
}

/// Component that owns the string with the field
#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Reflect, Default)]
#[reflect(Component)]
pub struct SingleLineTextField(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TextFieldBuilder {
    size: TextFieldSize,
    label: Option<String>,
    placeholder: Option<String>,
    hint_text: Option<String>,
}

impl TextFieldBuilder {
    pub const fn with_size(mut self, size: TextFieldSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub fn with_placeholder(mut self, placeholder: String) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    pub fn with_hint_text(mut self, hint_text: String) -> Self {
        self.hint_text = Some(hint_text);
        self
    }

    pub fn build(self, commands: &mut Commands) -> Entity {
        commands
            .spawn(NodeBundle {
                style: Style {
                    min_width: Val::Px(self.size.min_width()),
                    height: Val::Auto,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                self.with_text_field(parent);
            })
            .id()
    }

    pub fn child_build(self, commands: &mut ChildBuilder) -> Entity {
        commands
            .spawn(NodeBundle {
                style: Style {
                    min_width: Val::Px(self.size.min_width()),
                    height: Val::Auto,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Relative,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                self.with_text_field(parent);
            })
            .id()
    }

    fn with_text_field(self, parent: &mut ChildBuilder<'_>) {
        parent
            .spawn((
                Clickable,
                SingleLineTextField::default(),
                Placeholder(self.placeholder.clone().unwrap_or_default()),
                self.size,
                NodeBundle {
                    style: Style {
                        min_width: Val::Px(self.size.min_width()),
                        border: UiRect::all(Val::Px(2.0)),
                        height: Val::Px(self.size.height()),
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::Center,
                        padding: self.size.padding(),
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color: TextState::Default.background_color().into(),
                    border_radius: BorderRadius::all(Val::Px(8.)),
                    border_color: TextState::Default.border_color().into(),
                    ..default()
                },
            ))
            .with_children(|builder| {
                if let Some(label) = self.label {
                    builder.spawn((
                        Label,
                        Text::new(label),
                        TextStyle {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: self.size.label_font_size(),
                            color: Color::srgb(0.49, 0.53, 0.55),
                            ..default()
                        },
                    ));
                }

                builder.spawn((
                    TextField,
                    TextStyle {
                        // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: self.size.font_size(),
                        color: Color::srgba(0.29, 0.31, 0.33, 1.0),
                        ..default()
                    },
                ));

                if let Some(placeholder) = self.placeholder {
                    builder.spawn((
                        Visibility::Visible,
                        Placeholder(placeholder.clone()),
                        Text::new(placeholder),
                        Placeholder::text_style(&self.size),
                    ));
                }
            });

        if let Some(hint_text) = self.hint_text {
            // TODO: Fix alignment, not correctly align to the left/start
            parent
                .spawn(NodeBundle {
                    style: Style {
                        left: Val::Px(0.),
                        top: Val::Px(4.),
                        align_items: AlignItems::FlexStart,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::ZERO,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn((
                        Text::new(hint_text),
                        TextStyle {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: self.size.hint_font_size(),
                            color: TextState::Default.hint_color(),
                            ..default()
                        },
                    ));
                });
        }
    }
}
