use crate::error::Error;
use egml::model::geometry::{DirectPosition, LinearRing};
use egml::operations::geometry::Geometry;
use egml::operations::surface::Surface;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlanesDocument {
    pub planes: Vec<PlanesElement>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlanesElement {
    pub id: String,
    pub parent_id: Option<String>,
    pub point: VectorElement,
    pub normal: VectorElement,
    pub vertices: Vec<VectorElement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct VectorElement {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl TryFrom<&LinearRing> for PlanesElement {
    type Error = Error;

    fn try_from(value: &LinearRing) -> Result<Self, Self::Error> {
        let plane = value.plane_equation();

        let plane = Self {
            id: value.gml.id.to_string(),
            parent_id: None,
            point: plane.point.into(),
            normal: plane.normal().into(),
            vertices: value.points().into_iter().map(|x| (*x).into()).collect(),
        };
        Ok(plane)
    }
}

impl From<Vector3<f64>> for VectorElement {
    fn from(value: Vector3<f64>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<DirectPosition> for VectorElement {
    fn from(value: DirectPosition) -> Self {
        Self {
            x: value.x(),
            y: value.y(),
            z: value.z(),
        }
    }
}
