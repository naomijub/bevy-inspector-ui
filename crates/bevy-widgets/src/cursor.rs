use bevy_app::{App, Plugin, Update};
use bevy_ecs::{
    component::Component,
    query::With,
    system::{Query, Res},
};
use bevy_hierarchy::{Children, Parent};
use bevy_render::view::Visibility;
use bevy_time::{Time, Timer, TimerMode};

use crate::focus::Focus;

pub(crate) struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_cursor);
    }
}

#[derive(Component)]
pub(crate) struct Cursor {
    timer: Timer,
}

impl Default for Cursor {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        }
    }
}

pub(crate) fn update_cursor(
    time: Res<Time>,
    mut cursors: Query<(&mut Cursor, &mut Visibility, &Parent)>,
    focused_entities: Query<&Focus, With<Children>>,
) {
    for (mut cursor, mut visibility, parent) in cursors.iter_mut() {
        if focused_entities.get(parent.get()).is_err() {
            continue;
        }
        if cursor.timer.tick(time.delta()).just_finished() {
            *visibility = cursor_visibility(*visibility);
        }
    }
}

fn cursor_visibility(visibility: Visibility) -> Visibility {
    match visibility {
        Visibility::Inherited => Visibility::Hidden,
        Visibility::Hidden => Visibility::Visible,
        Visibility::Visible => Visibility::Hidden,
    }
}
