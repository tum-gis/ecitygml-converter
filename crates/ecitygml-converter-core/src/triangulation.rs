use ecitygml::model::city_model::CitygmlModel;
use ecitygml::operations::{CityObjectGeometryCollection, GeometryCollector, Visitable};
use egml::model::geometry::TriangulatedSurface;
use egml::operations::triangulate::Triangulate;
use tracing::warn;

pub fn triangulate(city_model: &CitygmlModel) -> Vec<TriangulatedSurface> {
    let mut geometry_collector = GeometryCollector::new();
    city_model.accept(&mut geometry_collector);

    let all_triangulated_surfaces: Vec<TriangulatedSurface> = geometry_collector
        .city_objects
        .values()
        .flat_map(triangulate_city_object_geometry)
        .collect();

    all_triangulated_surfaces
}

fn triangulate_city_object_geometry(
    city_object_geometry_collection: &CityObjectGeometryCollection,
) -> Vec<TriangulatedSurface> {
    let mut all_triangulated_surfaces: Vec<TriangulatedSurface> = Vec::new();

    let triangulated_surfaces: Vec<TriangulatedSurface> = city_object_geometry_collection
        .multi_surfaces
        .values()
        .filter_map(|x| {
            x.triangulate()
                .map_err(|e| {
                    warn!(
                        "error during triangulation of multi_surface ({}) with id: {}",
                        x.gml.id.to_string(),
                        e
                    )
                })
                .ok()
        })
        .collect();
    all_triangulated_surfaces.extend(triangulated_surfaces);

    let triangulated_surfaces: Vec<TriangulatedSurface> = city_object_geometry_collection
        .solids
        .values()
        .filter_map(|x| {
            x.triangulate()
                .map_err(|e| {
                    warn!(
                        "error during triangulation of solid ({}) with id: {}",
                        x.gml.id.to_string(),
                        e
                    )
                })
                .ok()
        })
        .collect();
    all_triangulated_surfaces.extend(triangulated_surfaces);

    all_triangulated_surfaces
}
