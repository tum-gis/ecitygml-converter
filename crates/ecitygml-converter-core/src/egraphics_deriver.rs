use crate::error::Error;
use ecitygml::arena::CityModelArena;
use ecitygml::model::common::{HasFeatureType, LevelOfDetail};
use ecitygml::model::core::refs::{
    AbstractCityObjectKindRef, AbstractOccupiedSpaceKindRef, AbstractPhysicalSpaceKindRef,
    AbstractSpaceBoundaryKindRef, AbstractSpaceKindRef, AbstractThematicSurfaceKindRef,
};
use ecitygml::model::core::{
    AsAbstractCityObject, AsAbstractFeature, AsAbstractSpace, AsAbstractThematicSurface, CityModel,
    TransformationMatrix4x4,
};
use ecitygml::model::generics::GenericAttributeKind;
use ecitygml::model::relief::AsAbstractReliefComponent;
use ecitygml::model::relief::refs::AbstractReliefComponentKindRef;
use ecitygml::model::vegetation::refs::AbstractVegetationObjectKindRef;
use ecitygml::store::ImplicitGeometryStore;
use egml::model::base::{AsAbstractGml, Id};
use egml::model::common::{ApplyTransform, Triangulate, Triangulation};
use egml::model::geometry;
use egml::model::geometry::DirectPosition;
use egml::model::geometry::aggregates::MultiSurface;
use egml::model::geometry::primitives::{Solid, TriangulatedSurface};
use egraphics::{Georeference, MeshRef, PropertyValue, TriangleMesh};
use glam::{DVec3, Mat4, Vec3, Vec4};
use nalgebra::{Point3, Vector3};
use num_traits::cast;
use std::collections::HashMap;
use tracing::{debug, info, warn};

pub fn derive_scene_from_city_model(
    mut city_model: CityModel,
    corner_min: Option<Point3<f64>>,
    corner_max: Option<Point3<f64>>,
    translation_vector: Option<Vector3<f64>>,
    lod_filter: &[LevelOfDetail],
    include_multi_surfaces: bool,
    include_solids: bool,
) -> Result<egraphics::Scene, Error> {
    let mut implicit_geometry_store = ImplicitGeometryStore::from_city_model(&city_model);
    if corner_min.is_some() || corner_max.is_some() {
        let filter_envelope = geometry::Envelope::new(
            corner_min.map_or_else(|| Ok(DirectPosition::MIN), |c| c.try_into())?,
            corner_max.map_or_else(|| Ok(DirectPosition::MAX), |c| c.try_into())?,
        )?;
        city_model.filter_city_objects_by_envelope(&filter_envelope);
        implicit_geometry_store.filter_placements_by_envelope(&filter_envelope);
    }

    let georeference = get_georeference(&city_model, translation_vector);
    if let Some(v) = translation_vector {
        city_model.apply_translation(v);
        implicit_geometry_store.apply_translation(v);
    }
    let city_model_arena = CityModelArena::from_city_model(city_model)?;
    info!(
        "Number of city objects: {}",
        city_model_arena.iter_city_objects().count(),
    );

    let mut scene = egraphics::Scene::new();
    scene.set_georeference(georeference);

    for current_entry in city_model_arena.iter_city_objects() {
        let feature_type = current_entry.feature_type().to_string();
        let class_key = scene.get_or_insert_class_key(feature_type);

        let mut object = egraphics::Object::new(
            Vec::new(),
            class_key,
            current_entry.feature_id().to_string().into(),
            None,
        );
        object.properties = derive_properties(current_entry);

        scene.add_object(object);
    }

    implicit_geometry_store
        .prototypes_with_placements()
        .into_iter()
        .for_each(|(_id, prototype, placement)| {
            let triangulated_surface = prototype.triangulate().unwrap();
            let triangle_mesh =
                convert_triangulation_to_graphics_triangle_mesh(&triangulated_surface);
            let mesh_key = scene.add_mesh(triangle_mesh);

            placement
                .into_iter()
                .for_each(|(current_object_id, current_lod, current_placement)| {
                    let Some(object) =
                        scene.get_object_by_id_mut(&current_object_id.to_string().into())
                    else {
                        return;
                    };
                    let lod = convert_lod(current_lod);
                    let matrix = convert_matrix(current_placement.instance_transform());
                    let mesh_ref = MeshRef::new(mesh_key, lod).with_transform(matrix);
                    object.add_mesh_ref(mesh_ref)
                })
        });

    for current_entry in city_model_arena.iter_city_objects() {
        add_explicit_city_object_triangle_meshes(
            &current_entry,
            lod_filter,
            include_multi_surfaces,
            include_solids,
            &mut scene,
        )?;
    }

    scene.rebuild_object_aabbs();
    Ok(scene)
}

