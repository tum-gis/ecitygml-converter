use crate::cli::ConvertToGraphicsArgs;
use crate::error::Error;
use std::time::Instant;
use tracing::info;

pub fn run(args: &ConvertToGraphicsArgs) -> Result<(), Error> {
    let lod_filter = args.lod_filter()?;

    info!("Start run on {}", args.input_path.display());
    let now = Instant::now();
    let city_model = ecitygml::io::GmlReader::from_path(&args.input_path)
        .and_then(|reader| {
            reader
                .with_rebuild_object_bounds(!args.skip_rebuilding_object_bounds)
                .finish()
        })
        .map_err(|source| Error::ReadInput {
            path: args.input_path.clone(),
            source,
        })?;
    info!("Read model in {:.3?}", now.elapsed());

    let translation = args.translation_vector(&city_model);
    if let Some(t) = translation {
        info!("Translating by {} {} {}", t.x, t.y, t.z);
    }

    let scene = ecitygml_converter::derive_scene_from_city_model(
        city_model,
        args.corner_min(),
        args.corner_max(),
        translation,
        &lod_filter,
        args.include_multi_surfaces(),
        args.include_solids(),
    )?;

    info!("Writing to {}", args.output_path.display());
    let now = Instant::now();
    egraphics::io::EgraphicsWriter::from_path(&args.output_path)
        .and_then(|writer| {
            writer
                .with_mesh_grouping(args.mesh_grouping())
                .finish(scene)
        })
        .map_err(|source| Error::WriteOutput {
            path: args.output_path.clone(),
            source,
        })?;
    info!("Wrote model in {:.3?}", now.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::cli::{Cli, Commands};
    use clap::Parser;
    use std::path::Path;

    fn run_with_paths(input: &Path, output: &Path) -> Result<(), Error> {
        let cli = Cli::try_parse_from([
            "ecitygml-converter-cli".as_ref(),
            "convert-to-graphics".as_ref(),
            "-i".as_ref(),
            input.as_os_str(),
            "-o".as_ref(),
            output.as_os_str(),
            "--translation-mode".as_ref(),
            "none".as_ref(),
        ])
        .expect("valid arguments");
        let Commands::ConvertToGraphics(args) = cli.command;
        run(&args)
    }

    #[test]
    fn read_error_names_the_input_path() {
        let input = Path::new("missing-input-file.gml");
        let error = run_with_paths(input, Path::new("city.egraphics")).unwrap_err();
        assert!(matches!(error, Error::ReadInput { .. }));
        assert!(error.to_string().contains("missing-input-file.gml"));
    }
}
