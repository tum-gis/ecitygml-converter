use crate::error::Error;
use ecitygml::operations::FeatureWithGeometry;
use egml::model::geometry;
use egml::model::geometry::DirectPosition;
use nalgebra::{Isometry3, Point3, Vector3};
use std::path::PathBuf;
use std::process::exit;
use std::time::Instant;
use tracing::{info, warn};

pub fn run(
    input_file_path: PathBuf,
    output_gltf_file_path: PathBuf,
    corner_min: Option<Point3<f64>>,
    corner_max: Option<Point3<f64>>,
    transform: Option<Isometry3<f64>>,
    derive_obj_file: bool,
) -> Result<(), Error> {
    info!("Start run on {}", input_file_path.to_str().unwrap());
    let now = Instant::now();
    let citygml_model = ecitygml::io::CitygmlReader::from_path(input_file_path)?.finish()?;
    info!("Read model in {:.3?}", now.elapsed());

    let filter_envelope = geometry::Envelope::new(
        corner_min.map_or_else(|| DirectPosition::MIN, |c| c.into()),
        corner_max.map_or_else(|| DirectPosition::MAX, |c| c.into()),
    )?;
    let mut citygml_model =
        ecitygml::transform::filter::filter_by_bounding_box(citygml_model, &filter_envelope)?;
    info!(
        "Number of wall elements: {}",
        citygml_model
            .building
            .iter()
            .fold(0, |acc, x| acc + x.wall_surface.len())
    );

    if let Some(isometry) = transform {
        citygml_model.apply_transform(&isometry);
    } else {
        let citygml_model_envelope = citygml_model.envelope().unwrap();
        warn!(
            "No transform defined, applying the lower corner {}",
            citygml_model_envelope.lower_corner()
        );
        let translation: Vector3<f64> = -Vector3::from(citygml_model_envelope.lower_corner());
        citygml_model.apply_transform(&Isometry3::new(translation, Default::default()));
    }

    info!(
        "Number of wall elements after transformed: {}",
        citygml_model
            .building
            .iter()
            .fold(0, |acc, x| acc + x.wall_surface.len())
    );

    let triangle_mesh = ecitygml_converter::citymodel_to_mesh(citygml_model)?;
    if triangle_mesh.is_empty() {
        info!("is empty");
        exit(1);
    }

    let now = Instant::now();
    egraphics::io::EgraphicsExporter::new(output_gltf_file_path)
        .with_derive_obj_file(derive_obj_file)
        .with_create_parent_directories(true)
        .finish(triangle_mesh)?;
    info!("Wrote model in {:.3?}", now.elapsed());

    Ok(())
}
