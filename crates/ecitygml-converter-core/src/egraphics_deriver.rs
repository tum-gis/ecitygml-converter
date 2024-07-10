use ecitygml::CitygmlModel;

use crate::error::Error;
use crate::triangulation::triangulate;
use nalgebra::Point3;

pub fn citymodel_to_mesh(city_model: CitygmlModel) -> Result<egraphics::TriangleMesh, Error> {
    let all_triangulated_surfaces = triangulate(&city_model);
    let all_triangles: Vec<egraphics::Triangle> = all_triangulated_surfaces
        .iter()
        .flat_map(|s| s.patches())
        .map(convert_to_graphics_triangle)
        .collect();

    let triangle_mesh = egraphics::TriangleMesh::new(all_triangles)?;
    Ok(triangle_mesh)
}

fn convert_to_graphics_triangle(triangle: &egml::geometry::Triangle) -> egraphics::Triangle {
    let points: Vec<Point3<f32>> = triangle
        .points()
        .iter()
        .map(|&p| {
            let point: Point3<f32> = p.into();
            point
        })
        .collect();

    egraphics::Triangle::from(points)
}
