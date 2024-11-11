#![allow(missing_docs)]
use bevy::{prelude::*, winit::WinitSettings};
use bevy_widgets::{text_field::*, WidgetsPlugin};
use builder::{TextInputBuilder, TextInputSize};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WidgetsPlugin))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, listener.after(TextInputSystem))
        .run();
}

fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2d);
    commands
        .spawn(Node {
            // fill the entire window
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..Default::default()
        })
        .with_children(|builder| {
            spawn_node(builder, TextInputSize::Small);
            spawn_node(builder, TextInputSize::Medium);
            builder
                .spawn(Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextInputBuilder::default()
                            .with_placeholder("placeholder".to_string())
                            .with_hint_text("hint text".to_string())
                            .clear_on_submit()
                            .build(),
                    );
                });
            spawn_node(builder, TextInputSize::Large);
        });
}

fn spawn_node(builder: &mut ChildBuilder<'_>, input_size: builder::TextInputSize) {
    builder
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextInputBuilder::default()
                    .with_size(input_size)
                    .with_placeholder("placeholder".to_string())
                    .with_hint_text("hint text".to_string())
                    .with_label("label".to_string())
                    .clear_on_submit()
                    .build(),
            );
        });
}

fn listener(mut events: EventReader<TextInputSubmitEvent>) {
    for event in events.read() {
        info!("{:?} submitted: {}", event.entity, event.value);
    }
}
