use crate::error::Error;
use crate::error::Error::{InvalidFileExtension, NoFileExtension};
use crate::planes::write_impl::write_plane_document;
use crate::planes::{FILE_EXTENSION_JSON_COMPRESSED, FILE_EXTENSION_JSON_UNCOMPRESSED};
use ecitygml::operations::GeometryCollector;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

pub const DEFAULT_COMPRESSION_LEVEL: i32 = 10;

/// `PlanesWriter` sets up a writer for the custom reader data structure.
///
#[derive(Debug, Clone)]
pub struct PlanesWriter<W: Write> {
    writer: W,
    compression_level: Option<i32>,
}

impl<W: Write> PlanesWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            compression_level: Some(DEFAULT_COMPRESSION_LEVEL),
        }
    }

    pub fn with_compressed(mut self, compressed: bool) -> Self {
        if compressed {
            self.compression_level = Some(DEFAULT_COMPRESSION_LEVEL);
        } else {
            self.compression_level = None;
        }
        self
    }

    pub fn finish(self, geometry_collector: GeometryCollector) -> Result<(), Error> {
        write_plane_document(
            BufWriter::new(self.writer),
            geometry_collector,
            self.compression_level,
        )?;
        Ok(())
    }
}

impl PlanesWriter<File> {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let extension = path.as_ref().extension().ok_or(NoFileExtension())?;
        if extension != FILE_EXTENSION_JSON_UNCOMPRESSED
            && extension != FILE_EXTENSION_JSON_COMPRESSED
        {
            return Err(InvalidFileExtension(
                extension.to_str().unwrap_or_default().to_string(),
            ));
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(Self::new(file))
    }
}
