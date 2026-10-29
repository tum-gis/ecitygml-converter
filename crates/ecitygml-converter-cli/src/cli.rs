use crate::error::Error;
use clap::{Args, Parser, Subcommand, ValueEnum, ValueHint};
use ecitygml::model::common::LevelOfDetail;
use ecitygml::model::core::CityModel;
use egraphics::io::MeshGrouping;
use nalgebra::{Point3, Vector3};
use std::path::PathBuf;

const CONVERT_TO_GRAPHICS_EXAMPLES: &str = "\
Examples:
  # Convert a dataset with the default settings
  ecitygml-converter-cli convert-to-graphics -i city.gml -o city.egraphics

  # Only LOD2 solids, one mesh per city object, without translation
  ecitygml-converter-cli convert-to-graphics -i city.gml.zst -o city.egraphics \\
      --lod 2 --geometry solid --mesh-grouping per-object --translation-mode none

  # Only city objects within a bounding box (in the coordinates of the input dataset)
  ecitygml-converter-cli convert-to-graphics -i city.gml -o city.egraphics \\
      --corner-min 690000 5335000 0 --corner-max 691000 5336000 1000";

/// Command-line tool for converting CityGML data to other representations.
#[derive(Parser)]
#[clap(author, version, about, propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Convert a CityGML dataset to a graphics representation (glTF in an egraphics archive)
    ///
    /// Reads a CityGML dataset, optionally filters its city objects by a bounding box, levels of
    /// detail and geometry types, translates the coordinates and triangulates the geometry. The
    /// result is written as an egraphics archive, a tar archive containing the glTF model and an
    /// info file with the georeference and the properties of the city objects.
    #[clap(after_long_help = CONVERT_TO_GRAPHICS_EXAMPLES)]
    ConvertToGraphics(ConvertToGraphicsArgs),
}

/// Options of the `convert-to-graphics` subcommand
#[derive(Args)]
pub struct ConvertToGraphicsArgs {
    /// Path to the CityGML dataset (.gml, .xml, .gml.gz or .gml.zst)
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub input_path: PathBuf,

    /// Path to the output file (.egraphics or .egraphics.tar)
    ///
    /// With .egraphics, the archive contains a compressed binary glTF (.glb) and a
    /// Zstandard-compressed info file. With .egraphics.tar, it contains an uncompressed,
    /// human-readable glTF (.gltf + .bin) and info file, which is useful for debugging.
    #[clap(short, long, value_hint = ValueHint::FilePath)]
    pub output_path: PathBuf,

    /// Minimum corner of the bounding box for filtering city objects
    ///
    /// Only city objects whose bounding shape (envelope) overlaps the box spanned by
    /// --corner-min and --corner-max are kept; city objects without a bounding shape are
    /// removed. The corners are given in the coordinates of the input dataset, i.e. before
    /// any translation. If only one corner is set, the box is unbounded on the other side.
    #[clap(
        long,
        value_names = ["X", "Y", "Z"],
        number_of_values = 3,
        allow_hyphen_values = true
    )]
    pub corner_min: Option<Vec<f64>>,

    /// Maximum corner of the bounding box for filtering city objects (see --corner-min)
    #[clap(
        long,
        value_names = ["X", "Y", "Z"],
        number_of_values = 3,
        allow_hyphen_values = true
    )]
    pub corner_max: Option<Vec<f64>>,

    /// Where to move the origin of the coordinates
    ///
    /// Large coordinates, such as those of projected coordinate reference systems, lose
    /// precision in the single-precision floats of glTF. Moving the origin into the dataset
    /// avoids this; the applied translation is stored in the georeference of the output. The
    /// position is computed from the bounding box of all city objects after filtering.
    #[clap(long, value_enum, default_value_t = TranslationMode::BottomCenter)]
    pub translation_mode: TranslationMode,

    /// Translation vector added to all coordinates; cannot be combined with --translation-mode
    ///
    /// For example, --custom-translation -690000 -5335000 0 moves the point
    /// (690000, 5335000, 0) to the origin.
    #[clap(
        long,
        value_names = ["X", "Y", "Z"],
        number_of_values = 3,
        allow_hyphen_values = true,
        conflicts_with = "translation_mode"
    )]
    pub custom_translation: Option<Vec<f64>>,

    /// Skip rebuilding the bounding shapes (envelopes) of city objects after reading
    ///
    /// Speeds up reading, but the bounding shapes stored in the dataset are then used as they
    /// are. These may be missing or inaccurate, which affects the bounding box filter
    /// (--corner-min, --corner-max) and the translation modes.
    #[clap(long)]
    pub skip_rebuilding_object_bounds: bool,

    /// Only include geometry of these levels of detail (0, 1, 2 or 3); all if omitted
    ///
    /// Can be repeated, e.g. --lod 1 --lod 2.
    #[clap(long, value_name = "LOD", value_parser = clap::value_parser!(u8).range(0..=3))]
    pub lod: Vec<u8>,

    /// Geometry types to include
    ///
    /// Can be repeated, e.g. --geometry surface --geometry solid.
    #[clap(long, value_name = "TYPE", value_enum, default_values_t = [GeometryType::Surface, GeometryType::Solid])]
    pub geometry: Vec<GeometryType>,

    /// How the geometry of the city objects is grouped into glTF meshes
    ///
    /// Geometry shared by multiple city objects (implicit geometries) is always written only
    /// once and referenced by each of them, regardless of this option.
    #[clap(long, value_enum, default_value_t = MeshGroupingArg::PerClass)]
    pub mesh_grouping: MeshGroupingArg,
}

