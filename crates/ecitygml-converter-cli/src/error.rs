use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcitygmlConverterError(#[from] ecitygml_converter::Error),
    #[error(transparent)]
    EcitygmlConverterIoError(#[from] ecitygml_converter::io::Error),

    #[error(transparent)]
    EcitygmlError(#[from] ecitygml::Error),
    #[error(transparent)]
    EcitygmlIoError(#[from] ecitygml::io::Error),
    #[error(transparent)]
    EcitygmlTransformError(#[from] ecitygml::transform::Error),
    #[error(transparent)]
    EgmlError(#[from] egml::Error),
    #[error(transparent)]
    EvoxelError(#[from] evoxel::Error),
    #[error(transparent)]
    EvoxelIoError(#[from] evoxel::io::Error),
    #[error(transparent)]
    EgraphicsError(#[from] egraphics::Error),
    #[error(transparent)]
    EgraphicsIoError(#[from] egraphics::io::Error),
    #[error(transparent)]
    ErosbagError(#[from] erosbag::Error),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
}