fn add_explicit_city_object_triangle_meshes(
    city_object_kind: &AbstractCityObjectKindRef,
    lod_filter: &[LevelOfDetail],
    include_multi_surfaces: bool,
    include_solids: bool,
    scene: &mut egraphics::Scene,
) -> Result<(), Error> {
    let lod_allowed = |lod: &LevelOfDetail| lod_filter.is_empty() || lod_filter.contains(lod);
    let feature_id = city_object_kind.feature_id();

    let mut mesh_refs: Vec<MeshRef> = Vec::new();

    match city_object_kind {
        AbstractCityObjectKindRef::AbstractSpaceKind(abstract_space) => match abstract_space {
            AbstractSpaceKindRef::AbstractLogicalSpaceKind(_) => {}
            AbstractSpaceKindRef::AbstractPhysicalSpaceKind(x) => {
                if include_solids {
                    x.solids_by_lod()
                        .into_iter()
                        .filter(|(l, _)| lod_allowed(l))
                        .for_each(|(l, property)| {
                            let mesh_result = property
                                .object()
                                .ok_or(Error::UnresolvedGeometryReference)
                                .and_then(convert_solid);
                            register_mesh_if_ok(scene, &mut mesh_refs, feature_id, l, mesh_result);
                        });
                }

                if include_multi_surfaces {
                    x.multi_surfaces_by_lod()
                        .into_iter()
                        .filter(|(l, _)| lod_allowed(l))
                        .for_each(|(l, property)| {
                            let mesh_result = property
                                .object()
                                .ok_or(Error::UnresolvedGeometryReference)
                                .and_then(convert_multi_surface);
                            register_mesh_if_ok(scene, &mut mesh_refs, feature_id, l, mesh_result);
                        });
                }
            }
        },
        AbstractCityObjectKindRef::AbstractSpaceBoundaryKind(abstract_space_boundary) => {
            match abstract_space_boundary {
                AbstractSpaceBoundaryKindRef::AbstractThematicSurfaceKind(x) => {
                    if include_multi_surfaces {
                        x.multi_surfaces_by_lod()
                            .into_iter()
                            .filter(|(l, _)| lod_allowed(l))
                            .for_each(|(l, property)| {
                                let mesh_result = property
                                    .object()
                                    .ok_or(Error::UnresolvedGeometryReference)
                                    .and_then(convert_multi_surface);
                                register_mesh_if_ok(
                                    scene,
                                    &mut mesh_refs,
                                    feature_id,
                                    l,
                                    mesh_result,
                                );
                            });
                    }
                }
                AbstractSpaceBoundaryKindRef::ReliefFeature(_) => {}
                AbstractSpaceBoundaryKindRef::AbstractReliefComponentKind(
                    abstract_relief_component,
                ) => match abstract_relief_component {
                    AbstractReliefComponentKindRef::TinRelief(tin_relief) => {
                        if include_multi_surfaces {
                            let lod = tin_relief.lod();
                            let mesh_result = tin_relief
                                .tin()
                                .and_then(|x| x.object())
                                .ok_or(Error::UnresolvedGeometryReference)
                                .map(convert_to_graphics_triangle_mesh);
                            register_mesh_if_ok(
                                scene,
                                &mut mesh_refs,
                                feature_id,
                                lod,
                                mesh_result,
                            );
                        }
                    }
                },
            }
        }
    }

    // Every city object was already registered into `scene` in the loop at
    // the top of `derive_scene_from_city_model`, before this function runs —
    // a missing entry here means an internal invariant was violated, not
    // malformed input.
    let graphics_object = scene
        .get_object_by_id_mut(&feature_id.to_string().into())
        .expect("city object must already be registered in scene");
    graphics_object.extend_mesh_refs(mesh_refs);

    Ok(())
}

fn register_mesh_if_ok(
    scene: &mut egraphics::Scene,
    mesh_refs: &mut Vec<MeshRef>,
    feature_id: &Id,
    lod: LevelOfDetail,
    mesh_result: Result<TriangleMesh, Error>,
) {
    match mesh_result {
        Ok(mesh) => {
            let mesh_key = scene.add_mesh(mesh);
            mesh_refs.push(MeshRef::new(mesh_key, convert_lod(lod)));
        }
        Err(error) => {
            warn!("skipping geometry for city object {feature_id} at LoD {lod:?}: {error}");
        }
    }
}

fn convert_lod(lod: ecitygml::model::common::LevelOfDetail) -> egraphics::LevelOfDetail {
    match lod {
        LevelOfDetail::Zero => egraphics::LevelOfDetail::Zero,
        LevelOfDetail::One => egraphics::LevelOfDetail::One,
        LevelOfDetail::Two => egraphics::LevelOfDetail::Two,
        LevelOfDetail::Three => egraphics::LevelOfDetail::Three,
    }
}

