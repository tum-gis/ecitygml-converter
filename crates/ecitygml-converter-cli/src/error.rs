use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to read the CityGML dataset `{}`", path.display())]
    ReadInput {
        path: PathBuf,
        #[source]
        source: ecitygml::io::Error,
    },
    #[error("failed to write the output file `{}`", path.display())]
    WriteOutput {
        path: PathBuf,
        #[source]
        source: egraphics::io::Error,
    },

    #[error(transparent)]
    EcitygmlConverter(#[from] ecitygml_converter::Error),
    #[error(transparent)]
    EcitygmlConverterIo(#[from] ecitygml_converter::io::Error),

    #[error(transparent)]
    Ecitygml(#[from] ecitygml::Error),
    #[error(transparent)]
    EcitygmlIo(#[from] ecitygml::io::Error),
    #[error(transparent)]
    EcitygmlTransform(#[from] ecitygml::transform::Error),
    #[error(transparent)]
    Egml(#[from] egml::Error),
    #[error(transparent)]
    Egraphics(#[from] egraphics::Error),
    #[error(transparent)]
    EgraphicsIo(#[from] egraphics::io::Error),

    #[error(transparent)]
    StdIo(#[from] std::io::Error),
}
