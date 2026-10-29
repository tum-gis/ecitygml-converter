use crate::error::Error;
use crate::planes::document::{PlanesDocument, PlanesElement};
use ecitygml::arena::CityModelArena;
use ecitygml::model::common::{FeatureType, LevelOfDetail};
use egml::model::base::Id;
use egml::model::geometry::aggregates::MultiSurface;
use std::io::Write;

pub fn write_plane_document<W: Write>(
    _writer: W,
    _city_object_arena: CityModelArena,
    _compression_level: Option<i32>,
) -> Result<(), Error> {
    let _planes_document = PlanesDocument::default();

    /*let planes: Vec<PlanesElement> = city_object_arena
        .features()
        .values()
        .par_iter()
        .flat_map(extract_planes_from_city_object_geometry)
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
    }*/

    //let j = serde_json::to_string(&document)?;
    //serde_json::to_writer_pretty(&mut writer, &planes_document)?;
    Ok(())
}

/*fn extract_planes_from_city_object_geometry(
    city_object_geometry_entry: &CityObjectGeometryEntry,
) -> Vec<PlanesElement> {
    city_object_geometry_entry
        .multi_surfaces
        .iter()
        .flat_map(|x| {
            extract_planes_from_multi_surface(
                x.1,
                city_object_geometry_entry.id(),
                *x.0,
                city_object_geometry_entry.class,
            )
        })
        .collect()
}*/

#[expect(dead_code, reason = "not used yet")]
fn extract_planes_from_multi_surface(
    _multi_surface: &MultiSurface,
    _city_object_id: &Id,
    _level_of_detail: LevelOfDetail,
    _city_object_class: FeatureType,
) -> Vec<PlanesElement> {
    todo!("improve");

    /*multi_surface
    .surface_member()
    .iter()
    .map(|x| {
        let mut plane: PlanesElement = x.try_into().unwrap();
        plane.city_object_id = Some(city_object_id.clone().into());
        plane.level_of_detail = Some(LevelOfDetailElement::from(level_of_detail));
        plane.city_object_class = CityObjectClassElement::try_from(city_object_class).ok();
        // let a = current_exterior_ring.normal();
        plane
    })
    .collect()*/
}
