use crate::error::Error;
use crate::planes::document::{
    CityObjectClassElement, LevelOfDetailElement, PlanesDocument, PlanesElement,
};
use ecitygml::common::{CityObjectClass, LevelOfDetail};
use ecitygml::operations::{CityObjectGeometryCollection, GeometryCollector};
use egml::model::base::Id;
use egml::model::geometry::MultiSurface;
use rayon::prelude::*;
use std::io::{Cursor, Write};

pub fn write_plane_document<W: Write>(
    mut writer: W,
    geometry_collector: GeometryCollector,
    compression_level: Option<i32>,
) -> Result<(), Error> {
    let mut planes_document = PlanesDocument::default();

    let planes: Vec<PlanesElement> = geometry_collector
        .city_objects
        .par_iter()
        .flat_map(|x| extract_planes_from_city_object_geometry_collection(x.1))
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

fn extract_planes_from_city_object_geometry_collection(
    city_object_geometry_collection: &CityObjectGeometryCollection,
) -> Vec<PlanesElement> {
    city_object_geometry_collection
        .multi_surfaces
        .iter()
        .flat_map(|x| {
            extract_planes_from_multi_surface(
                x.1,
                &city_object_geometry_collection.gml.id,
                *x.0,
                city_object_geometry_collection.class,
            )
        })
        .collect()
}

fn extract_planes_from_multi_surface(
    multi_surface: &MultiSurface,
    city_object_id: &Id,
    level_of_detail: LevelOfDetail,
    city_object_class: CityObjectClass,
) -> Vec<PlanesElement> {
    multi_surface
        .surface_member()
        .iter()
        .map(|x| {
            let current_exterior_ring = &x.exterior;
            let mut plane: PlanesElement = current_exterior_ring.try_into().unwrap();
            plane.city_object_id = Some(city_object_id.clone().into());
            plane.level_of_detail = Some(LevelOfDetailElement::from(level_of_detail));
            plane.city_object_class = Some(CityObjectClassElement::from(city_object_class));
            // let a = current_exterior_ring.normal();
            plane
        })
        .collect()
}
