// #![no_std]
#![deny(missing_docs)]

//! The `s2json` Rust crate provides functionalities to read and write S2JSON Spec data structures.
//! This crate is a 0 dependency package that uses `no_std` and is intended to be used in
//! embedded systems and WASM applications.
//! NOTE: WM stands for WGS84 and S2 stands for S2Geometry

extern crate alloc;

/// All geometry types and structs
pub mod geometry;
/// Conjoined CellID System
pub mod id;
/// All S2 tooling
pub mod s2;
/// All utility tools
pub mod util;
/// All values types and structs
pub mod values;
/// All WM tooling
pub mod wm;

pub use geometry::*;
pub use id::*;
pub use s2::*;
pub use util::*;
pub use values::*;
pub use wm::*;

use serde::{Deserialize, Serialize};

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

//? S2 specific type

/// Cube-face on the S2 sphere
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Face {
    /// Face 0
    Face0 = 0,
    /// Face 1
    Face1 = 1,
    /// Face 2
    Face2 = 2,
    /// Face 3
    Face3 = 3,
    /// Face 4
    Face4 = 4,
    /// Face 5
    Face5 = 5,
}
impl From<Face> for u8 {
    fn from(face: Face) -> Self {
        face as u8
    }
}
impl From<u8> for Face {
    fn from(face: u8) -> Self {
        match face {
            1 => Face::Face1,
            2 => Face::Face2,
            3 => Face::Face3,
            4 => Face::Face4,
            5 => Face::Face5,
            _ => Face::Face0,
        }
    }
}

//? FeatureCollections

/// WM FeatureCollection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct FeatureCollection {
    /// Type will always be "FeatureCollection"
    #[serde(rename = "type")]
    pub _type: String,
    /// Collection of WM features
    pub features: Vec<WMFeature>,
    /// Attribution data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributions: Option<Attributions>,
    /// Bounding box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<BBox>,
}
impl FeatureCollection {
    /// Create a new FeatureCollection
    pub fn new(attributions: Option<Attributions>) -> Self {
        Self {
            _type: "FeatureCollection".to_string(),
            features: Vec::new(),
            attributions,
            bbox: None,
        }
    }

    /// update the bounding box
    pub fn update_bbox(&mut self, bbox: BBox) {
        let mut self_bbox = self.bbox.unwrap_or_default();
        self_bbox.merge(&bbox);
        self.bbox = Some(self_bbox);
    }
}

/// S2 FeatureCollection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct S2FeatureCollection {
    /// Type will always be "S2FeatureCollection"
    #[serde(rename = "type")]
    pub _type: String,
    /// Collection of S2 features
    pub features: Vec<S2Feature>,
    /// Track the faces that were used to generate the features
    pub faces: Vec<Face>,
    /// Attribution data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributions: Option<Attributions>,
    /// Bounding box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<BBox>,
}
impl S2FeatureCollection {
    /// Create a new S2FeatureCollection
    pub fn new(attributions: Option<Attributions>) -> Self {
        Self {
            _type: "S2FeatureCollection".to_string(),
            features: Vec::new(),
            faces: Vec::new(),
            attributions,
            bbox: None,
        }
    }

    /// update the bounding box
    pub fn update_bbox(&mut self, bbox: BBox) {
        let mut self_bbox = self.bbox.unwrap_or_default();
        self_bbox.merge(&bbox);
        self.bbox = Some(self_bbox);
    }

    /// Add a face, ensuring it is unique
    pub fn add_face(&mut self, face: Face) {
        if !self.faces.contains(&face) {
            self.faces.push(face);
        }
    }
}

//? Features

