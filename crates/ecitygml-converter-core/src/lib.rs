mod egraphics_deriver;
#[cfg(feature = "rosbag")]
mod erosbag_deriver;
mod error;
#[cfg(feature = "voxel")]
mod evoxel_deriver;

mod triangulation;

#[doc(inline)]
pub use error::Error;

#[doc(inline)]
pub use egraphics_deriver::citymodel_to_mesh;

#[cfg(feature = "rosbag")]
#[doc(inline)]
pub use erosbag_deriver::citymodel_to_rosbag;

#[cfg(feature = "voxel")]
#[doc(inline)]
pub use evoxel_deriver::citymodel_to_voxel;
