use serde::{Deserialize, Serialize};

use alloc::vec::Vec;

/// Importing necessary types (equivalent to importing from 'values')
use crate::*;

/// Vector Point type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum VectorPointGeometryType {
    /// Point Type
    #[default]
    Point,
}
impl From<&str> for VectorPointGeometryType {
    fn from(_: &str) -> Self {
        VectorPointGeometryType::Point
    }
}

/// Vector MultiPoint type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum VectorMultiPointGeometryType {
    /// Point Type
    #[default]
    MultiPoint,
}
impl From<&str> for VectorMultiPointGeometryType {
    fn from(_: &str) -> Self {
        VectorMultiPointGeometryType::MultiPoint
    }
}

/// Vector LineString type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum VectorLineStringGeometryType {
    /// Point Type
    #[default]
    LineString,
}
impl From<&str> for VectorLineStringGeometryType {
    fn from(_: &str) -> Self {
        VectorLineStringGeometryType::LineString
    }
}

/// Vector MultiLineString type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum VectorMultiLineStringGeometryType {
    /// Point Type
    #[default]
    MultiLineString,
}
impl From<&str> for VectorMultiLineStringGeometryType {
    fn from(_: &str) -> Self {
        VectorMultiLineStringGeometryType::MultiLineString
    }
}

/// Vector Polygon type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum VectorPolygonGeometryType {
    /// Point Type
    #[default]
    Polygon,
}
impl From<&str> for VectorPolygonGeometryType {
    fn from(_: &str) -> Self {
        VectorPolygonGeometryType::Polygon
    }
}

/// Vector MultiPolygon type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum VectorMultiPolygonGeometryType {
    /// Point Type
    #[default]
    MultiPolygon,
}
impl From<&str> for VectorMultiPolygonGeometryType {
    fn from(_: &str) -> Self {
        VectorMultiPolygonGeometryType::MultiPolygon
    }
}

/// Definition of a Vector MultiPoint
pub type VectorMultiPoint<M = MValue> = Vec<VectorPoint<M>>;
/// Definition of a Vector LineString
pub type VectorLineString<M = MValue> = Vec<VectorPoint<M>>;
/// Definition of a Vector MultiLineString
pub type VectorMultiLineString<M = MValue> = Vec<VectorLineString<M>>;
/// Definition of a Vector Polygon
pub type VectorPolygon<M = MValue> = Vec<VectorLineString<M>>;
/// Definition of a Vector MultiPolygon
pub type VectorMultiPolygon<M = MValue> = Vec<VectorPolygon<M>>;

/// All possible geometry shapes
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum VectorGeometry<M: MValueCompatible = MValue> {
    /// Point Shape
    Point(VectorPointGeometry<M>),
    /// MultiPoint Shape
    MultiPoint(VectorMultiPointGeometry<M>),
    /// LineString Shape
    LineString(VectorLineStringGeometry<M>),
    /// MultiLineString Shape
    MultiLineString(VectorMultiLineStringGeometry<M>),
    /// Polygon Shape
    Polygon(VectorPolygonGeometry<M>),
    /// MultiPolygon Shape
    MultiPolygon(VectorMultiPolygonGeometry<M>),
}
impl<M: MValueCompatible> VectorGeometry<M> {
    /// Get the vec_bbox of the geometry
    pub fn vec_bbox(&self) -> &Option<BBox3D> {
        match self {
            VectorGeometry::Point(g) => &g.vec_bbox,
            VectorGeometry::MultiPoint(g) => &g.vec_bbox,
            VectorGeometry::LineString(g) => &g.vec_bbox,
            VectorGeometry::MultiLineString(g) => &g.vec_bbox,
            VectorGeometry::Polygon(g) => &g.vec_bbox,
            VectorGeometry::MultiPolygon(g) => &g.vec_bbox,
        }
    }

