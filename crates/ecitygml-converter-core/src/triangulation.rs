use ecitygml::CitygmlModel;

use tracing::warn;

pub fn triangulate(city_model: &CitygmlModel) -> Vec<egml::geometry::TriangulatedSurface> {
    let mut all_triangulated_surfaces: Vec<egml::geometry::TriangulatedSurface> = Vec::new();

    for current_city_furniture in city_model.city_furniture().iter() {
        if let Some(lod1_solid) = current_city_furniture.lod1_solid() {
            let mut triangulated_surface =
                egml::transform::triangulate::triangulate_solid(lod1_solid).unwrap();
            all_triangulated_surfaces.append(&mut triangulated_surface);
        }

        if let Some(lod2_solid) = current_city_furniture.lod2_solid() {
            let mut triangulated_surface =
                egml::transform::triangulate::triangulate_solid(lod2_solid).unwrap();
            all_triangulated_surfaces.append(&mut triangulated_surface);
        }

        if let Some(lod2_multi_surface) = current_city_furniture.lod2_multi_surface() {
            let mut triangulated_surface =
                egml::transform::triangulate::triangulate_multi_surface(lod2_multi_surface)
                    .unwrap();
            all_triangulated_surfaces.append(&mut triangulated_surface);
        }
    }

    for current_solitary_vegetation_object in city_model.solitary_vegetation_object().iter() {
        if let Some(lod1_solid) = current_solitary_vegetation_object.lod1_solid() {
            let mut triangulated_surface =
                egml::transform::triangulate::triangulate_solid(lod1_solid).unwrap();
            all_triangulated_surfaces.append(&mut triangulated_surface);
        }
    }

    for current_traffic_area in city_model.traffic_area().iter() {
        if let Some(lod2_multi_surface) = current_traffic_area.lod2_multi_surface() {
            let mut triangulated_surface =
                egml::transform::triangulate::triangulate_multi_surface(lod2_multi_surface)
                    .unwrap();
            all_triangulated_surfaces.append(&mut triangulated_surface);
        }
    }

    // TODO improve code quality
    for current_wall_surface in city_model.wall_surface().iter() {
        if let Some(lod2_multi_surface) = current_wall_surface.lod2_multi_surface() {
            let mut triangulated_surface =
                egml::transform::triangulate::triangulate_multi_surface(lod2_multi_surface)
                    .unwrap();
            all_triangulated_surfaces.append(&mut triangulated_surface);
        }

        if let Some(lod3_multi_surface) = current_wall_surface.lod3_multi_surface() {
            match egml::transform::triangulate::triangulate_multi_surface(lod3_multi_surface) {
                Ok(mut triangulated_surface) => {
                    all_triangulated_surfaces.append(&mut triangulated_surface);
                }
                Err(e) => {
                    warn!("triangulation error: {}", e.to_string())
                }
            }
        }
    }

    all_triangulated_surfaces
}
