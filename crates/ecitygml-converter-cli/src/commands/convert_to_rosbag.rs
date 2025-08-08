use crate::error::Error;
use ecitygml::operations::FeatureWithGeometry;
use ecitygml_converter::citymodel_to_rosbag;
use egml::model::geometry;
use egml::model::geometry::DirectPosition;
use erosbag::Rosbag;
use nalgebra::{Isometry3, Point3, Vector3};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;
use tracing::{info, warn};

pub fn run(
    input_file_path: PathBuf,
    rosbag_directory_path: PathBuf,
    corner_min: Option<Point3<f64>>,
    corner_max: Option<Point3<f64>>,
    translation_offset: Option<Vector3<f64>>,
) -> Result<(), Error> {
    info!("Start run on {}", input_file_path.to_str().unwrap());
    let now = Instant::now();
    let citygml_model = ecitygml::io::CitygmlReader::from_path(input_file_path)?.finish()?;
    info!("Read model in {:.3?}", now.elapsed());

    let envelope = geometry::Envelope::new(
        corner_min.map_or_else(|| DirectPosition::MIN, |c| c.into()),
        corner_max.map_or_else(|| DirectPosition::MAX, |c| c.into()),
    )?;
    let mut citygml_model =
        ecitygml::transform::filter::filter_by_bounding_box(citygml_model, &envelope)?;

    // citygml_model.members.first().unwrap().
    if rosbag_directory_path.exists() {
        warn!("Removing old rosbag");
        fs::remove_dir_all(&rosbag_directory_path)?;
    }

    info!("Open rosbag of path: {}", rosbag_directory_path.display());
    let rosbag = Rosbag::new(&rosbag_directory_path)?;

    if let Some(v) = translation_offset {
        citygml_model.apply_transform(&Isometry3::new(v, Default::default()));
    }
    citymodel_to_rosbag(citygml_model, rosbag);

    Ok(())
}
