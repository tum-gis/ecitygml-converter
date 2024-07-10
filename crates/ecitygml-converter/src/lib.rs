//! `ecitygml-converter` is a library for converting [CityGML](https://www.ogc.org/standard/citygml/) data to other representations.
//!

pub use ecitygml_converter_core::{citymodel_to_mesh, Error};

#[cfg(feature = "voxel")]
pub use ecitygml_converter_core::citymodel_to_voxel;

#[cfg(feature = "rosbag")]
pub use ecitygml_converter_core::citymodel_to_rosbag;
