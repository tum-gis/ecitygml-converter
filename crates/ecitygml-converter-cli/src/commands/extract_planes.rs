use crate::error::Error;
use ecitygml::operations::{GeometryCollector, Visitable};
use ecitygml_converter::io::PlanesWriter;
use std::path::PathBuf;
use std::time::Instant;
use tracing::info;

pub fn run(input_file_path: PathBuf, output_file_path: PathBuf) -> Result<(), Error> {
    info!("Start run on {}", input_file_path.to_str().unwrap());
    let now = Instant::now();
    let citygml_model = ecitygml::io::CitygmlReader::from_path(input_file_path)?.finish()?;
    info!("Read model in {:.3?}", now.elapsed());

    let mut geometry_collector = GeometryCollector::new();
    citygml_model.accept(&mut geometry_collector);

    //let planes_container = PlanesContainer::new();

    let writer = PlanesWriter::from_path(output_file_path)?.with_compressed(false);
    writer.finish(geometry_collector)?;

    Ok(())
}
