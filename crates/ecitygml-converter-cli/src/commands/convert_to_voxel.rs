use ecitygml_converter::citymodel_to_voxel;
use std::path::PathBuf;
use std::time::Instant;
use tracing::info;

pub fn run(input_file_path: PathBuf, output_file_path: PathBuf) {
    info!("Start run on {}", input_file_path.to_str().unwrap());
    let now = Instant::now();
    let citygml_model = ecitygml::io::CitygmlReader::from_path(input_file_path)
        .unwrap()
        .finish()
        .unwrap();
    info!("Read model in {}ms", now.elapsed().as_millis());

    let voxel_grid = citymodel_to_voxel(citygml_model, 0.1, 0.3).unwrap();

    info!("Write voxel grid to {}", &output_file_path.display());
    evoxel::io::EvoxelWriter::new(output_file_path)
        .with_compressed(false)
        .finish(&voxel_grid)
        .expect("should work");
}
