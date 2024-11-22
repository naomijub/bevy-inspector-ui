//! # Widgets for Bevy
//! This crate explores the use of Bevy's UI framework with a comprehensive design system for widgets and inspector plugins.
//! Design System was created by UX Jay Kim and can be found at [design system](https://www.figma.com/design/jLdiumully7s5wbCt5rcMb/Space-Editor?node-id=0-1&node-type=canvas&t=GsipYUUPKPYDBLj5-0)
#![allow(
    dead_code,
    clippy::redundant_pub_crate,

    // mandatory to use bevy
    clippy::needless_pass_by_ref_mut,
    clippy::borrow_interior_mutable_const,
    clippy::type_complexity,
)]
use bevy::app::{App, Plugin, Update};
use buttons::{systems::button_system, ButtonClickedEvent};
use clipboard::ClipboardPlugin;
use focus::FocusPlugin;
use text_field::TextInputPlugin;

/// Module containing all button related configuration
pub mod buttons;
/// Module containing all clipboard related configuration
pub mod clipboard;
/// Module containing all focus related configuration
pub mod focus;
/// Module containing all single line text field related configuration
pub mod text_field;
/// Module containing numeric input related configuration
pub mod numeric_field;

/// Plugin for all Bevy widgets
pub struct WidgetsPlugin;

impl Plugin for WidgetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ButtonClickedEvent>()
            // Base/Transversal plugins
            .add_plugins((ClipboardPlugin, FocusPlugin, TextInputPlugin))
            .add_systems(Update, button_system);
    }
}
