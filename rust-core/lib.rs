#![no_std]
#![forbid(unsafe_code)]
#![feature(coverage_attribute)]
#![deny(missing_docs)]

//! The `s2json` Rust crate provides functionalities to read and write S2JSON Spec data structures.
//! This crate is a 0 dependency package that uses `no_std` and is intended to be used in
//! embedded systems and WASM applications.
//! NOTE: WG stands for WGS84 and S2 stands for S2Geometry

extern crate alloc;

/// All geometry types and structs
pub mod geometry;
/// All json, value, shape impl
pub mod impls;
/// BTreeMap wrapper
pub mod map;
/// All shape types and structs
pub mod shape;
/// All values types and structs
pub mod value;

use alloc::{string::String, vec::Vec};
pub use geometry::*;
pub use impls::*;
pub use map::*;
use serde::{Deserialize, Serialize};
pub use shape::*;
pub use value::*;

/// All projections that can be used
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Projection {
    /// S2
    #[default]
    S2,
    /// WG
    WG,
}

//? S2 specific type

/// Cube-face on the S2 sphere
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Face {
    /// Face 0
    #[default]
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
impl serde::Serialize for Face {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

impl<'de> serde::Deserialize<'de> for Face {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        match value {
            0 => Ok(Face::Face0),
            1 => Ok(Face::Face1),
            2 => Ok(Face::Face2),
            3 => Ok(Face::Face3),
            4 => Ok(Face::Face4),
            5 => Ok(Face::Face5),
            _ => Err(serde::de::Error::custom("Invalid face value")),
        }
    }
}

/// FeatureCollection type string
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum FeatureCollectionType {
    /// WG FeatureCollection
    #[default]
    FeatureCollection,
}
impl From<&str> for FeatureCollectionType {
    fn from(_: &str) -> Self {
        FeatureCollectionType::FeatureCollection
    }
}

/// FeatureCollection type string
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum S2FeatureCollectionType {
    /// WG FeatureCollection
    #[default]
    S2FeatureCollection,
}
impl From<&str> for S2FeatureCollectionType {
    fn from(_: &str) -> Self {
        S2FeatureCollectionType::S2FeatureCollection
    }
}

//? FeatureCollections

/// WG FeatureCollection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct FeatureCollection<M = (), P: Clone + Default = Properties, D: Clone + Default = MValue> {
    /// Type will always be "FeatureCollection"
    #[serde(rename = "type")]
    pub _type: FeatureCollectionType,
    /// Collection of WG features
    pub features: Vec<Features<M, P, D>>,
    /// Attribution data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributions: Option<Attributions>,
    /// Bounding box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<BBox>,
}
impl<M, P: Clone + Default, D: Clone + Default> FeatureCollection<M, P, D> {
    /// Create a new FeatureCollection
    pub fn new(attributions: Option<Attributions>) -> Self {
        Self { _type: "FeatureCollection".into(), features: Vec::new(), attributions, bbox: None }
    }

    /// update the bounding box
    pub fn update_bbox(&mut self, bbox: BBox) {
        let mut self_bbox = self.bbox.unwrap_or_default();
        self_bbox = self_bbox.merge(&bbox);
        self.bbox = Some(self_bbox);
    }
}

/// S2 FeatureCollection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct S2FeatureCollection<M = (), P: Clone + Default = Properties, D: Clone + Default = MValue>
{
    /// Type will always be "S2FeatureCollection"
    #[serde(rename = "type")]
    pub _type: S2FeatureCollectionType,
    /// Collection of S2 features
    pub features: Vec<VectorFeature<M, P, D>>,
    /// Track the faces that were used to generate the features
    pub faces: Vec<Face>,
    /// Attribution data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributions: Option<Attributions>,
    /// Bounding box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<BBox>,
}
impl<M, P: Clone + Default, D: Clone + Default> S2FeatureCollection<M, P, D> {
    /// Create a new S2FeatureCollection
    pub fn new(attributions: Option<Attributions>) -> Self {
        Self {
            _type: "S2FeatureCollection".into(),
            features: Vec::new(),
            faces: Vec::new(),
            attributions,
            bbox: None,
        }
    }

