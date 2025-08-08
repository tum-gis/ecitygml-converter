use crate::error::Error;
use ecitygml::common::{CityObjectClass, LevelOfDetail};
use egml::model::geometry::{DirectPosition, LinearRing};
use egml::operations::geometry::Geometry;
use egml::operations::surface::Surface;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlanesDocument {
    pub planes: Vec<PlanesElement>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlanesElement {
    pub id: String,
    pub city_object_id: Option<String>,
    pub level_of_detail: Option<LevelOfDetailElement>,
    pub city_object_class: Option<CityObjectClassElement>,
    pub point: VectorElement,
    pub normal: VectorElement,
    pub vertices: Vec<VectorElement>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum LevelOfDetailElement {
    Zero,
    One,
    Two,
    Three,
}

impl From<LevelOfDetail> for LevelOfDetailElement {
    fn from(value: LevelOfDetail) -> Self {
        match value {
            LevelOfDetail::Zero => LevelOfDetailElement::Zero,
            LevelOfDetail::One => LevelOfDetailElement::One,
            LevelOfDetail::Two => LevelOfDetailElement::Two,
            LevelOfDetail::Three => LevelOfDetailElement::Three,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CityObjectClassElement {
    AuxiliaryTrafficArea,
    AuxiliaryTrafficSpace,
    Bridge,
    BridgeConstructiveElement,
    BridgeFurniture,
    BridgeInstallation,
    BridgePart,
    BridgeRoom,
    Building,
    BuildingConstructiveElement,
    BuildingFurniture,
    BuildingInstallation,
    BuildingPart,
    BuildingRoom,
    BuildingUnit,
    CeilingSurface,
    CityFurniture,
    CityObjectGroup,
    ClearanceSpace,
    Door,
    DoorSurface,
    FloorSurface,
    GenericLogicalSpace,
    GenericOccupiedSpace,
    GenericThematicSurface,
    GenericUnoccupiedSpace,
    GroundSurface,
    Hole,
    HoleSurface,
    HollowSpace,
    InteriorWallSurface,
    Intersection,
    Marking,
    OtherConstruction,
    OuterCeilingSurface,
    OuterFloorSurface,
    PlantCover,
    Railway,
    Road,
    RoofSurface,
    Section,
    SolitaryVegetationObject,
    Square,
    Story,
    Track,
    TrafficArea,
    TrafficSpace,
    Tunnel,
    TunnelConstructiveElement,
    TunnelFurniture,
    TunnelInstallation,
    TunnelPart,
    WallSurface,
    WaterBody,
    WaterGroundSurface,
    WaterSurface,
    Waterway,
    Window,
    WindowSurface,
}

impl From<CityObjectClass> for CityObjectClassElement {
    fn from(value: CityObjectClass) -> Self {
        match value {
            CityObjectClass::AuxiliaryTrafficArea => CityObjectClassElement::AuxiliaryTrafficArea,
            CityObjectClass::AuxiliaryTrafficSpace => CityObjectClassElement::AuxiliaryTrafficSpace,
            CityObjectClass::Bridge => CityObjectClassElement::Bridge,
            CityObjectClass::BridgeConstructiveElement => {
                CityObjectClassElement::BridgeConstructiveElement
            }
            CityObjectClass::BridgeFurniture => CityObjectClassElement::BridgeFurniture,
            CityObjectClass::BridgeInstallation => CityObjectClassElement::BridgeInstallation,
            CityObjectClass::BridgePart => CityObjectClassElement::BridgePart,
            CityObjectClass::BridgeRoom => CityObjectClassElement::BridgeRoom,
            CityObjectClass::Building => CityObjectClassElement::Building,
            CityObjectClass::BuildingConstructiveElement => {
                CityObjectClassElement::BuildingConstructiveElement
            }
            CityObjectClass::BuildingFurniture => CityObjectClassElement::BuildingFurniture,
            CityObjectClass::BuildingInstallation => CityObjectClassElement::BuildingInstallation,
            CityObjectClass::BuildingPart => CityObjectClassElement::BuildingPart,
            CityObjectClass::BuildingRoom => CityObjectClassElement::BuildingRoom,
            CityObjectClass::BuildingUnit => CityObjectClassElement::BuildingUnit,
            CityObjectClass::CeilingSurface => CityObjectClassElement::CeilingSurface,
            CityObjectClass::CityFurniture => CityObjectClassElement::CityFurniture,
            CityObjectClass::CityObjectGroup => CityObjectClassElement::CityObjectGroup,
            CityObjectClass::ClearanceSpace => CityObjectClassElement::ClearanceSpace,
            CityObjectClass::Door => CityObjectClassElement::Door,
            CityObjectClass::DoorSurface => CityObjectClassElement::DoorSurface,
            CityObjectClass::FloorSurface => CityObjectClassElement::FloorSurface,
            CityObjectClass::GenericLogicalSpace => CityObjectClassElement::GenericLogicalSpace,
            CityObjectClass::GenericOccupiedSpace => CityObjectClassElement::GenericOccupiedSpace,
            CityObjectClass::GenericThematicSurface => {
                CityObjectClassElement::GenericThematicSurface
            }
            CityObjectClass::GenericUnoccupiedSpace => {
                CityObjectClassElement::GenericUnoccupiedSpace
            }
            CityObjectClass::GroundSurface => CityObjectClassElement::GroundSurface,
            CityObjectClass::Hole => CityObjectClassElement::Hole,
            CityObjectClass::HoleSurface => CityObjectClassElement::HoleSurface,
            CityObjectClass::HollowSpace => CityObjectClassElement::HollowSpace,
            CityObjectClass::InteriorWallSurface => CityObjectClassElement::InteriorWallSurface,
            CityObjectClass::Intersection => CityObjectClassElement::Intersection,
            CityObjectClass::Marking => CityObjectClassElement::Marking,
            CityObjectClass::OtherConstruction => CityObjectClassElement::OtherConstruction,
            CityObjectClass::OuterCeilingSurface => CityObjectClassElement::OuterCeilingSurface,
            CityObjectClass::OuterFloorSurface => CityObjectClassElement::OuterFloorSurface,
            CityObjectClass::PlantCover => CityObjectClassElement::PlantCover,
            CityObjectClass::Railway => CityObjectClassElement::Railway,
            CityObjectClass::Road => CityObjectClassElement::Road,
            CityObjectClass::RoofSurface => CityObjectClassElement::RoofSurface,
            CityObjectClass::Section => CityObjectClassElement::Section,
            CityObjectClass::SolitaryVegetationObject => {
                CityObjectClassElement::SolitaryVegetationObject
            }
            CityObjectClass::Square => CityObjectClassElement::Square,
            CityObjectClass::Story => CityObjectClassElement::Story,
            CityObjectClass::Track => CityObjectClassElement::Track,
            CityObjectClass::TrafficArea => CityObjectClassElement::TrafficArea,
            CityObjectClass::TrafficSpace => CityObjectClassElement::TrafficSpace,
            CityObjectClass::Tunnel => CityObjectClassElement::Tunnel,
            CityObjectClass::TunnelConstructiveElement => {
                CityObjectClassElement::TunnelConstructiveElement
            }
            CityObjectClass::TunnelFurniture => CityObjectClassElement::TunnelFurniture,
            CityObjectClass::TunnelInstallation => CityObjectClassElement::TunnelInstallation,
            CityObjectClass::TunnelPart => CityObjectClassElement::TunnelPart,
            CityObjectClass::WallSurface => CityObjectClassElement::WallSurface,
            CityObjectClass::WaterBody => CityObjectClassElement::WaterBody,
            CityObjectClass::WaterGroundSurface => CityObjectClassElement::WaterGroundSurface,
            CityObjectClass::WaterSurface => CityObjectClassElement::WaterSurface,
            CityObjectClass::Waterway => CityObjectClassElement::Waterway,
            CityObjectClass::Window => CityObjectClassElement::Window,
            CityObjectClass::WindowSurface => CityObjectClassElement::WaterSurface,
        }
    }
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
            city_object_id: None,
            level_of_detail: None,
            city_object_class: None,
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
