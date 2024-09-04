# S2JSON Spec Changelog

## 1.0.0

Initial release.

Rough estimation of the changes from GeoJSON:

* Properties data is clearly defined on how it can be shaped.
* 🧊 Support for 3D geometries.
* 🏷️ Support for metadata.
* ♏ Support for M-Values for each geometry point.
* 📦 Support for bounding boxes
* 🫥 Updated spec to handle vector structures.
* 🔨 Tools for converting between GeoJSON, S2JSON, and quad-tree Tile structures
* 🪩 Support for `S2Feature` and `S2FeatureCollection` types based upon the S2 Geometry *spherical projection*.
* ♻️ Feature Properties & M-Values are defined in scope to ensure they can be easily processed by lower level languages as structures, but also adds value to other projects down the line.
* 🛑 GeoJSON no longer supports `GeometryCollection`.
* 📝 Attribution can be added to either a `FeatureCollection` or `S2FeatureCollection`