fn convert_matrix(transformation_matrix: TransformationMatrix4x4) -> Mat4 {
    let m = transformation_matrix.matrix();

    // CityGML's Z-up axes are remapped to the engine's Y-up scene space via
    // (x, y, z) -> (x, z, -y), matching the swap in
    // `convert_to_graphics_triangle_mesh`. Conjugating the matrix by that
    // basis change keeps it consistent with the already-remapped prototype
    // mesh vertices it will be applied to.
    let swap_column = |col: usize| {
        Vec4::new(
            m[(0, col)] as f32,
            m[(2, col)] as f32,
            -m[(1, col)] as f32,
            m[(3, col)] as f32,
        )
    };

    Mat4::from_cols(
        swap_column(0),
        swap_column(2),
        -swap_column(1),
        swap_column(3),
    )
}

fn convert_multi_surface(multi_surface: &MultiSurface) -> Result<TriangleMesh, Error> {
    let geometry_id = multi_surface.id().map(|x| x.to_string());

    let triangulation = multi_surface.triangulate()?;
    let mut triangle_mesh = convert_triangulation_to_graphics_triangle_mesh(&triangulation);

    triangle_mesh.geometry_id = geometry_id;
    Ok(triangle_mesh)
}

fn convert_solid(solid: &Solid) -> Result<TriangleMesh, Error> {
    let geometry_id = solid.id().map(|x| x.to_string());

    let triangulation = solid
        .exterior()
        .ok_or(Error::SolidMissingExterior)?
        .triangulate()?;
    let mut triangle_mesh = convert_triangulation_to_graphics_triangle_mesh(&triangulation);

    triangle_mesh.geometry_id = geometry_id;
    Ok(triangle_mesh)
}

fn convert_triangulation_to_graphics_triangle_mesh(triangulation: &Triangulation) -> TriangleMesh {
    for current_error in triangulation.skipped() {
        debug!("skipped geometry: {}", current_error);
    }
    convert_to_graphics_triangle_mesh(triangulation.surface())
}

fn convert_to_graphics_triangle_mesh(triangulated_surface: &TriangulatedSurface) -> TriangleMesh {
    let triangles = triangulated_surface.triangles();

    let mut vertices: Vec<Vec3> = Vec::with_capacity(triangles.len() * 3);
    let mut indices: Vec<[u32; 3]> = Vec::with_capacity(triangles.len());

    for triangle in triangles {
        let base = vertices.len() as u32;
        for p in triangle.points() {
            let x: f32 = cast(p.x()).expect("coordinate overflow");
            let y: f32 = cast(p.z()).expect("coordinate overflow");
            let z: f32 = cast(-p.y()).expect("coordinate overflow");
            vertices.push(Vec3::new(x, y, z));
        }
        indices.push([base, base + 1, base + 2]);
    }

    let triangle_mesh = TriangleMesh::new(vertices, indices);
    triangle_mesh.deduplicate_vertices();
    triangle_mesh
}

