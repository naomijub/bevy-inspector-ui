#![allow(missing_docs)]
use bevy::{prelude::*, winit::WinitSettings};
use bevy_widgets::{
    buttons::prelude::{self as button, ButtonRadius, ButtonSize, ButtonType, SpawnButton},
    WidgetsPlugin,
};

const MARGIN: Val = Val::Px(12.);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WidgetsPlugin))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2d);
    commands.spawn_button("text", button::Default);

    commands
        .spawn(NodeBundle {
            style: Style {
                // fill the entire window
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                padding: UiRect::all(MARGIN),
                row_gap: MARGIN,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Percent(100.),
                        flex_direction: FlexDirection::Column,
                        row_gap: MARGIN,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    // spawn one child node for each combination of `AlignItems` and `JustifyContent`
                    let types = [
                        (ButtonType::Primary, ButtonRadius::Squared),
                        (ButtonType::Primary, ButtonRadius::Rounded),
                        (ButtonType::Secondary, ButtonRadius::Squared),
                        (ButtonType::Secondary, ButtonRadius::Rounded),
                        (ButtonType::Tertiary, ButtonRadius::Squared),
                        (ButtonType::Tertiary, ButtonRadius::Rounded),
                    ];
                    let sizes = [ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];
                    for sizes in sizes {
                        builder
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(100.),
                                    height: Val::Percent(100.),
                                    flex_direction: FlexDirection::Row,
                                    column_gap: MARGIN,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|builder| {
                                for ty in types {
                                    spawn_new_button(builder, &sizes, ty);
                                }
                            });
                    }
                });
        });
}

fn spawn_new_button(builder: &mut ChildBuilder, size: &ButtonSize, ty: (ButtonType, ButtonRadius)) {
    builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..Default::default()
            },
            background_color: BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
            ..Default::default()
        })
        .with_children(|builder| match (size, ty.0, ty.1) {
            (ButtonSize::Small, ButtonType::Primary, ButtonRadius::Squared) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Primary, button::Small, button::Squared),
                );
            }
            (ButtonSize::Small, ButtonType::Primary, ButtonRadius::Rounded) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Primary, button::Small, button::Rounded),
                );
            }
            (ButtonSize::Small, ButtonType::Secondary, ButtonRadius::Squared) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Secondary, button::Small, button::Squared),
                );
            }
            (ButtonSize::Small, ButtonType::Secondary, ButtonRadius::Rounded) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Secondary, button::Small, button::Rounded),
                );
            }
            (ButtonSize::Small, ButtonType::Tertiary, ButtonRadius::Squared) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Tertiary, button::Small, button::Squared),
                );
            }
            (ButtonSize::Small, ButtonType::Tertiary, ButtonRadius::Rounded) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Tertiary, button::Small, button::Rounded),
                );
            }
            (ButtonSize::Medium, ButtonType::Primary, ButtonRadius::Squared) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Primary, button::Medium, button::Squared),
                );
            }
            (ButtonSize::Medium, ButtonType::Primary, ButtonRadius::Rounded) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Primary, button::Medium, button::Rounded),
                );
            }
            (ButtonSize::Medium, ButtonType::Secondary, ButtonRadius::Squared) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Secondary, button::Medium, button::Squared),
                );
            }
            (ButtonSize::Medium, ButtonType::Secondary, ButtonRadius::Rounded) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Secondary, button::Medium, button::Rounded),
                );
            }
            (ButtonSize::Medium, ButtonType::Tertiary, ButtonRadius::Squared) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Tertiary, button::Medium, button::Squared),
                );
            }
            (ButtonSize::Medium, ButtonType::Tertiary, ButtonRadius::Rounded) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Tertiary, button::Medium, button::Rounded),
                );
            }
            (ButtonSize::Large, ButtonType::Primary, ButtonRadius::Squared) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Primary, button::Large, button::Squared),
                );
            }
            (ButtonSize::Large, ButtonType::Primary, ButtonRadius::Rounded) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Primary, button::Large, button::Rounded),
                );
            }
            (ButtonSize::Large, ButtonType::Secondary, ButtonRadius::Squared) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Secondary, button::Large, button::Squared),
                );
            }
            (ButtonSize::Large, ButtonType::Secondary, ButtonRadius::Rounded) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Secondary, button::Large, button::Rounded),
                );
            }
            (ButtonSize::Large, ButtonType::Tertiary, ButtonRadius::Squared) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Tertiary, button::Large, button::Squared),
                );
            }
            (ButtonSize::Large, ButtonType::Tertiary, ButtonRadius::Rounded) => {
                builder.spawn_button(
                    column_name(size),
                    (button::Tertiary, button::Large, button::Rounded),
                );
            }
        });
}

fn column_name(size: &ButtonSize) -> String {
    match size {
        ButtonSize::Small => String::from("Small"),
        ButtonSize::Medium => String::from("Medium"),
        ButtonSize::Large => String::from("Large"),
    }
}
