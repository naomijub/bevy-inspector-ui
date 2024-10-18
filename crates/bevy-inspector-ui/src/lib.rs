#![allow(
    dead_code,
    clippy::redundant_pub_crate,

    // remove
    missing_docs,

    // mandatory
    clippy::needless_pass_by_ref_mut,
    clippy::borrow_interior_mutable_const,
    clippy::type_complexity,
)]

pub mod inspector_options;

pub mod restricted_world_view;
mod utils;

#[doc(inline)]
pub use inspector_options::InspectorOptions;

#[doc(hidden)]
pub mod __macro_exports {
    pub use bevy_reflect;
}

/// Reexports of commonly used types
pub mod prelude {
    // for `#[derive(Reflect)] #[reflect(InspectorOptions)]
    pub use crate::inspector_options::InspectorOptions;
    pub use crate::inspector_options::ReflectInspectorOptions;
}
