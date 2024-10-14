use bevy_app::{App, Plugin};
use bevy_ecs::{
    observer::Trigger,
    prelude::{Component, Entity, Event, Resource},
    query::With,
    system::{Commands, Query},
};
use bevy_input::ButtonInput;
use bevy_picking::{
    pointer::PointerButton,
    prelude::{Click, Pointer},
};

pub struct FocusPlugin;

impl Plugin for FocusPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetFocus>()
            .add_event::<ClearFocus>()
            .add_event::<GotFocus>()
            .add_event::<LostFocus>();

        app.add_systems(
            bevy_app::Update,
            |mut commands: Commands,
             input: bevy_ecs::system::Res<ButtonInput<bevy_input::keyboard::KeyCode>>| {
                if input.just_pressed(bevy_input::keyboard::KeyCode::Escape) {
                    commands.trigger_targets(ClearFocus, Entity::PLACEHOLDER);
                }
            },
        );
        app.add_observer(set_focus)
            .add_observer(clear_focus)
            .add_observer(mouse_click);
    }
}

/// Component which indicates that a widget has focus.
#[derive(Component)]
pub struct Focus;

/// Mark that a widget can receive click input events to add focus
#[derive(Component)]
pub struct Clickable;

/// Event indicating that a widget has received focus event due to click.
/// - Needs manual implementation to react to this triggered event.
/// > Only works automatically if the widget has the [`Clickable`] component
#[derive(Event)]
pub struct GotFocus(pub Option<Pointer<Click>>);

/// Event indicating that a widget has lost focus due to focus or click somewhere else
/// - Needs manual implementation to react to this triggered event
#[derive(Event)]
pub struct LostFocus;

/// Set focus to a widget
/// Event to be called with `commands.set_focus(entity)`
#[derive(Event)]
pub struct SetFocus;

/// Remove focus from widgets
/// Event to be called with `commands.clear_focus()`
#[derive(Event)]
pub struct ClearFocus;

/// Extension trait for [`Commands`]
/// Contains commands to set and clear widget focus
pub trait FocusExt {
    /// Set input focus to the given target
    fn set_focus(&mut self, target: Entity);

    /// Clears focus in all widgets
    fn clear_focus(&mut self);
}

impl FocusExt for Commands<'_, '_> {
    fn set_focus(&mut self, target: Entity) {
        self.trigger_targets(SetFocus, target);
    }

    fn clear_focus(&mut self) {
        self.trigger(ClearFocus);
    }
}

#[derive(Resource)]
struct NeedClearFocus(bool);

fn set_focus(
    trigger: Trigger<SetFocus>,
    mut commands: Commands,
    with_focus: Query<Entity, With<Focus>>,
) {
    let set_entity = trigger.entity();
    for entity in with_focus.iter() {
        if entity == set_entity {
            continue;
        }
        commands.entity(entity).remove::<Focus>();
        commands.trigger_targets(LostFocus, entity);
    }
    commands.entity(set_entity).insert(Focus);
    commands.trigger_targets(GotFocus(None), set_entity);
}

fn clear_focus(
    _: Trigger<ClearFocus>,
    mut commands: Commands,
    focused: Query<Entity, With<Focus>>,
) {
    for entity in focused.iter() {
        commands.entity(entity).remove::<Focus>();
        commands.trigger_targets(LostFocus, entity);
    }
}

fn mouse_click(
    mut click: Trigger<Pointer<Click>>,
    mut commands: Commands,
    clickable_entities: Query<Entity, With<Clickable>>,
    focus_entities: Query<Entity, With<Focus>>,
) {
    if click.event().button != PointerButton::Primary {
        return;
    }

    let entity = click.entity();
    if clickable_entities.contains(entity) {
        click.propagate(false);

        for e in focus_entities.iter() {
            if e == entity {
                continue;
            }
            commands.entity(e).remove::<Focus>();
            commands.trigger_targets(LostFocus, e);
        }
        commands.entity(entity).insert(Focus);
        commands.trigger_targets(GotFocus(Some(click.event().clone())), entity);
    }
}
