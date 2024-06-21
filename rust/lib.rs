#![no_std]
#![deny(missing_docs)]

//! The `s2json` Rust crate provides functionalities to read and write S2JSON Spec data structures.
//! This crate is a 0 dependency package that uses `no_std` and is intended to be used in
//! embedded systems and WASM applications.
//! NOTE: WG stands for WGS84 and S2 stands for S2Geometry

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

/// All geometry types and structs
pub mod geometry;
/// All values types and structs
pub mod values;

pub use geometry::*;
pub use values::*;

//? S2 specific type

/// Cube-face on the S2 sphere
pub type Face = u8;
/// Cube face 0
pub const FACE_0: Face = 0;
/// Cube face 1
pub const FACE_1: Face = 1;
/// Cube face 2
pub const FACE_2: Face = 2;
/// Cube face 3
pub const FACE_3: Face = 3;
/// Cube face 4
pub const FACE_4: Face = 4;
/// Cube face 5
pub const FACE_5: Face = 5;

//? FeatureCollections

/// WG FeatureCollection
#[derive(Debug, PartialEq)]
pub struct FeatureCollection {
    /// Collection of WG features
    pub features: Vec<Feature>,
    /// Attribution data
    pub attributions: Option<Attributions>,
    /// Bounding box
    pub bbox: Option<BBox>,
}

/// S2 FeatureCollection
#[derive(Debug, PartialEq)]
pub struct S2FeatureCollection {
    /// Collection of S2 features
    pub features: Vec<S2Feature>,
    /// Attribution data
    pub attributions: Option<Attributions>,
    /// Bounding box
    pub bbox: Option<BBox>,
}

//? Features

/// Component to build either an S2 or WG Feature
#[derive(Debug, PartialEq)]
pub struct Feature {
    /// Unique identifier
    pub id: Option<u64>,
    /// Properties of the feature
    pub properties: Properties,
    /// Geometry of the feature
    pub geometry: Geometry,
}

/// Component to build either an S2 or WG Feature
#[derive(Debug, PartialEq)]
pub struct S2Feature {
    /// Unique identifier
    pub id: Option<u64>,
    /// Cube-Face of the feature
    pub face: Face,
    /// Properties of the feature
    pub properties: Properties,
    /// Geometry of the feature
    pub geometry: Geometry,
}

//? Utility types

/// Attribution data is stored in an object.
/// The key is the name of the attribution, and the value is the href link
/// e.g. { "Open S2": "https://opens2.com/legal/data" }
pub type Attributions = BTreeMap<String, String>;

/// Either an S2 or WG FeatureCollection
pub enum FeatureCollections {
    /// An WG FeatureCollection
    FeatureCollection(FeatureCollection),
    /// An S2 FeatureCollection
    S2FeatureCollection(S2FeatureCollection),
}

/// Either an S2 or WG Feature
pub enum Features {
    /// An WG Feature
    Feature(Feature),
    /// An S2 Feature
    S2Feature(S2Feature),
}

/// All major S2JSON types
pub enum JSONCollection {
    /// An WG FeatureCollection
    FeatureCollection(FeatureCollection),
    /// An S2 FeatureCollection
    S2FeatureCollection(S2FeatureCollection),
    /// An WG Feature
    Feature(Feature),
    /// An S2 Feature
    S2Feature(S2Feature),
}
