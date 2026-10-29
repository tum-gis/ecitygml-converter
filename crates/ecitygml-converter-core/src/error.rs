use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcitygmlError(#[from] ecitygml::Error),
    #[error(transparent)]
    EgmlError(#[from] egml::Error),

    #[error(transparent)]
    EgraphicsError(#[from] egraphics::Error),
    #[error("solid has no exterior shell")]
    SolidMissingExterior,
    #[error("exterior shell has no surface member")]
    ShellMissingSurface,
    #[error("geometry reference could not be resolved (unresolved or broken xlink)")]
    UnresolvedGeometryReference,
}
