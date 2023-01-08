use ecitygml::CitygmlModel;
use egml::geometry::LinearRing;
use erosbag::ros_messages::{std_msgs, RosMessageType};
use erosbag::{Rosbag, RosbagOpenOptions};
use tracing::info;

use chrono::{DateTime, TimeZone, Utc};

use erosbag::ros_messages::builtin_msgs::Duration;
use erosbag::ros_messages::geometry_msgs::Pose;
use erosbag::ros_messages::visualization_msgs::{
    Marker, MarkerActionType, MarkerArray, MarkerObjectType,
};
use erosbag::topics::qos_profile::QualityOfServiceProfile;
use erosbag::topics::topic::{TopicMetadata, TopicSerializationFormat};
use nalgebra::{Isometry3, Point3, Quaternion, Translation3, UnitQuaternion, Vector3};
use std::path::PathBuf;

pub fn citymodel_to_rosbag(city_model: CitygmlModel, mut rosbag: Rosbag) {
    info!("Number of objects: {}", city_model.number_of_objects());

    let topic_name: String = "/marker_array".to_string();
    rosbag.create_topic(
        &topic_name,
        &TopicMetadata::new(
            RosMessageType::VisualizationMessagesMarkerArray,
            TopicSerializationFormat::CDR,
            vec![QualityOfServiceProfile::new_for_static_tf_topic()],
        ),
    );

    for current_step in 0..2 {
        let timestamp =
            Utc.timestamp_opt(1579007185, 0).unwrap() + chrono::Duration::seconds(current_step);
        let marker_message_array =
            create_marker_array(&city_model, current_step * 10000, timestamp);
        rosbag.append_message(&topic_name, &marker_message_array);
    }

    rosbag.close();
}

fn create_marker_array(
    city_model: &CitygmlModel,
    _start_id: i64,
    timestamp: DateTime<Utc>,
) -> MarkerArray {
    let mut marker_message_array = MarkerArray::default();
    let mut vert_id = 0;

    for current_member in city_model
        .wall_surface()
        .iter()
        .filter(|x| x.lod2_multi_surface().is_some())
    {
        for current_geometry in current_member
            .lod2_multi_surface()
            .as_ref()
            .unwrap()
            .members()
            .iter()
        {
            let marker_message = get_marker_message(vert_id, current_geometry, timestamp);
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

fn get_marker_message(start_id: i64, linear_ring: &LinearRing, timestamp: DateTime<Utc>) -> Marker {
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
    marker_message.color = std_msgs::ColorRGBA {
        r: 0.0,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    marker_message.lifetime = Duration::MAX;

    for current_point in linear_ring.points().iter() {
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

fn read_bag(_rosbag_directory_path: PathBuf) {
    let rosbag_directory_path = PathBuf::from("~/markers_test_recorded/");
    let rosbag = RosbagOpenOptions::new()
        .read_write(true)
        .open(&rosbag_directory_path)
        .unwrap();

    let ros = rosbag.get_all_topic_names();
    rosbag.get_visualization_markers(&"/marker".to_string());
    info!("test {:?}", ros);
}
