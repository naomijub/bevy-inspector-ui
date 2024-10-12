use arboard::Clipboard;
use bevy_app::{App, Plugin};
use bevy_derive::{Deref, DerefMut};
use bevy_ecs::system::Resource;

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
