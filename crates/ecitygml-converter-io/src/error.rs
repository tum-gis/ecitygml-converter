use ecitygml::model::common::FeatureType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcitygmlError(#[from] ecitygml::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parsing(#[from] serde_json::Error),

    #[error("file extension is invalid")]
    NoFileExtension(),
    #[error("file extension `{0}` is invalid")]
    InvalidFileExtension(String),
    #[error("`{0}` is not a city object class")]
    NotACityObjectClass(FeatureType),
}
