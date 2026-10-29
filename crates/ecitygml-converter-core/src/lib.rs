mod egraphics_deriver;
mod error;
mod triangulation;

#[doc(inline)]
pub use error::Error;

#[doc(inline)]
pub use triangulation::triangulate;

#[doc(inline)]
pub use egraphics_deriver::derive_scene_from_city_model;
