use crate::error::Error;
use crate::planes::document::{PlanesDocument, PlanesElement};
use ecitygml::operations::GeometryCollector;
use rayon::prelude::*;
use std::io::{Cursor, Write};

pub fn write_plane_document<W: Write>(
    mut writer: W,
    geometry_collector: GeometryCollector,
    compression_level: Option<i32>,
) -> Result<(), Error> {
    let mut planes_document = PlanesDocument::default();

    let planes: Vec<PlanesElement> = geometry_collector
        .multi_surface
        .par_iter()
        .flat_map(|x| x.surface_member())
        .map(|x| {
            let current_exterior_ring = &x.exterior;
            let mut plane: PlanesElement = current_exterior_ring.try_into().unwrap();

            plane.parent_id = Some(x.gml.id.clone().into());
            plane

            // let a = current_exterior_ring.normal();
        })
        .collect();
    planes_document.planes = planes;

    let mut info_document_buffer: Vec<u8> = Vec::new();
    if let Some(compression_level) = compression_level {
        serde_json::to_writer(&mut info_document_buffer, &planes_document)?;
        let mut info_document_compressed_buffer: Vec<u8> = Vec::new();
        zstd::stream::copy_encode(
            Cursor::new(info_document_buffer),
            &mut info_document_compressed_buffer,
            compression_level,
        )?;
        writer.write_all(&info_document_compressed_buffer)?;
    } else {
        serde_json::to_writer_pretty(&mut info_document_buffer, &planes_document)?;
        writer.write_all(&info_document_buffer)?;
    }

    //let j = serde_json::to_string(&document)?;
    //serde_json::to_writer_pretty(&mut writer, &planes_document)?;
    Ok(())
}
