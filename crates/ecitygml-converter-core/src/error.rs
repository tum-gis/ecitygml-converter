use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcitygmlError(#[from] ecitygml::Error),
    #[error(transparent)]
    EgmlError(#[from] egml::Error),
    #[cfg(feature = "voxel")]
    #[error(transparent)]
    EvoxelError(#[from] evoxel::Error),
    #[error(transparent)]
    EgraphicsError(#[from] egraphics::Error),
}
