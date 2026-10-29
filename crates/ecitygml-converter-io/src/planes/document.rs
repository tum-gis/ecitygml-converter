use crate::error::Error;
use ecitygml::model::common::{FeatureType, LevelOfDetail};
use egml::model::base::AsAbstractGml;
use egml::model::geometry::DirectPosition;
use egml::model::geometry::primitives::Polygon;
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
    pub city_object_class: Option<FeatureTypeElement>,
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
pub enum FeatureTypeElement {
    Appearance,
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
    CityModel,
    CityObjectGroup,
    ClearanceSpace,
    ClosureSurface,
    Door,
    DoorSurface,
    FloorSurface,
    GenericLogicalSpace,
    GenericOccupiedSpace,
    GenericThematicSurface,
    GenericUnoccupiedSpace,
    GeoreferencedTexture,
    GroundSurface,
    Hole,
    HoleSurface,
    HollowSpace,
    InteriorWallSurface,
    Intersection,
    LandUse,
    Marking,
    OtherConstruction,
    OuterCeilingSurface,
    OuterFloorSurface,
    ParameterizedTexture,
    PlantCover,
    PointCloud,
    Railway,
    ReliefFeature,
    Road,
    RoofSurface,
    Section,
    SolitaryVegetationObject,
    Square,
    Storey,
    TinRelief,
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
    X3DMaterial,
}

impl TryFrom<FeatureType> for FeatureTypeElement {
    type Error = Error;

