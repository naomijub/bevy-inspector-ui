#![allow(
    dead_code,
    clippy::redundant_pub_crate,

    // mandatory
    clippy::needless_pass_by_ref_mut,
    clippy::borrow_interior_mutable_const,
    clippy::type_complexity,
)]
use bevy_app::{App, Plugin, Update};
use buttons::button_system;
use clipboard::ClipboardPlugin;
use focus::FocusPlugin;

pub mod buttons;
pub mod clipboard;
pub mod cursor;
pub mod focus;
pub mod text_field;

pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Base/Transversal plugins
            .add_plugins((ClipboardPlugin, FocusPlugin))
            .add_systems(Update, button_system);
    }
}