impl ConvertToGraphicsArgs {
    /// Minimum corner of the bounding box filter
    pub fn corner_min(&self) -> Option<Point3<f64>> {
        self.corner_min.as_deref().map(Point3::from_slice)
    }

    /// Maximum corner of the bounding box filter
    pub fn corner_max(&self) -> Option<Point3<f64>> {
        self.corner_max.as_deref().map(Point3::from_slice)
    }

    /// Levels of detail to include; empty means all
    pub fn lod_filter(&self) -> Result<Vec<LevelOfDetail>, Error> {
        self.lod
            .iter()
            .map(|&l| LevelOfDetail::try_from(l).map_err(Error::from))
            .collect()
    }

    pub fn include_multi_surfaces(&self) -> bool {
        self.geometry.contains(&GeometryType::Surface)
    }

    pub fn include_solids(&self) -> bool {
        self.geometry.contains(&GeometryType::Solid)
    }

    pub fn mesh_grouping(&self) -> MeshGrouping {
        self.mesh_grouping.into()
    }

    /// Returns the vector added to all coordinates, or `None` if they are kept
    ///
    /// A custom translation takes precedence; otherwise the anchor point selected by the
    /// translation mode is computed from the bounding box of the city objects and moved to the
    /// origin.
    pub fn translation_vector(&self, city_model: &CityModel) -> Option<Vector3<f64>> {
        if let Some(custom_translation) = self.custom_translation.as_deref() {
            return Some(Vector3::from_column_slice(custom_translation));
        }
        if self.translation_mode == TranslationMode::None {
            return None;
        }

        let envelope = city_model.compute_city_object_members_envelope()?;
        let lower_corner = Vector3::from(envelope.lower_corner());
        let center: Vector3<f64> = envelope.center().into();
        let anchor = match self.translation_mode {
            TranslationMode::None => return None,
            TranslationMode::MinCorner => lower_corner,
            TranslationMode::Center => center,
            TranslationMode::BottomCenter => Vector3::new(center.x, center.y, lower_corner.z),
        };
        Some(-anchor)
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, ValueEnum)]
pub enum TranslationMode {
    /// Keep the original coordinates
    None,
    /// Move the minimum corner of the bounding box to the origin
    MinCorner,
    /// Move the center of the bounding box to the origin
    Center,
    /// Move the center of the bottom face of the bounding box to the origin
    BottomCenter,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, ValueEnum)]
pub enum GeometryType {
    /// Multi-surfaces, e.g. the LOD0 footprints or LOD2 roof and wall surfaces
    Surface,
    /// Solids, e.g. the LOD1 or LOD2 volumes of buildings
    Solid,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, ValueEnum)]
pub enum MeshGroupingArg {
    /// One mesh per city object class: fewer draw calls, but single objects cannot be separated
    PerClass,
    /// One node and mesh per city object: maps geometry to objects, but scales poorly
    PerObject,
}

impl From<MeshGroupingArg> for MeshGrouping {
    fn from(value: MeshGroupingArg) -> Self {
        match value {
            MeshGroupingArg::PerClass => MeshGrouping::PerClass,
            MeshGroupingArg::PerObject => MeshGrouping::PerObject,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_lod(lod: &str) -> Result<Vec<u8>, clap::Error> {
        let cli = Cli::try_parse_from([
            "ecitygml-converter-cli",
            "convert-to-graphics",
            "-i",
            "city.gml",
            "-o",
            "city.egraphics",
            "--lod",
            lod,
        ])?;
        let Commands::ConvertToGraphics(args) = cli.command;
        Ok(args.lod)
    }

    #[test]
    fn accepts_valid_lods() {
        for lod in 0..=3 {
            assert_eq!(parse_lod(&lod.to_string()).unwrap(), vec![lod]);
        }
    }

    #[test]
    fn rejects_invalid_lods() {
        for lod in ["4", "-1", "two"] {
            assert!(parse_lod(lod).is_err(), "--lod {lod} should be rejected");
        }
    }
}