    /// update the bounding box
    pub fn update_bbox(&mut self, bbox: BBox) {
        let mut self_bbox = self.bbox.unwrap_or_default();
        self_bbox = self_bbox.merge(&bbox);
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

/// Feature type string
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum FeatureType {
    /// WG Feature
    #[default]
    Feature,
}
impl From<&str> for FeatureType {
    fn from(_: &str) -> Self {
        FeatureType::Feature
    }
}

/// Component to build an WG Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Feature<M = (), P: Clone + Default = Properties, D: Clone + Default = MValue> {
    /// Type will always be "Feature"
    #[serde(rename = "type")]
    pub _type: FeatureType,
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    /// Properties of the feature
    pub properties: P,
    /// Geometry of the feature
    pub geometry: Geometry<D>,
    /// Metadata of the feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<M>,
}
impl<M, P: Clone + Default, D: Clone + Default> Feature<M, P, D> {
    /// Create a new Feature
    pub fn new(id: Option<u64>, properties: P, geometry: Geometry<D>, metadata: Option<M>) -> Self {
        Self { _type: "Feature".into(), id, properties, geometry, metadata }
    }
}
impl<M, P: Clone + Default, D: Clone + Default> Default for Feature<M, P, D> {
    fn default() -> Self {
        Self {
            _type: "Feature".into(),
            id: None,
            properties: Default::default(),
            geometry: Default::default(),
            metadata: None,
        }
    }
}

/// Feature type string
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum VectorFeatureType {
    /// WG Feature
    #[default]
    VectorFeature,
    /// S2 Feature
    S2Feature,
}
impl From<&str> for VectorFeatureType {
    fn from(s: &str) -> Self {
        match s {
            "S2Feature" => VectorFeatureType::S2Feature,
            _ => VectorFeatureType::VectorFeature,
        }
    }
}

/// Component to build an WG or S2 Vector Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VectorFeature<M = (), P: Clone + Default = Properties, D: Clone + Default = MValue> {
    /// Type will always be "VectorFeature"
    #[serde(rename = "type")]
    pub _type: VectorFeatureType,
    /// Unique identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    /// Face of the feature
    pub face: Face,
    /// Properties of the feature
    pub properties: P,
    /// Geometry of the feature
    pub geometry: VectorGeometry<D>,
    /// Metadata of the feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<M>,
}
impl<M, P: Clone + Default, D: Clone + Default> Default for VectorFeature<M, P, D> {
    fn default() -> Self {
        Self {
            _type: "VectorFeature".into(),
            face: 0.into(),
            id: None,
            properties: Default::default(),
            geometry: Default::default(),
            metadata: None,
        }
    }
}
impl<M, P: Clone + Default, D: Clone + Default> VectorFeature<M, P, D> {
    /// Create a new VectorFeature in the WG format
    pub fn new_wm(
        id: Option<u64>,
        properties: P,
        geometry: VectorGeometry<D>,
        metadata: Option<M>,
    ) -> Self {
        Self { _type: "VectorFeature".into(), face: 0.into(), id, properties, geometry, metadata }
    }

    /// Create a new VectorFeature in the WG format
    pub fn new_s2(
        id: Option<u64>,
        face: Face,
        properties: P,
        geometry: VectorGeometry<D>,
        metadata: Option<M>,
    ) -> Self {
        Self { _type: "S2Feature".into(), face, id, properties, geometry, metadata }
    }

    /// Create a new VectorFeature using an input VectorFeature. Assign new geometry if provided
    pub fn from_vector_feature(
        feature: &VectorFeature<M, P, D>,
        geometry: Option<VectorGeometry<D>>,
    ) -> Self
    where
        M: Clone,
    {
        Self {
            _type: feature._type.clone(),
            id: feature.id,
            face: feature.face,
            properties: feature.properties.clone(),
            geometry: geometry.unwrap_or(feature.geometry.clone()),
            metadata: feature.metadata.clone(),
        }
    }

    /// Create a VectorFeature that set's properties and geometry to m-values.
    /// Update the metadata to user defined value
    pub fn to_m_vector_feature<M2>(
        &self,
        to_meta: impl FnOnce(Option<&M>) -> Option<M2>,
    ) -> VectorFeature<M2, Properties, MValue>
    where
        M: Clone,
        P: MValueCompatible,
        D: MValueCompatible,
    {
        VectorFeature {
            _type: self._type.clone(),
            id: self.id,
            face: self.face,
            properties: self.properties.clone().into(),
            geometry: self.geometry.to_m_geometry(),
            metadata: to_meta(self.metadata.as_ref()),
        }
    }
}

//? Utility types

/// Attribution data is stored in an object.
/// The key is the name of the attribution, and the value is the href link
/// e.g. { "Open S2": "https://opens2.com/legal/data" }
pub type Attributions = Map<String, String>;

/// Either an S2 or WG FeatureCollection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum FeatureCollections<M = (), P: Clone + Default = Properties, D: Clone + Default = MValue> {
    /// An WG FeatureCollection
    FeatureCollection(FeatureCollection<M, P, D>),
    /// An S2 FeatureCollection
    S2FeatureCollection(S2FeatureCollection<M, P, D>),
}

/// Either an S2, Vector WG or WG Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Features<M = (), P: Clone + Default = Properties, D: Clone + Default = MValue> {
    /// An WG Feature
    Feature(Feature<M, P, D>),
    /// An WG or S2 Vector Feature
    VectorFeature(VectorFeature<M, P, D>),
}

/// All major S2JSON types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum JSONCollection<M = (), P: Clone + Default = Properties, D: Clone + Default = MValue> {
    /// An WG FeatureCollection
    FeatureCollection(FeatureCollection<M, P, D>),
    /// An S2 FeatureCollection
    S2FeatureCollection(S2FeatureCollection<M, P, D>),
    /// An WG Feature
    Feature(Feature<M, P, D>),
    /// An WG Vector Feature
    VectorFeature(VectorFeature<M, P, D>),
}
