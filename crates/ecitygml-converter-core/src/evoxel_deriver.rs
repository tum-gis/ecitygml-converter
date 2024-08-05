use chrono::Utc;
use ecoord::{ChannelId, Transform, TransformId};
use evoxel::VoxelGridInfo;

use crate::error::Error;
use crate::triangulation::triangulate;
use ecitygml::model::city_model::CitygmlModel;
use ecitygml::operations::FeatureWithGeometry;
use egml::model::geometry::{DirectPosition, Triangle};
use egml::operations::geometry::Geometry;
use itertools::Itertools;
use nalgebra::{Isometry3, Point3, Vector3};
use polars::frame::DataFrame;
use polars::prelude::{NamedFrom, Series};
use rayon::prelude::*;
use std::collections::HashMap;
use tracing::info;

pub fn citymodel_to_voxel(
    mut city_model: CitygmlModel,
    resolution: f64,
    distance_threshold: f64,
) -> Result<evoxel::VoxelGrid, Error> {
    let lower_corner: Vector3<f64> = (*city_model
        .envelope()
        .ok_or(ecitygml::Error::ContainsNoMembers("".to_string()))?
        .lower_corner())
    .into();

    city_model.apply_transform(&Isometry3::new(-lower_corner, Default::default()));

    //let resolution = 0.1;
    //let _envelope = local_city_model.get_envelope();
    //let _steps: Vector3<f64> = envelope.size() / resolution;
    //let _size = envelope.size();
    //info!("size: ")

    // see: https://stackoverflow.com/questions/21638509/determine-voxels-that-a-triangle-is-in
    // https://github.com/davidstutz/mesh-voxelization

    //let corner_min = city_model.get_min();
    //let corner_max = city_model.get_();

    let all_triangulated_surfaces = triangulate(&city_model);
    let all_triangles: Vec<&Triangle> = all_triangulated_surfaces
        .iter()
        .flat_map(|t| t.patches())
        .collect();
    info!("number of triangles: {}", &all_triangles.len());

    // https://docs.rs/rayon/latest/rayon/iter/trait.ParallelIterator.html#method.fold
    let occupied_voxels: Vec<Point3<i64>> = all_triangles
        .par_iter()
        .flat_map(|t| derive_occupancy_indexes(t, resolution, distance_threshold))
        .collect();
    info!("number of occupied voxels: {}", &occupied_voxels.len());
    let occupied_voxels: Vec<Point3<i64>> = occupied_voxels.into_iter().unique().collect();

    //let a = derive_occupancy_indexes(current_triangle, resolution);
    // https://docs.rs/parry3d-f64/latest/parry3d_f64/query/trait.PointQuery.html#method.distance_to_local_point
    // https://stackoverflow.com/questions/2924795/fastest-way-to-compute-point-to-triangle-distance-in-3d
    // https://rapier.rs/docs/user_guides/rust/advanced_collision_detection

    //all_triangles.first().unwrap().env

    let x_series = Series::new(
        evoxel::VoxelDataColumnNames::X.as_str(),
        occupied_voxels.iter().map(|v| v.x).collect::<Vec<i64>>(),
    );
    let y_series = Series::new(
        evoxel::VoxelDataColumnNames::Y.as_str(),
        occupied_voxels.iter().map(|v| v.y).collect::<Vec<i64>>(),
    );
    let z_series = Series::new(
        evoxel::VoxelDataColumnNames::Z.as_str(),
        occupied_voxels.iter().map(|v| v.z).collect::<Vec<i64>>(),
    );
    let data: DataFrame = DataFrame::new(vec![x_series, y_series, z_series]).unwrap();
    //let now = Instant::now();
    /*let data = data
    .unique(
        Some(&[
            evoxel::VoxelDataColumnNames::X.as_str().to_string(),
            evoxel::VoxelDataColumnNames::Y.as_str().to_string(),
            evoxel::VoxelDataColumnNames::Z.as_str().to_string(),
        ]),
        UniqueKeepStrategy::First,
    )
    .unwrap();*/
    //info!(
    //    "height of df: {} in {}ms",
    //    &data.height(),
    //    now.elapsed().as_millis()
    //);

    let info: VoxelGridInfo = VoxelGridInfo::new("local".into(), resolution, None, None, None);

    let channel_id = ecoord::ChannelId::from("world");
    let transform_id = ecoord::TransformId::new("world".into(), "local".into());
    let transform = ecoord::Transform::new(Utc::now(), lower_corner, Default::default());
    let transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> =
        HashMap::from([((channel_id, transform_id), vec![transform])]);
    let frames = ecoord::ReferenceFrames::new(
        transforms,
        Default::default(),
        Default::default(),
        Default::default(),
    )
    .unwrap();

    let voxel_grid = evoxel::VoxelGrid::new(data, info, frames)?;
    Ok(voxel_grid)
}

fn derive_occupancy_indexes(
    triangle: &Triangle,
    resolution: f64,
    distance_threshold: f64,
) -> Vec<Point3<i64>> {
    let envelope = triangle.envelope().enlarge(distance_threshold).unwrap();

    let lower_corner_index: nalgebra::Point3<i64> = Point3::new(
        (envelope.lower_corner().x() / resolution).floor() as i64,
        (envelope.lower_corner().y() / resolution).floor() as i64,
        (envelope.lower_corner().z() / resolution).floor() as i64,
    );

    let upper_corner_index: nalgebra::Point3<i64> = Point3::new(
        (envelope.upper_corner().x() / resolution).ceil() as i64,
        (envelope.upper_corner().y() / resolution).ceil() as i64,
        (envelope.upper_corner().z() / resolution).ceil() as i64,
    );

    /*info!(
        "lower_corner_index: {}, {}, {}; upper_corner_index: {}, {}, {}",
        lower_corner_index.x,
        lower_corner_index.y,
        lower_corner_index.z,
        upper_corner_index.x,
        upper_corner_index.y,
        upper_corner_index.z
    );*/

    let mut occupancy_indexes: Vec<Point3<i64>> = Vec::new();

    for current_x_index in lower_corner_index.x..=upper_corner_index.x {
        for current_y_index in lower_corner_index.y..=upper_corner_index.y {
            for current_z_index in lower_corner_index.z..=upper_corner_index.z {
                let voxel_center_point = DirectPosition::new(
                    resolution * current_x_index as f64,
                    resolution * current_y_index as f64,
                    resolution * current_z_index as f64,
                )
                .unwrap();

                let distance = triangle.distance_to_local_point(&voxel_center_point);
                if distance < distance_threshold {
                    occupancy_indexes.push(Point3::new(
                        current_x_index,
                        current_y_index,
                        current_z_index,
                    ));
                }
                // info!("distance: {distance}");
            }
        }
    }

    // let t: parry3d_f64::shape::Triangle = current_triangle.clone().into();

    //info!("length: {}", occupancy_indexes.len());
    //info!("area: {}", triangle.area());

    occupancy_indexes
}
