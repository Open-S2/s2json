#![no_std]
#![deny(missing_docs)]

//! The `s2json` Rust crate provides functionalities to read and write S2JSON Spec data structures.
//! This crate is a 0 dependency package that uses `no_std` and is intended to be used in
//! embedded systems and WASM applications.
//! NOTE: WM stands for WGS84 and S2 stands for S2Geometry

extern crate alloc;

/// All geometry types and structs
pub mod geometry;
/// BTreeMap wrapper
pub mod map;
/// All values types and structs
pub mod value;
/// All values impl
pub mod value_impl;
/// The VectorPoint struct is a powerful tool for 2D and 3D points
pub mod vector_point;

pub use geometry::*;
pub use map::*;
pub use value::*;
pub use vector_point::*;

use serde::{Deserialize, Serialize};

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

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
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
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

//? FeatureCollections

/// WM FeatureCollection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct FeatureCollection<M = (), P: MValueCompatible = Properties, D: MValueCompatible = MValue>
{
    /// Type will always be "FeatureCollection"
    #[serde(rename = "type")]
    pub _type: String,
    /// Collection of WM features
    pub features: Vec<WMFeature<M, P, D>>,
    /// Attribution data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributions: Option<Attributions>,
    /// Bounding box
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<BBox>,
}
impl<M, P: MValueCompatible, D: MValueCompatible> FeatureCollection<M, P, D> {
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
        self_bbox = self_bbox.merge(&bbox);
        self.bbox = Some(self_bbox);
    }
}

/// S2 FeatureCollection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct S2FeatureCollection<
    M = (),
    P: MValueCompatible = Properties,
    D: MValueCompatible = MValue,
