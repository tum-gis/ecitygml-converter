//! `ecitygml-converter` is a library for converting [CityGML](https://www.ogc.org/standards/citygml/) data to other representations.
//!

pub use ecitygml_converter_core::{Error, citymodel_to_mesh, triangulate};

#[cfg(feature = "voxel")]
pub use ecitygml_converter_core::citymodel_to_voxel;

#[cfg(feature = "rosbag")]
pub use ecitygml_converter_core::citymodel_to_rosbag;

pub use ecitygml_converter_io as io;
