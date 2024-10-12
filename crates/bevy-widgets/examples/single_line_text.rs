use bevy::{prelude::*, winit::WinitSettings};
use bevy_widgets::{text_field::*, WidgetsPlugin};

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
    TextFieldBuilder::default()
        .with_hint_text("hint_text".to_string())
        .with_label("label".to_string())
        .with_placeholder("placeholder".to_string())
        .build(&mut commands);
}
