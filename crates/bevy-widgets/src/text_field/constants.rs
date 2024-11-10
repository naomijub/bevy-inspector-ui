use bevy::{asset::Handle, color::Color, text::Font};

pub(super) const HINT_FONT_SIZE: f32 = 8.0;
pub(super) const LABEL_SMALL_FONT_SIZE: f32 = 8.0;
pub(super) const LABEL_MEDIUM_LARGE_FONT_SIZE: f32 = 10.0;
pub(super) const SMALL_FONT_SIZE: f32 = 10.0;
pub(super) const MEDIUM_LARGE_FONT_SIZE: f32 = 13.0;

pub(super) const DEFAULT_FONT_COLOR: Color = Color::srgb(0.29, 0.31, 0.33);
pub(super) const PLACEHOLDER_FONT_COLOR: Color = Color::srgb(0.49, 0.53, 0.55);

pub(super) const DEFAULT_HINT_COLOR: Color = Color::srgb(0.19, 0.49, 0.71);
pub(super) const WARNING_HINT_COLOR: Color = Color::srgb(0.91, 0.71, 0.);
pub(super) const ERROR_HINT_COLOR: Color = Color::srgb(0.91, 0., 0.);
pub(super) const DISABLED_HINT_COLOR: Color = Color::srgb(0.49, 0.53, 0.55);

pub(super) const DEFAULT_BACKGROUND_COLOR: Color = Color::srgb(0.93, 0.97, 1.0);
pub(super) const SELECTED_BORDER_COLOR: Color = Color::srgb(0.51, 0.79, 1.);
pub(super) const SELECTED_BACKGROUND_COLOR: Color = Color::srgb(0.93, 0.97, 1.0);
pub(super) const WARNING_BORDER_COLOR: Color = Color::srgb(1., 0.78, 0.);
pub(super) const WARNING_BACKGROUND_COLOR: Color = Color::srgb(1., 0.98, 0.9);
pub(super) const ERROR_BORDER_COLOR: Color = Color::srgb(1.0, 0.0, 0.);
pub(super) const ERROR_BACKGROUND_COLOR: Color = Color::srgb(1., 0.9, 0.9);
pub(super) const DISABLED_BACKGROUND_COLOR: Color = Color::srgb(0.8, 0.83, 0.85);

pub const CURSOR_HANDLE: Handle<Font> = Handle::weak_from_u128(10482756907980398621);
