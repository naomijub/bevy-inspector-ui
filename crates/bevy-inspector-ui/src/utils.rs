use bevy_ecs::change_detection::{DetectChangesMut, MutUntyped};
use bevy_ecs::ptr::PtrMut;

// workaround for https://github.com/bevyengine/bevy/pull/6430
pub fn mut_untyped_split(mut mut_untyped: MutUntyped<'_>) -> (PtrMut<'_>, impl FnOnce() + '_) {
    // bypass_change_detection returns a `&mut PtrMut` which is basically useless, because all its methods take `self`
    let ptr = mut_untyped.bypass_change_detection();
    // SAFETY: this is exactly the same PtrMut, just not in a `&mut`. The old one is no longer accessible
    let ptr = unsafe { PtrMut::new(std::ptr::NonNull::new_unchecked(ptr.as_ptr())) };

    (ptr, move || mut_untyped.set_changed())
}

pub mod guess_entity_name {
    use bevy_core::Name;
    use bevy_ecs::{archetype::Archetype, prelude::*, world::unsafe_world_cell::UnsafeWorldCell};
    use bevy_log::error;
    use std::cell::LazyCell;
    use std::collections::HashMap;

    use crate::restricted_world_view::RestrictedWorldView;

    #[allow(clippy::declare_interior_mutable_const)]
    const NAME_ASSOCIATIONS: LazyCell<HashMap<&'static str, String>> = LazyCell::new(|| {
        [
            (
                "bevy_window::window::PrimaryWindow",
                "Primary Window".to_string(),
            ),
            (
                "bevy_core_pipeline::core_3d::camera_3d::Camera3d",
                "Camera3d".to_string(),
            ),
            (
                "bevy_core_pipeline::core_2d::camera_2d::Camera2d",
                "Camera2d".to_string(),
            ),
            ("bevy_pbr::light::PointLight", "PointLight".to_string()),
            (
                "bevy_pbr::light::DirectionalLight",
                "DirectionalLight".to_string(),
            ),
            ("bevy_pbr::light::AmbientLight", "AmbientLight".to_string()),
            ("bevy_text::text::Text", "Text".to_string()),
            ("bevy_ui::ui_node::Node", "Node".to_string()),
            (
                "bevy_asset::handle::Handle<bevy_pbr::pbr_material::StandardMaterial>",
                "Pbr Mesh".to_string(),
            ),
            ("bevy_window::window::Window", "Window".to_string()),
        ]
        .iter()
        .cloned()
        .collect()
    });

    /// Guesses an appropriate entity name like `Light (6)` or falls back to `Entity (8)`
    pub fn guess_entity_name(world: &World, entity: Entity) -> String {
        match world.get_entity(entity) {
            Ok(entity_ref) => entity_ref.get::<Name>().map_or_else(
                || {
                    guess_entity_name_inner(
                        world.as_unsafe_world_cell_readonly(),
                        entity,
                        entity_ref.archetype(),
                    )
                },
                |name| format!("{} ({})", name.as_str(), entity),
            ),

            Err(entity_err) => {
                error!("Failed to get entity: {}", entity_err);
                format!("Entity {} (unregistered)", entity_err.index())
            }
        }
    }

    pub(crate) fn guess_entity_name_restricted(
        world: &mut RestrictedWorldView<'_>,
        entity: Entity,
    ) -> String {
        match world.world().get_entity(entity) {
            Some(cell) => {
                if world.allows_access_to_component((entity, std::any::TypeId::of::<Name>())) {
                    // SAFETY: we have access and don't keep reference
                    if let Some(name) = unsafe { cell.get::<Name>() } {
                        return format!("{} ({})", name.as_str(), entity);
                    }
                }
                guess_entity_name_inner(world.world(), entity, cell.archetype())
            }
            None => format!("Entity {} (inexistent)", entity.index()),
        }
    }

    fn guess_entity_name_inner(
        world: UnsafeWorldCell<'_>,
        entity: Entity,
        archetype: &Archetype,
    ) -> String {
        let type_names = archetype.components().filter_map(|id| {
            let name = world.components().get_info(id)?.name();
            Some(name)
        });

        for component_type in type_names {
            if let Some(name) = NAME_ASSOCIATIONS.get(component_type) {
                return format!("{name} ({entity})");
            }
        }

        format!("Entity ({entity})")
    }
}
