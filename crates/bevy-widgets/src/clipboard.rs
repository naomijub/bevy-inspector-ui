use arboard::Clipboard;
use bevy::app::{App, Plugin};
use bevy::ecs::system::Resource;
use bevy::prelude::{Deref, DerefMut};

/// Plugin containing the copy+paste from clipboard
pub struct ClipboardPlugin;

impl Plugin for ClipboardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ClipboardContext>();
    }
}

/// Contains clipboard api instance
#[derive(Resource, Deref, DerefMut)]
pub struct ClipboardContext(pub Clipboard);

impl Default for ClipboardContext {
    fn default() -> Self {
        Self(Clipboard::new().unwrap())
    }
}
