use ecitygml::model::core::CityModel;
use ecitygml::store::CityModelGraphicsStore;
use egraphics::Scene;

pub fn triangulate(city_model: CityModel) -> Scene {
    let _city_object_graphics_store = CityModelGraphicsStore::from_city_model(city_model);

    /*let all_triangulated_surfaces: Vec<TriangulatedSurface> = city_object_geometry_store
    .objects
    .values()
    .flat_map(triangulate_city_object_geometry)
    .collect();*/

    // all_triangulated_surfaces

    Scene::new()
}

/*fn triangulate_city_object_geometry(
    city_object_geometry_entry: AbstractCityObjectKindRef,
) -> Result<Vec<TriangulatedSurface>, Error> {
    let mut all_triangulated_surfaces: Vec<TriangulatedSurface> = Vec::new();

    let triangulated_surfaces: Vec<TriangulatedSurface> = city_object_geometry_entry
        .multi_surfaces
        .values()
        .filter_map(|x| {
            x.triangulate()
                .map_err(|e| {
                    warn!(
                        "error during triangulation of multi_surface ({:?}) with id: {}",
                        x.id(),
                        e
                    )
                })
                .ok()
        })
        .collect();
    all_triangulated_surfaces.extend(triangulated_surfaces);

    let triangulated_surfaces = city_object_geometry_entry
        .solids
        .values()
        .map(|x| {
            let exterior = x.exterior().ok_or(Error::SolidMissingExterior)?;
            let surface = exterior.object.as_ref().ok_or(Error::ShellMissingSurface)?;
            surface.triangulate().map_err(Error::from)
        })
        .collect::<Result<Vec<_>, _>>()?;
    all_triangulated_surfaces.extend(triangulated_surfaces);

    Ok(all_triangulated_surfaces)
}*/
