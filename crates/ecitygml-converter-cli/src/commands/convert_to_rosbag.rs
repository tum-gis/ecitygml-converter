use ecitygml_converter::citymodel_to_rosbag;
use egml::geometry::DirectPosition;
use erosbag::RosbagOpenOptions;
use nalgebra::{Point3, Vector3};
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
) {
    info!("Start run on {}", input_file_path.to_str().unwrap());
    let now = Instant::now();
    let citygml_model = ecitygml::io::CitygmlReader::from_path(input_file_path)
        .unwrap()
        .finish()
        .unwrap();
    info!("Read model in {}ms", now.elapsed().as_millis());

    let envelope = egml::geometry::Envelope::new(
        corner_min.map_or_else(|| DirectPosition::MIN, |c| c.into()),
        corner_max.map_or_else(|| DirectPosition::MAX, |c| c.into()),
    )
    .unwrap();
    let citygml_model =
        ecitygml::transform::filter::filter_by_bounding_box(citygml_model, &envelope).unwrap();

    // citygml_model.members.first().unwrap().
    if rosbag_directory_path.exists() {
        warn!("Removing old rosbag");
        fs::remove_dir_all(&rosbag_directory_path).expect("TODO: panic message");
    }

    info!("Open rosbag of path: {}", rosbag_directory_path.display());
    let rosbag = RosbagOpenOptions::new()
        .create_new(true)
        .open(&rosbag_directory_path)
        .unwrap();

    let transformed_citygml_model = match translation_offset {
        Some(v) => ecitygml::transform::offset::offset_citygml_model(citygml_model, &v).unwrap(),
        _ => citygml_model,
    };
    citymodel_to_rosbag(transformed_citygml_model, rosbag);
}