> {
    /// Type will always be "S2FeatureCollection"
    #[serde(rename = "type")]
    pub _type: String,
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
impl<M, P: MValueCompatible, D: MValueCompatible> S2FeatureCollection<M, P, D> {
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

/// Component to build an WM Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Feature<M = (), P: MValueCompatible = Properties, D: MValueCompatible = MValue> {
    /// Type will always be "Feature"
    #[serde(rename = "type")]
    pub _type: String,
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
impl<M, P: MValueCompatible, D: MValueCompatible> Feature<M, P, D> {
    /// Create a new Feature
    pub fn new(id: Option<u64>, properties: P, geometry: Geometry<D>, metadata: Option<M>) -> Self {
        Self { _type: "Feature".to_string(), id, properties, geometry, metadata }
    }
}
impl<M, P: MValueCompatible, D: MValueCompatible> Default for Feature<M, P, D> {
    fn default() -> Self {
        Self {
            _type: "Feature".to_string(),
            id: None,
            properties: Default::default(),
            geometry: Default::default(),
            metadata: None,
        }
    }
}

/// Component to build an WM or S2 Vector Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct VectorFeature<M = (), P: MValueCompatible = Properties, D: MValueCompatible = MValue> {
    /// Type will always be "VectorFeature"
    #[serde(rename = "type")]
    pub _type: String,
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
impl<M, P: MValueCompatible, D: MValueCompatible> Default for VectorFeature<M, P, D> {
    fn default() -> Self {
        Self {
            _type: "VectorFeature".to_string(),
            face: 0.into(),
            id: None,
            properties: Default::default(),
            geometry: Default::default(),
            metadata: None,
        }
    }
}
impl<M, P: MValueCompatible, D: MValueCompatible> VectorFeature<M, P, D> {
    /// Create a new VectorFeature in the WM format
    pub fn new_wm(
        id: Option<u64>,
        properties: P,
        geometry: VectorGeometry<D>,
        metadata: Option<M>,
    ) -> Self {
        Self {
            _type: "VectorFeature".to_string(),
            face: 0.into(),
            id,
            properties,
            geometry,
            metadata,
        }
    }

    /// Create a new VectorFeature in the WM format
    pub fn new_s2(
        id: Option<u64>,
        face: Face,
        properties: P,
        geometry: VectorGeometry<D>,
        metadata: Option<M>,
    ) -> Self {
        Self { _type: "S2Feature".to_string(), face, id, properties, geometry, metadata }
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
}

//? Utility types

/// Attribution data is stored in an object.
/// The key is the name of the attribution, and the value is the href link
/// e.g. { "Open S2": "https://opens2.com/legal/data" }
pub type Attributions = Map<String, String>;

/// Either an S2 or WM FeatureCollection
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FeatureCollections<M = (), P: MValueCompatible = Properties, D: MValueCompatible = MValue>
{
    /// An WM FeatureCollection
    FeatureCollection(FeatureCollection<M, P, D>),
    /// An S2 FeatureCollection
    S2FeatureCollection(S2FeatureCollection<M, P, D>),
}

/// Either an S2 or WM Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Features<M = (), P: MValueCompatible = Properties, D: MValueCompatible = MValue> {
    /// An WM Feature
    Feature(Feature<M, P, D>),
    /// An WM or S2 Vector Feature
    VectorFeature(VectorFeature<M, P, D>),
}

/// Either an WM Feature or an WM Vector Feature
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum WMFeature<M = (), P: MValueCompatible = Properties, D: MValueCompatible = MValue> {
    /// An WM Feature
    Feature(Feature<M, P, D>),
    /// An WM Vector Feature
    VectorFeature(VectorFeature<M, P, D>),
}

/// All major S2JSON types
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum JSONCollection<M = (), P: MValueCompatible = Properties, D: MValueCompatible = MValue> {
    /// An WM FeatureCollection
    FeatureCollection(FeatureCollection<M, P, D>),
    /// An S2 FeatureCollection
    S2FeatureCollection(S2FeatureCollection<M, P, D>),
    /// An WM Feature
    Feature(Feature<M, P, D>),
    /// An WM Vector Feature
    VectorFeature(VectorFeature<M, P, D>),
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

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

    #[test]
    fn defaults() {
        let f: Feature = Default::default();
        assert_eq!(f._type, "Feature");
        assert_eq!(f.id, None);
        assert_eq!(f.properties, Properties::default());
        assert_eq!(f.geometry, Geometry::default());
        assert_eq!(f.metadata, None);

        let f: VectorFeature = Default::default();
        assert_eq!(f._type, "VectorFeature");
        assert_eq!(f.id, None);
        assert_eq!(f.face, 0.into());
        assert_eq!(f.properties, Properties::default());
        assert_eq!(f.geometry, VectorGeometry::default());
        assert_eq!(f.metadata, None);
    }

    #[test]
    fn feature_collection_new() {
        let mut attributions = Attributions::new();
        attributions.insert("Open S2".to_string(), "https://opens2.com/legal/data".to_string());
        let mut fc = FeatureCollection::<()>::new(Some(attributions.clone()));
        assert_eq!(fc._type, "FeatureCollection");
        assert_eq!(fc.features.len(), 0);
        assert_eq!(fc.attributions, Some(attributions.clone()));
        // update_bbox
        fc.update_bbox(BBox::new(5., -2., 35., 2.2));
        assert_eq!(fc.bbox, Some(BBox::new(5., -2., 35., 2.2)));

        let string = serde_json::to_string(&fc).unwrap();
        assert_eq!(string, "{\"type\":\"FeatureCollection\",\"features\":[],\"attributions\":{\"Open S2\":\"https://opens2.com/legal/data\"},\"bbox\":[5.0,-2.0,35.0,2.2]}");
        let back_to_fc: FeatureCollection = serde_json::from_str(&string).unwrap();
        assert_eq!(back_to_fc, fc);
    }

    #[test]
    fn s2_feature_collection_new() {
        let mut attributions = Attributions::new();
        attributions.insert("Open S2".to_string(), "https://opens2.com/legal/data".to_string());
        let mut fc = S2FeatureCollection::<()>::new(Some(attributions.clone()));
        assert_eq!(fc._type, "S2FeatureCollection");
        assert_eq!(fc.features.len(), 0);
        assert_eq!(fc.attributions, Some(attributions.clone()));
        // update_bbox
        fc.update_bbox(BBox::new(5., -2., 35., 2.2));
        assert_eq!(fc.bbox, Some(BBox::new(5., -2., 35., 2.2)));
        // add face
        fc.add_face(0.into());
        fc.add_face(3.into());
        assert_eq!(fc.faces, vec![0.into(), 3.into()]);

        let string = serde_json::to_string(&fc).unwrap();
        assert_eq!(string, "{\"type\":\"S2FeatureCollection\",\"features\":[],\"faces\":[\"Face0\",\"Face3\"],\"attributions\":{\"Open S2\":\"https://opens2.com/legal/data\"},\"bbox\":[5.0,-2.0,35.0,2.2]}");
        let back_to_fc: S2FeatureCollection = serde_json::from_str(&string).unwrap();
        assert_eq!(back_to_fc, fc);
    }

    #[test]
    fn feature_new() {
        let fc: Feature = Feature::new(
            Some(22),
            Properties::new(),
            Geometry::Point(PointGeometry {
                _type: "Point".into(),
                coordinates: (0.0, 0.0),
                m_values: None,
                bbox: None,
            }),
            None,
        );
        assert_eq!(fc.id, Some(22));
        assert_eq!(fc._type, "Feature");
        assert_eq!(
            fc.geometry,
            Geometry::Point(PointGeometry {
                _type: "Point".into(),
                coordinates: (0.0, 0.0),
                m_values: None,
                bbox: None,
            })
        );
        assert_eq!(fc.properties, Properties::new());
        assert_eq!(fc.metadata, None);
    }

    #[test]
    fn s2_feature_new() {
        let fc: VectorFeature = VectorFeature::new_wm(
            Some(55),
            Properties::new(),
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint { x: 0.0, y: 1.0, z: Some(3.), m: None, t: None },
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: None,
                indices: None,
                tesselation: None,
            }),
            None,
        );
        assert_eq!(fc.id, Some(55));
        assert_eq!(fc._type, "VectorFeature");
        assert_eq!(
            fc.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint { x: 0.0, y: 1.0, z: Some(3.), m: None, t: None },
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: None,
                indices: None,
                tesselation: None,
            })
        );
        assert_eq!(fc.properties, Properties::new());
        assert_eq!(fc.metadata, None);
        assert_eq!(fc.face, 0.into());

        // S2

        #[derive(PartialEq, Clone, Debug)]
        struct MetaTest {
            name: String,
            value: String,
        }

        let fc = VectorFeature::<MetaTest>::new_s2(
            Some(55),
            3.into(),
            Properties::new(),
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint { x: 0.0, y: 1.0, z: Some(3.), m: None, t: None },
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: None,
                indices: None,
                tesselation: None,
            }),
            Some(MetaTest { name: "test".to_string(), value: "value".to_string() }),
        );
        assert_eq!(fc.id, Some(55));
        assert_eq!(fc._type, "S2Feature");
        assert_eq!(
            fc.geometry,
            VectorGeometry::Point(VectorPointGeometry {
                _type: "Point".into(),
                coordinates: VectorPoint { x: 0.0, y: 1.0, z: Some(3.), m: None, t: None },
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: None,
                indices: None,
                tesselation: None,
            })
        );
        assert_eq!(fc.properties, Properties::new());
        assert_eq!(
            fc.metadata,
            Some(MetaTest { name: "test".to_string(), value: "value".to_string() })
        );
        assert_eq!(fc.face, 3.into());

        // from_vector_feature

        let new_geo = VectorGeometry::Point(VectorPointGeometry {
            _type: "Point".into(),
            coordinates: VectorPoint { x: 5.0, y: 4.0, z: Some(-3.), m: None, t: None },
            bbox: None,
            is_3d: true,
            offset: None,
            vec_bbox: None,
            indices: None,
            tesselation: None,
        });
        let fc_clone_new_geometry =
            VectorFeature::<MetaTest>::from_vector_feature(&fc, Some(new_geo.clone()));

        assert_eq!(fc_clone_new_geometry.geometry, new_geo);
    }
}