fn derive_properties(
    abstract_city_object_kind: AbstractCityObjectKindRef,
) -> HashMap<String, PropertyValue> {
    let mut properties: HashMap<String, PropertyValue> = HashMap::new();

    match abstract_city_object_kind {
        AbstractCityObjectKindRef::AbstractSpaceKind(abstract_space_kind) => {
            match abstract_space_kind {
                AbstractSpaceKindRef::AbstractLogicalSpaceKind(_) => {}
                AbstractSpaceKindRef::AbstractPhysicalSpaceKind(abstract_physical_space_kind) => {
                    match abstract_physical_space_kind {
                        AbstractPhysicalSpaceKindRef::AbstractOccupiedSpaceKind(
                            abstract_occupied_space,
                        ) => match abstract_occupied_space {
                            AbstractOccupiedSpaceKindRef::CityFurniture(_) => {}
                            AbstractOccupiedSpaceKindRef::AbstractConstructionKind(_) => {}
                            AbstractOccupiedSpaceKindRef::AbstractConstructiveElementKind(_) => {}
                            AbstractOccupiedSpaceKindRef::AbstractFillingElementKind(_) => {}
                            AbstractOccupiedSpaceKindRef::GenericOccupiedSpace(_) => {}
                            AbstractOccupiedSpaceKindRef::AbstractInstallationKind(_) => {}
                            AbstractOccupiedSpaceKindRef::AbstractVegetationObjectKind(
                                abstract_vegetation_object,
                            ) => match abstract_vegetation_object {
                                AbstractVegetationObjectKindRef::PlantCover(_x) => {}
                                AbstractVegetationObjectKindRef::SolitaryVegetationObject(x) => {
                                    if let Some(class) = x.class() {
                                        properties.insert(
                                            "class".to_string(),
                                            PropertyValue::String(class.value().to_string()),
                                        );
                                    }
                                    x.functions().iter().for_each(|x| {
                                        properties.insert(
                                            "function".to_string(),
                                            PropertyValue::String(x.value().to_string()),
                                        );
                                    });
                                    x.usages().iter().for_each(|x| {
                                        properties.insert(
                                            "usage".to_string(),
                                            PropertyValue::String(x.value().to_string()),
                                        );
                                    });

                                    if let Some(height) = x.height() {
                                        properties.insert(
                                            "height".to_string(),
                                            PropertyValue::Float(height.value),
                                        );
                                    }
                                }
                            },
                            AbstractOccupiedSpaceKindRef::WaterBody(_) => {}
                            AbstractOccupiedSpaceKindRef::AbstractFurnitureKind(_) => {}
                        },
                        AbstractPhysicalSpaceKindRef::AbstractUnoccupiedSpaceKind(_) => {}
                    }
                }
            }
        }
        AbstractCityObjectKindRef::AbstractSpaceBoundaryKind(abstract_space_boundary) => {
            match abstract_space_boundary {
                AbstractSpaceBoundaryKindRef::AbstractThematicSurfaceKind(
                    abstract_thematic_surface,
                ) => match abstract_thematic_surface {
                    AbstractThematicSurfaceKindRef::AuxiliaryTrafficArea(_) => {}
                    AbstractThematicSurfaceKindRef::ClosureSurface(_) => {}
                    AbstractThematicSurfaceKindRef::AbstractConstructionSurfaceKind(_) => {}
                    AbstractThematicSurfaceKindRef::AbstractFillingSurfaceKind(_) => {}
                    AbstractThematicSurfaceKindRef::GenericThematicSurface(_) => {}
                    AbstractThematicSurfaceKindRef::HoleSurface(_) => {}
                    AbstractThematicSurfaceKindRef::LandUse(x) => {
                        if let Some(class) = x.class() {
                            properties.insert(
                                "class".to_string(),
                                PropertyValue::String(class.value().to_string()),
                            );
                        }
                        x.functions().iter().for_each(|x| {
                            properties.insert(
                                "function".to_string(),
                                PropertyValue::String(x.value().to_string()),
                            );
                        });
                        x.usages().iter().for_each(|x| {
                            properties.insert(
                                "usage".to_string(),
                                PropertyValue::String(x.value().to_string()),
                            );
                        })
                    }
                    AbstractThematicSurfaceKindRef::Marking(_) => {}
                    AbstractThematicSurfaceKindRef::TrafficArea(_) => {}
                    AbstractThematicSurfaceKindRef::AbstractWaterBoundarySurfaceKind(_) => {}
                },
                AbstractSpaceBoundaryKindRef::ReliefFeature(_) => {}
                AbstractSpaceBoundaryKindRef::AbstractReliefComponentKind(_) => {}
            }
        }
    }

    properties.extend(
        abstract_city_object_kind
            .generic_attributes()
            .iter()
            .flat_map(|x| convert_attribute(x.clone())),
    );

    properties
}

fn convert_attribute(generic_attribute: GenericAttributeKind) -> Option<(String, PropertyValue)> {
    match generic_attribute {
        GenericAttributeKind::StringAttribute(x) => Some((x.name, PropertyValue::String(x.value))),
        GenericAttributeKind::IntAttribute(x) => Some((x.name, PropertyValue::Int(x.value))),
        GenericAttributeKind::DoubleAttribute(x) => Some((x.name, PropertyValue::Float(x.value))),
        GenericAttributeKind::MeasureAttribute(_x) => None,
        GenericAttributeKind::GenericAttributeSet(_x) => None,
        GenericAttributeKind::DateAttribute(x) => {
            Some((x.name, PropertyValue::String(x.value.date().to_string())))
        }
        GenericAttributeKind::CodeAttribute(x) => {
            Some((x.name, PropertyValue::String(x.value.value().to_string())))
        }
        GenericAttributeKind::UriAttribute(x) => {
            Some((x.name, PropertyValue::String(x.value.to_string())))
        }
    }
}

fn get_georeference(
    city_model: &CityModel,
    translation_vector: Option<Vector3<f64>>,
) -> Option<Georeference> {
    let city_model_srs_name: Option<String> =
        egml::model::feature::AsAbstractFeature::bounded_by(city_model)
            .and_then(|x| x.envelope())
            .and_then(|x| x.srs_name())
            .map(|x| x.to_string());

    let mut georeferencing = Georeference::new();
    georeferencing.crs = city_model_srs_name;
    georeferencing.offset = translation_vector.map(|v| DVec3::new(-v.x, -v.y, -v.z));

    Some(georeferencing)
}