    /// Create a new point
    pub fn new_point(coordinates: VectorPoint<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::Point(VectorPointGeometry {
            _type: VectorPointGeometryType::default(),
            is_3d: coordinates.z.is_some(),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new multipoint
    pub fn new_multipoint(coordinates: VectorMultiPoint<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::MultiPoint(VectorMultiPointGeometry {
            _type: VectorMultiPointGeometryType::default(),
            is_3d: coordinates[0].z.is_some(),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new linestring
    pub fn new_linestring(coordinates: VectorLineString<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::LineString(VectorLineStringGeometry {
            _type: VectorLineStringGeometryType::default(),
            is_3d: coordinates[0].z.is_some(),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new multilinestring
    pub fn new_multilinestring(
        coordinates: VectorMultiLineString<M>,
        bbox: Option<BBox3D>,
    ) -> Self {
        VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
            _type: VectorMultiLineStringGeometryType::default(),
            is_3d: coordinates[0][0].z.is_some(),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new polygon
    pub fn new_polygon(coordinates: VectorPolygon<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: VectorPolygonGeometryType::default(),
            is_3d: coordinates[0][0].z.is_some(),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new multipolygon
    pub fn new_multipolygon(coordinates: VectorMultiPolygon<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
            _type: VectorMultiPolygonGeometryType::default(),
            is_3d: coordinates[0][0][0].z.is_some(),
            coordinates,
            bbox,
            ..Default::default()
        })
    }
}
impl<M: MValueCompatible> Default for VectorGeometry<M> {
    fn default() -> Self {
        VectorGeometry::Point(VectorPointGeometry::default())
    }
}

/// BaseGeometry is the a generic geometry type
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct VectorBaseGeometry<T, G = VectorGeometry, O = VectorOffsets> {
    /// The geometry type
    #[serde(rename = "type")]
    pub _type: T,
    /// Specifies if the geometry is 3D or 2D
    #[serde(rename = "is3D")]
    pub is_3d: bool,
    /// The geometry shape
    pub coordinates: G,
    /// The geometry offsets if applicable
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<O>,
    /// The BBox shape - always in lon-lat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<BBox3D>,
    /// temporary bbox to track 0->1 clipping
    #[serde(skip)]
    pub vec_bbox: Option<BBox3D>,
    /// Polygon and MultiPolygon specific property
    pub indices: Option<Vec<u32>>,
    /// Polygon and MultiPolygon specific property
    pub tesselation: Option<f64>,
}

/// All possible geometry offsets
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum VectorOffsets {
    /// LineString offset
    LineOffset(VectorLineOffset),
    /// MultiLineString offset
    MultiLineOffset(VectorMultiLineOffset),
    /// Polygon offset
    PolygonOffset(VectorPolygonOffset),
    /// MultiPolygon offset
    MultiPolygonOffset(VectorMultiPolygonOffset),
}
impl Default for VectorOffsets {
    fn default() -> Self {
        VectorOffsets::LineOffset(0.0)
    }
}
/** An offset defines how far the starting line is from the original starting point pre-slice */
pub type VectorLineOffset = f64;
/** A collection of offsets */
pub type VectorMultiLineOffset = Vec<VectorLineOffset>;
/** A collection of offsets */
pub type VectorPolygonOffset = VectorMultiLineOffset;
/** A collection of collections of offsets */
pub type VectorMultiPolygonOffset = Vec<VectorPolygonOffset>;

/// PointGeometry is a point
pub type VectorPointGeometry<M = MValue> =
    VectorBaseGeometry<VectorPointGeometryType, VectorPoint<M>>;
/// MultiPointGeometry contains multiple points
pub type VectorMultiPointGeometry<M = MValue> =
    VectorBaseGeometry<VectorMultiPointGeometryType, VectorMultiPoint<M>, VectorLineOffset>;
/// LineStringGeometry is a line
pub type VectorLineStringGeometry<M = MValue> =
    VectorBaseGeometry<VectorLineStringGeometryType, VectorLineString<M>, VectorLineOffset>;
/// MultiLineStringGeometry contains multiple lines
pub type VectorMultiLineStringGeometry<M = MValue> = VectorBaseGeometry<
    VectorMultiLineStringGeometryType,
    VectorMultiLineString<M>,
    VectorMultiLineOffset,
>;
/// PolygonGeometry is a polygon with potential holes
pub type VectorPolygonGeometry<M = MValue> =
    VectorBaseGeometry<VectorPolygonGeometryType, VectorPolygon<M>, VectorPolygonOffset>;
/// MultiPolygonGeometry is a polygon with multiple polygons with their own potential holes
pub type VectorMultiPolygonGeometry<M = MValue> = VectorBaseGeometry<
    VectorMultiPolygonGeometryType,
    VectorMultiPolygon<M>,
    VectorMultiPolygonOffset,
>;
