#![allow(missing_docs)]
use std::str::FromStr;

use bevy::{prelude::*, winit::WinitSettings};
use bevy_widgets::{
    buttons::{
        prelude::{ButtonBuilder, ButtonRadius, ButtonSize, ButtonType},
        ButtonClickedEvent,
    },
    WidgetsPlugin,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WidgetsPlugin))
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_systems(Startup, setup)
        .add_systems(Update, listener)
        .run();
}

fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2d);
    ButtonBuilder::from_str("hello world")
        .unwrap()
        .with_radius(ButtonRadius::Squared)
        .with_type(ButtonType::Primary)
        .with_size(ButtonSize::Medium)
        // Slow part
        .with_fixed_width(Val::Px(200.))
        .build(&mut commands);
}

fn listener(mut events: EventReader<ButtonClickedEvent>) {
    for event in events.read() {
        info!("{:?} clicked: {}", event.entity, event.value);
    }
}
