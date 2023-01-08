use crate::triangulation::triangulate;
use ecitygml::{CitygmlModel, TrafficArea, WallSurface};
use egraphics::core::Triangle;

pub fn citymodel_to_mesh(city_model: CitygmlModel) -> egraphics::core::TriangleMesh {
    let mut all_triangles = Vec::new();

    // TODO improve code quality
    let wall_surfaces_with_geometry: Vec<&WallSurface> = city_model
        .wall_surface()
        .iter()
        .filter(|x| x.lod2_multi_surface().is_some())
        .collect();
    for current_wall_surface in wall_surfaces_with_geometry.iter() {
        let geom = current_wall_surface.lod2_multi_surface().as_ref().unwrap();

        let mut new_triangles: Vec<Triangle> =
            geom.members().iter().flat_map(triangulate).collect();
        all_triangles.append(&mut new_triangles);
    }

    let traffic_areas_with_geometry: Vec<&TrafficArea> = city_model
        .traffic_area()
        .iter()
        .filter(|x| x.lod2_multi_surface().is_some())
        .collect();
    for current_traffic_area in traffic_areas_with_geometry.iter() {
        let geom = current_traffic_area.lod2_multi_surface().as_ref().unwrap();

        let mut new_triangles: Vec<Triangle> =
            geom.members().iter().flat_map(triangulate).collect();
        all_triangles.append(&mut new_triangles);
    }

    egraphics::core::TriangleMesh {
        triangles: all_triangles,
    }
}
