use crate::error::Error;
use ecitygml_converter::citymodel_to_voxel;
use std::path::PathBuf;
use std::time::Instant;
use tracing::info;

pub fn run(
    input_file_path: PathBuf,
    output_file_path: PathBuf,
    resolution: f64,
    distance_threshold: f64,
) -> Result<(), Error> {
    info!("Start run on {}", input_file_path.to_str().unwrap());
    let now = Instant::now();
    let citygml_model = ecitygml::io::CitygmlReader::from_path(input_file_path)?.finish()?;
    info!("Read model in {:.3?}", now.elapsed());

    let voxel_grid = citymodel_to_voxel(citygml_model, resolution, distance_threshold)?;

    info!("Write voxel grid to {}", &output_file_path.display());
    evoxel::io::EvoxelWriter::new(output_file_path)
        .with_compressed(false)
        .finish(&voxel_grid)?;

    Ok(())
}
