mod arguments;
mod commands;

extern crate quick_xml;
extern crate serde;

use crate::arguments::{Arguments, Commands};
use crate::commands::convert_to_graphics;
#[cfg(feature = "rosbag")]
use crate::commands::convert_to_rosbag;
#[cfg(feature = "voxel")]
use crate::commands::convert_to_voxel;
use clap::Parser;
use nalgebra::Point3;
use nalgebra::Vector3;
use std::path::{Path, PathBuf};
use tracing::info;

fn main() {
    tracing_subscriber::fmt::init();
    let arguments = Arguments::parse();

    match &arguments.command {
        Commands::ConvertToGraphics {
            input_path,
            output_path,
            corner_min,
            corner_max,
            offset,
        } => {
            info!("Transform to a graphics format.");
            let input_file_path = Path::new(&input_path).canonicalize().unwrap();

            let corner_min: Option<Point3<f64>> =
                corner_min.as_ref().map(|v| Point3::new(v[0], v[1], v[2]));
            let corner_max: Option<Point3<f64>> =
                corner_max.as_ref().map(|v| Point3::new(v[0], v[1], v[2]));
            let translation_offset: Option<Vector3<f64>> =
                offset.as_ref().map(|v| Vector3::new(v[0], v[1], v[2]));

            let output_gltf_file_path = if output_path.is_some() {
                PathBuf::from(output_path.as_ref().unwrap())
            } else {
                let target_directory_name = input_file_path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned()
                    + "_graphic_formats";
                input_file_path
                    .parent()
                    .unwrap()
                    .join(target_directory_name)
            }
            .join(PathBuf::from("model.gltf"));

            convert_to_graphics::run(
                input_file_path,
                output_gltf_file_path,
                corner_min,
                corner_max,
                translation_offset,
            );
        }
        #[cfg(feature = "rosbag")]
        Commands::ConvertToRosbag {
            input_path,
            rosbag_directory_path,
            corner_min,
            corner_max,
            offset,
        } => {
            info!("Transform to a ROS bag");
            let input_file_path = Path::new(&input_path).canonicalize().unwrap();

            let corner_min: Option<Point3<f64>> =
                corner_min.as_ref().map(|v| Point3::new(v[0], v[1], v[2]));
            let corner_max: Option<Point3<f64>> =
                corner_max.as_ref().map(|v| Point3::new(v[0], v[1], v[2]));
            let translation_offset: Option<Vector3<f64>> =
                offset.as_ref().map(|v| Vector3::new(v[0], v[1], v[2]));

            let output_gltf_file_path = PathBuf::from(rosbag_directory_path);

            convert_to_rosbag::run(
                input_file_path,
                output_gltf_file_path,
                corner_min,
                corner_max,
                translation_offset,
            );
        }
        #[cfg(feature = "voxel")]
        Commands::ConvertToVoxel {
            input_path,
            output_path,
        } => {
            let input_file_path = Path::new(&input_path).canonicalize().unwrap();
            let output_directory_path = PathBuf::from(&output_path);

            convert_to_voxel::run(input_file_path, output_directory_path);
        }
    }
}