    fn try_from(value: FeatureType) -> Result<Self, Self::Error> {
        match value {
            FeatureType::AuxiliaryTrafficArea => Ok(FeatureTypeElement::AuxiliaryTrafficArea),
            FeatureType::AuxiliaryTrafficSpace => Ok(FeatureTypeElement::AuxiliaryTrafficSpace),
            FeatureType::Bridge => Ok(FeatureTypeElement::Bridge),
            FeatureType::BridgeConstructiveElement => {
                Ok(FeatureTypeElement::BridgeConstructiveElement)
            }
            FeatureType::BridgeFurniture => Ok(FeatureTypeElement::BridgeFurniture),
            FeatureType::BridgeInstallation => Ok(FeatureTypeElement::BridgeInstallation),
            FeatureType::BridgePart => Ok(FeatureTypeElement::BridgePart),
            FeatureType::BridgeRoom => Ok(FeatureTypeElement::BridgeRoom),
            FeatureType::Building => Ok(FeatureTypeElement::Building),
            FeatureType::BuildingConstructiveElement => {
                Ok(FeatureTypeElement::BuildingConstructiveElement)
            }
            FeatureType::BuildingFurniture => Ok(FeatureTypeElement::BuildingFurniture),
            FeatureType::BuildingInstallation => Ok(FeatureTypeElement::BuildingInstallation),
            FeatureType::BuildingPart => Ok(FeatureTypeElement::BuildingPart),
            FeatureType::BuildingRoom => Ok(FeatureTypeElement::BuildingRoom),
            FeatureType::BuildingUnit => Ok(FeatureTypeElement::BuildingUnit),
            FeatureType::CeilingSurface => Ok(FeatureTypeElement::CeilingSurface),
            FeatureType::CityFurniture => Ok(FeatureTypeElement::CityFurniture),
            FeatureType::CityObjectGroup => Ok(FeatureTypeElement::CityObjectGroup),
            FeatureType::ClearanceSpace => Ok(FeatureTypeElement::ClearanceSpace),
            FeatureType::Door => Ok(FeatureTypeElement::Door),
            FeatureType::DoorSurface => Ok(FeatureTypeElement::DoorSurface),
            FeatureType::FloorSurface => Ok(FeatureTypeElement::FloorSurface),
            FeatureType::GenericLogicalSpace => Ok(FeatureTypeElement::GenericLogicalSpace),
            FeatureType::GenericOccupiedSpace => Ok(FeatureTypeElement::GenericOccupiedSpace),
            FeatureType::GenericThematicSurface => Ok(FeatureTypeElement::GenericThematicSurface),
            FeatureType::GenericUnoccupiedSpace => Ok(FeatureTypeElement::GenericUnoccupiedSpace),
            FeatureType::GroundSurface => Ok(FeatureTypeElement::GroundSurface),
            FeatureType::Hole => Ok(FeatureTypeElement::Hole),
            FeatureType::HoleSurface => Ok(FeatureTypeElement::HoleSurface),
            FeatureType::HollowSpace => Ok(FeatureTypeElement::HollowSpace),
            FeatureType::InteriorWallSurface => Ok(FeatureTypeElement::InteriorWallSurface),
            FeatureType::Intersection => Ok(FeatureTypeElement::Intersection),
            FeatureType::LandUse => Ok(FeatureTypeElement::LandUse),
            FeatureType::Marking => Ok(FeatureTypeElement::Marking),
            FeatureType::OtherConstruction => Ok(FeatureTypeElement::OtherConstruction),
            FeatureType::OuterCeilingSurface => Ok(FeatureTypeElement::OuterCeilingSurface),
            FeatureType::OuterFloorSurface => Ok(FeatureTypeElement::OuterFloorSurface),
            FeatureType::PlantCover => Ok(FeatureTypeElement::PlantCover),
            FeatureType::Railway => Ok(FeatureTypeElement::Railway),
            FeatureType::ReliefFeature => Ok(FeatureTypeElement::ReliefFeature),
            FeatureType::Road => Ok(FeatureTypeElement::Road),
            FeatureType::RoofSurface => Ok(FeatureTypeElement::RoofSurface),
            FeatureType::Section => Ok(FeatureTypeElement::Section),
            FeatureType::SolitaryVegetationObject => {
                Ok(FeatureTypeElement::SolitaryVegetationObject)
            }
            FeatureType::Square => Ok(FeatureTypeElement::Square),
            FeatureType::Storey => Ok(FeatureTypeElement::Storey),
            FeatureType::TinRelief => Ok(FeatureTypeElement::TinRelief),
            FeatureType::Track => Ok(FeatureTypeElement::Track),
            FeatureType::TrafficArea => Ok(FeatureTypeElement::TrafficArea),
            FeatureType::TrafficSpace => Ok(FeatureTypeElement::TrafficSpace),
            FeatureType::Tunnel => Ok(FeatureTypeElement::Tunnel),
            FeatureType::TunnelConstructiveElement => {
                Ok(FeatureTypeElement::TunnelConstructiveElement)
            }
            FeatureType::TunnelFurniture => Ok(FeatureTypeElement::TunnelFurniture),
            FeatureType::TunnelInstallation => Ok(FeatureTypeElement::TunnelInstallation),
            FeatureType::TunnelPart => Ok(FeatureTypeElement::TunnelPart),
            FeatureType::WallSurface => Ok(FeatureTypeElement::WallSurface),
            FeatureType::WaterBody => Ok(FeatureTypeElement::WaterBody),
            FeatureType::WaterGroundSurface => Ok(FeatureTypeElement::WaterGroundSurface),
            FeatureType::WaterSurface => Ok(FeatureTypeElement::WaterSurface),
            FeatureType::Waterway => Ok(FeatureTypeElement::Waterway),
            FeatureType::Window => Ok(FeatureTypeElement::Window),
            FeatureType::WindowSurface => Ok(FeatureTypeElement::WindowSurface),
            FeatureType::ClosureSurface => Ok(FeatureTypeElement::ClosureSurface),
            FeatureType::Appearance => Ok(FeatureTypeElement::Appearance),

            FeatureType::GeoreferencedTexture => Ok(FeatureTypeElement::GeoreferencedTexture),
            FeatureType::CityModel => Ok(FeatureTypeElement::CityModel),

            FeatureType::ParameterizedTexture => Ok(FeatureTypeElement::ParameterizedTexture),
            FeatureType::X3DMaterial => Ok(FeatureTypeElement::X3DMaterial),
            FeatureType::PointCloud => Ok(FeatureTypeElement::PointCloud),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct VectorElement {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl TryFrom<&Polygon> for PlanesElement {
    type Error = Error;

    fn try_from(value: &Polygon) -> Result<Self, Self::Error> {
        let plane = value.plane_equation();

        let plane = Self {
            id: value.id().map(|x| x.to_string()).unwrap_or_default(),
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