/// Component to build an WM Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Feature<M = ()> {
    /// Type will always be "Feature"
    #[serde(rename = "type")]
    pub _type: String,
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    /// Properties of the feature
    pub properties: Properties,
    /// Geometry of the feature
    pub geometry: Geometry,
    /// Metadata of the feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<M>,
}
impl<M> Feature<M> {
    /// Create a new Feature
    pub fn new(
        id: Option<u64>,
        properties: Properties,
        geometry: Geometry,
        metadata: Option<M>,
    ) -> Self {
        Self { _type: "Feature".to_string(), id, properties, geometry, metadata }
    }
}

/// Component to build an WM Vector Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VectorFeature<M = ()> {
    /// Type will always be "VectorFeature"
    #[serde(rename = "type")]
    pub _type: String,
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    /// Properties of the feature
    pub properties: Properties,
    /// Geometry of the feature
    pub geometry: VectorGeometry,
    /// Metadata of the feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<M>,
}
impl<M> VectorFeature<M> {
    /// Create a new VectorFeature
    pub fn new(
        id: Option<u64>,
        properties: Properties,
        geometry: VectorGeometry,
        metadata: Option<M>,
    ) -> Self {
        Self { _type: "VectorFeature".to_string(), id, properties, geometry, metadata }
    }
}

/// Component to build an S2 Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct S2Feature<M = ()> {
    /// Type will always be "S2Feature"
    #[serde(rename = "type")]
    pub _type: String,
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    /// Cube-Face of the feature
    pub face: Face,
    /// Properties of the feature
    pub properties: Properties,
    /// Geometry of the feature
    pub geometry: VectorGeometry,
    /// Metadata of the feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<M>,
}
impl<M> S2Feature<M> {
    /// Create a new S2Feature
    pub fn new(
        id: Option<u64>,
        face: Face,
        properties: Properties,
        geometry: VectorGeometry,
        metadata: Option<M>,
    ) -> Self {
        Self { _type: "S2Feature".to_string(), id, face, properties, geometry, metadata }
    }
}

//? Utility types

/// Attribution data is stored in an object.
/// The key is the name of the attribution, and the value is the href link
/// e.g. { "Open S2": "https://opens2.com/legal/data" }
pub type Attributions = BTreeMap<String, String>;

/// Either an S2 or WM FeatureCollection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FeatureCollections {
    /// An WM FeatureCollection
    FeatureCollection(FeatureCollection),
    /// An S2 FeatureCollection
    S2FeatureCollection(S2FeatureCollection),
}

/// Either an S2 or WM Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Features {
    /// An WM Feature
    Feature(Feature),
    /// An WM Vector Feature
    VectorFeature(VectorFeature),
    /// An S2 Feature
    S2Feature(S2Feature),
}

/// Either an WM Feature or an WM Vector Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum WMFeature {
    /// An WM Feature
    Feature(Feature),
    /// An WM Vector Feature
    VectorFeature(VectorFeature),
}

/// All major S2JSON types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum S2JSONCollection {
    /// An WM FeatureCollection
    FeatureCollection(FeatureCollection),
    /// An S2 FeatureCollection
    S2FeatureCollection(S2FeatureCollection),
    /// An WM Feature
    Feature(Feature),
    /// An WM Vector Feature
    VectorFeature(VectorFeature),
    /// An S2 Feature
    S2Feature(S2Feature),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn face() {
        let face = Face::Face0;
        assert_eq!(u8::from(face), 0);
        let face = Face::Face1;
        assert_eq!(u8::from(face), 1);
        let face = Face::Face2;
        assert_eq!(u8::from(face), 2);
        let face = Face::Face3;
        assert_eq!(u8::from(face), 3);
        let face = Face::Face4;
        assert_eq!(u8::from(face), 4);
        let face = Face::Face5;
        assert_eq!(u8::from(face), 5);

        assert_eq!(Face::Face0, Face::from(0));
        assert_eq!(Face::Face1, Face::from(1));
        assert_eq!(Face::Face2, Face::from(2));
        assert_eq!(Face::Face3, Face::from(3));
        assert_eq!(Face::Face4, Face::from(4));
        assert_eq!(Face::Face5, Face::from(5));
    }
}
