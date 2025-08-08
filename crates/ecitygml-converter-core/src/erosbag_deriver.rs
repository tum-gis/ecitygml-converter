use erosbag::Rosbag;
use erosbag::ros_messages::std_msgs;
use tracing::info;

use chrono::{DateTime, Utc};

use erosbag::ros_messages::builtin_msgs::Duration;
use erosbag::ros_messages::geometry_msgs::Pose;
use erosbag::ros_messages::visualization_msgs::{
    Marker, MarkerActionType, MarkerArray, MarkerObjectType,
};
use nalgebra::{Isometry3, Point3, Quaternion, Translation3, UnitQuaternion, Vector3};

use ecitygml::model::city_model::CitygmlModel;
use ecitygml::model::transportation::TrafficSpace;
use egml::model::geometry::LinearRing;
use egml::operations::geometry::Geometry;

pub fn citymodel_to_rosbag(city_model: CitygmlModel, rosbag: Rosbag) {
    info!("Number of objects: {}", city_model.number_of_objects());

    let topic_name: String = "/marker_array".to_string();
    todo!("migrate to ROS bag MCAP encoding");
    /*rosbag
        .create_topic(
            &topic_name,
            &TopicMetadata::new(
                RosMessageType::VisualizationMessagesMarkerArray,
                TopicSerializationFormat::CDR,
                vec![QualityOfServiceProfile::new_for_static_tf_topic()],
            ),
        )?;

    for current_step in 0..2 {
        let timestamp =
            Utc.timestamp_opt(1579007185, 0).unwrap() + chrono::Duration::seconds(current_step);
        let marker_message_array =
            create_marker_array(&city_model, current_step * 10000, timestamp);
        rosbag
            .append_message(&topic_name, &marker_message_array)?;
    }

    rosbag.close();*/
}

fn create_marker_array(
    city_model: &CitygmlModel,
    _start_id: i64,
    timestamp: DateTime<Utc>,
) -> MarkerArray {
    let mut marker_message_array = MarkerArray::default();
    let mut vert_id = 0;

    for current_member in city_model
        .building
        .iter()
        .flat_map(|x| &x.wall_surface)
        .filter(|x| x.thematic_surface.lod2_multi_surface.is_some())
    {
        for current_geometry in current_member
            .thematic_surface
            .lod2_multi_surface
            .as_ref()
            .unwrap()
            .surface_member()
            .iter()
            .map(|x| &x.exterior)
        {
            let marker_message = get_marker_message(
                vert_id,
                current_geometry,
                timestamp,
                std_msgs::ColorRGBA {
                    r: 0.0,
                    g: 0.0,
                    b: 1.0,
                    a: 1.0,
                },
            );
            vert_id += current_geometry.points().len() as i64;
            marker_message_array.markers.push(marker_message);
        }
    }

    for current_member in city_model
        .road
        .iter()
        .flat_map(|x| {
            let mut traffic_spaces: Vec<&TrafficSpace> =
                x.section.iter().flat_map(|y| &y.traffic_space).collect();
            traffic_spaces.extend(x.intersection.iter().flat_map(|y| &y.traffic_space));
            traffic_spaces
        })
        .flat_map(|x| &x.traffic_area)
        .filter(|x| x.thematic_surface.lod2_multi_surface.is_some())
    {
        for current_geometry in current_member
            .thematic_surface
            .lod2_multi_surface
            .as_ref()
            .unwrap()
            .surface_member()
            .iter()
            .map(|x| &x.exterior)
        {
            let marker_message = get_marker_message(
                vert_id,
                current_geometry,
                timestamp,
                std_msgs::ColorRGBA {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0,
                    a: 1.0,
                },
            );
            vert_id += current_geometry.points().len() as i64;
            marker_message_array.markers.push(marker_message);
        }
    }

    /*for current_step in 0..10 {
        let georeferenced_position =
            Vector3::<f64>::new(678123.3962647032, 5403660.837644646, 416.92890148324415);
        //let point: Vector3<f64> =
        //    georeferenced_position + translation.unwrap_or(Vector3::new(0.0, 0.0, 0.0));

        let point: Vector3<f64> = Vector3::<f64>::new(
            1.0 * current_step as f64,
            1.0 * current_step as f64,
            1.0 * current_step as f64,
        );
        let quaternion = UnitQuaternion::from_quaternion(Quaternion::new(1.0, 0.0, 0.0, 0.0));
        let isometry = Isometry3::from_parts(Translation3::from(point), quaternion);
        let m: Point = point.into();
        let pose_message: Pose = isometry.into();

        let mut marker_message = Marker::default();
        marker_message.header.frame_id = format!("world_offset");

        marker_message.header.stamp = timestamp.into();

        marker_message.ns = format!("basic_shapes");
        marker_message.id = (start_id + current_step) as i32;

        marker_message.type_ = MarkerObjectType::Sphere as i32;
        marker_message.action = MarkerActionType::Add as i32;

        marker_message.pose = pose_message;
        marker_message.scale = nalgebra::Vector3::<f64>::new(3.0, 3.0, 3.0).into();
        marker_message.color = std_msgs::ColorRGBA {
            r: 0.0,
            g: 0.0,
            b: 1.0,
            a: 1.0,
        };
        marker_message.lifetime = Duration::MAX;
        marker_message_array.markers.push(marker_message);
        //rosbag.append_message(&topic_name, &marker_message);
    }*/

    marker_message_array
}

fn get_marker_message(
    start_id: i64,
    linear_ring: &LinearRing,
    timestamp: DateTime<Utc>,
    color: std_msgs::ColorRGBA,
) -> Marker {
    let mut marker_message = Marker::default();
    let mut id = start_id;

    marker_message.header.frame_id = "world_offset".to_string();
    marker_message.header.stamp = timestamp.into();
    marker_message.ns = "basic_shapes".to_string();
    marker_message.id = id as i32;
    marker_message.type_ = MarkerObjectType::LineStrip as i32;
    marker_message.action = MarkerActionType::Add as i32;

    //marker_message.pose = pose_message;
    marker_message.scale = nalgebra::Vector3::<f64>::new(0.1, 0.1, 0.1).into();
    marker_message.color = color;
    marker_message.lifetime = Duration::MAX;

    for current_point in linear_ring.points().into_iter() {
        id += 1;

        let current_vertex: Point3<f64> = (*current_point).into();
        let current_vertex: Vector3<f64> = current_vertex.coords;
        let quaternion = UnitQuaternion::from_quaternion(Quaternion::new(1.0, 0.0, 0.0, 0.0));
        let isometry = Isometry3::from_parts(Translation3::from(current_vertex), quaternion);
        let _pose_message: Pose = isometry.into();

        let point_message: erosbag::ros_messages::geometry_msgs::Point = current_vertex.into();
        marker_message.points.push(point_message);
    }

    marker_message
}
