use bevy_app::{App, Plugin, Update};
use buttons::button_system;

pub mod buttons;

pub mod prelude {
    pub use super::buttons::ButtonBuilder;
}

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
    }
}
