# ecitygml-converter-cli

CLI tool for converting [CityGML](https://www.ogc.org/standards/citygml/) data to other representations.

The library is at an early stage of development.

## Installation

Prebuilt binaries for Linux, macOS and Windows are attached to the [releases](https://github.com/tum-gis/ecitygml-converter/releases).
Alternatively, build it from source:

```bash
cargo install --path crates/ecitygml-converter-cli
```

## Usage

```bash
ecitygml-converter-cli convert-to-graphics -i city.gml -o city.egraphics
```

The `convert-to-graphics` command reads a CityGML dataset (`.gml`, `.xml`, `.gml.gz` or `.gml.zst`), triangulates its geometry and writes it as an egraphics archive: a tar archive containing a glTF model and an info file with the georeference and the properties of the city objects.
With the `.egraphics` extension, the archive contains a compressed binary glTF (`.glb`); with `.egraphics.tar`, an uncompressed, human-readable glTF (`.gltf` + `.bin`).

| Option                                                 | Description                                                                                     | Default             |
|--------------------------------------------------------|-------------------------------------------------------------------------------------------------|---------------------|
| `-i, --input-path <PATH>`                              | CityGML dataset to read                                                                         | required            |
| `-o, --output-path <PATH>`                             | Output file (`.egraphics` or `.egraphics.tar`)                                                  | required            |
| `--corner-min <X> <Y> <Z>`, `--corner-max <X> <Y> <Z>` | Only keep city objects overlapping this bounding box, in the coordinates of the input dataset   | no filter           |
| `--translation-mode <MODE>`                            | Where to move the origin: `none`, `min-corner`, `center` or `bottom-center` of the bounding box | `bottom-center`     |
| `--custom-translation <X> <Y> <Z>`                     | Translation vector added to all coordinates; cannot be combined with `--translation-mode`       |                     |
| `--lod <LOD>`                                          | Only include these levels of detail (0–3); repeatable                                           | all                 |
| `--geometry <TYPE>`                                    | Geometry types to include: `surface`, `solid`; repeatable                                       | both                |
| `--mesh-grouping <MODE>`                               | `per-class` (one mesh per class) or `per-object` (one mesh per city object)                     | `per-class`         |
| `--skip-rebuilding-object-bounds`                      | Use the bounding shapes stored in the dataset instead of recomputing them                       |                     |

Translating the coordinates keeps large values, e.g. of projected coordinate reference systems, from losing precision in glTF's single-precision floats; the applied translation is stored in the georeference of the output.

More examples:

```bash
# Only LOD2 solids, one mesh per city object, without translation
ecitygml-converter-cli convert-to-graphics -i city.gml.zst -o city.egraphics \
    --lod 2 --geometry solid --mesh-grouping per-object --translation-mode none

# Only city objects within a bounding box
ecitygml-converter-cli convert-to-graphics -i city.gml -o city.egraphics \
    --corner-min 690000 5335000 0 --corner-max 691000 5336000 1000
```

Run `ecitygml-converter-cli convert-to-graphics --help` for the full documentation of all options.

## Contributing

The library is developed at the [TUM Chair of Geoinformatics](https://github.com/tum-gis) and contributions are highly welcome.
