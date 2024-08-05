use ecitygml::model::city_model::CitygmlModel;
use ecitygml::operations::{GeometryCollector, Visitable};
use egml::model::geometry::TriangulatedSurface;
use egml::transform::triangulate::{triangulate_multi_surface, triangulate_solid};
use tracing::warn;

pub fn triangulate(city_model: &CitygmlModel) -> Vec<TriangulatedSurface> {
    let mut geometry_collector = GeometryCollector::new();
    city_model.accept(&mut geometry_collector);

    let mut all_triangulated_surfaces: Vec<TriangulatedSurface> = Vec::new();

    /*for current_wall_surface in city_model.building.iter().flat_map(|x| &x.wall_surface) {
        let id = current_wall_surface
            .thematic_surface
            .city_object
            .gml
            .id
            .clone();
        //println!("id: {}", id.to_string());

        if let Some(g) = &current_wall_surface.thematic_surface.lod3_multi_surface {
            let triangulated_surface = triangulate_multi_surface(g).map_err(|e| {
                warn!(
                    "error during triangulation of multi_surface ({}) with id: {}",
                    id.to_string(),
                    e
                )
            });

            if let Ok(o) = triangulated_surface {
                all_triangulated_surfaces.extend(o);
            }
        }

        //for in current_wall_surface.thematic_surface.
    }*/

    let triangulated_surfaces: Vec<TriangulatedSurface> = geometry_collector
        .multi_surface
        .iter()
        .flat_map(|x| {
            triangulate_multi_surface(x)
                .map_err(|e| {
                    warn!(
                        "error during triangulation of multi_surface ({}) with id: {}",
                        x.gml.id.to_string(),
                        e
                    )
                })
                .ok()
        })
        .flatten()
        .collect();
    all_triangulated_surfaces.extend(triangulated_surfaces);

    let triangulated_surfaces: Vec<TriangulatedSurface> = geometry_collector
        .solid
        .iter()
        .flat_map(|x| {
            triangulate_solid(x)
                .map_err(|e| {
                    warn!(
                        "error during triangulation of solid ({}) with id: {}",
                        x.gml.id.to_string(),
                        e
                    )
                })
                .ok()
        })
        .flatten()
        .collect();
    all_triangulated_surfaces.extend(triangulated_surfaces);

    all_triangulated_surfaces
}
