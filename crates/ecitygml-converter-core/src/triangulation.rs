use egml::geometry::LinearRing;
use itertools::Itertools;
use nalgebra::Point3;

// TODO: implement triangulation in `egml` crate

pub fn triangulate(linear_ring: &LinearRing) -> Vec<egraphics::core::Triangle> {
    let points: Vec<Point3<f32>> = linear_ring
        .points()
        .iter()
        .map(|p| Point3::<f32>::from(*p))
        .collect();

    let first_point = *points.first().unwrap();
    let mut triangles: Vec<egraphics::core::Triangle> = Vec::new();

    for (prev, next) in points
        .iter()
        .skip(1)
        .collect::<Vec<&Point3<f32>>>()
        .iter()
        .tuple_windows()
    {
        //print!("{:?}", point_chunks);

        let new_triangle = egraphics::core::Triangle::from(vec![first_point, **prev, **next]);

        triangles.push(new_triangle);
    }

    //print!("{:?}", triangles);
    //let a = linear_ring.iter().skip(1).collect::<Vec<&Point3<f32>>>().chunks(2);
    //println!("{:?}", a);
    triangles
}
