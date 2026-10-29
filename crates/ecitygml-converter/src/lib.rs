//! `ecitygml-converter` is a library for converting [CityGML](https://www.ogc.org/standards/citygml/) data to other representations.
//!

pub use ecitygml_converter_core::{Error, derive_scene_from_city_model, triangulate};

pub use ecitygml_converter_io as io;
