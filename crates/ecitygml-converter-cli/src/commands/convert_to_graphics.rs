use nalgebra::{Point3, Vector3};
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;
use tracing::info;

pub fn run(
    input_file_path: PathBuf,
    output_gltf_file_path: PathBuf,
    corner_min: Option<Point3<f64>>,
    corner_max: Option<Point3<f64>>,
    translation_offset: Option<Vector3<f64>>,
) {
    info!("Start run on {}", input_file_path.to_str().unwrap());
    let now = Instant::now();
    let citygml_model = ecitygml::io::CitygmlReader::new(input_file_path)
        .with_corner_min(corner_min)
        .with_corner_max(corner_max)
        .finish()
        .unwrap();
    info!("Read model in {}ms", now.elapsed().as_millis());

    let transformed_citygml_model = match translation_offset {
        Some(v) => ecitygml::transform::offset::offset_citygml_model(citygml_model, &v).unwrap(),
        _ => citygml_model,
    };

    let triangle_mesh = ecitygml_converter::citymodel_to_mesh(transformed_citygml_model);
    if triangle_mesh.is_empty() {
        info!("is empty");
        exit(1);
    }

    egraphics::io::EgraphicsExporter::new(output_gltf_file_path)
        .with_derive_obj_file(true)
        .with_create_parent_directories(true)
        .finish(triangle_mesh)
        .unwrap();
}
