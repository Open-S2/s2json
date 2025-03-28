use crate::*;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// Enum to represent specific vector geometry types as strings
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Default)]
pub enum VectorGeometryType {
    /// Point
    #[default]
    Point,
    /// MultiPoint
    MultiPoint,
    /// LineString
    LineString,
    /// MultiLineString
    MultiLineString,
    /// Polygon
    Polygon,
    /// MultiPolygon
    MultiPolygon,
}
impl From<&str> for VectorGeometryType {
    fn from(s: &str) -> Self {
        match s {
            "Point" => VectorGeometryType::Point,
            "MultiPoint" => VectorGeometryType::MultiPoint,
            "LineString" => VectorGeometryType::LineString,
            "MultiLineString" => VectorGeometryType::MultiLineString,
            "Polygon" => VectorGeometryType::Polygon,
            "MultiPolygon" => VectorGeometryType::MultiPolygon,
            _ => panic!("Invalid vector geometry type: {}", s),
        }
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
#[derive(Clone, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum VectorGeometry<M: Clone + Default = MValue> {
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
impl<M: Clone + Default> VectorGeometry<M> {
    /// Get the bbox of the geometry
    pub fn bbox(&self) -> &Option<BBox3D> {
        match self {
            VectorGeometry::Point(g) => &g.bbox,
            VectorGeometry::MultiPoint(g) => &g.bbox,
            VectorGeometry::LineString(g) => &g.bbox,
            VectorGeometry::MultiLineString(g) => &g.bbox,
            VectorGeometry::Polygon(g) => &g.bbox,
            VectorGeometry::MultiPolygon(g) => &g.bbox,
        }
    }

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
            _type: VectorGeometryType::Point,
            is_3d: coordinates.z.is_some(),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new multipoint
    pub fn new_multipoint(coordinates: VectorMultiPoint<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::MultiPoint(VectorMultiPointGeometry {
            _type: VectorGeometryType::MultiPoint,
            is_3d: coordinates.iter().any(|point| point.z.is_some()),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new linestring
    pub fn new_linestring(coordinates: VectorLineString<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::LineString(VectorLineStringGeometry {
            _type: VectorGeometryType::LineString,
            is_3d: coordinates.iter().any(|point| point.z.is_some()),
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
            _type: VectorGeometryType::MultiLineString,
            is_3d: coordinates.iter().any(|line| line.iter().any(|point| point.z.is_some())),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new polygon
    pub fn new_polygon(coordinates: VectorPolygon<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: VectorGeometryType::Polygon,
            is_3d: coordinates.iter().any(|ring| ring.iter().any(|point| point.z.is_some())),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new multipolygon
    pub fn new_multipolygon(coordinates: VectorMultiPolygon<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
            _type: VectorGeometryType::MultiPolygon,
            is_3d: coordinates.iter().any(|polygon| {
                polygon.iter().any(|ring| ring.iter().any(|point| point.z.is_some()))
            }),
            coordinates,
            bbox,
            ..Default::default()
        })
    }
}
impl<M: Clone + Default> Default for VectorGeometry<M> {
    fn default() -> Self {
        VectorGeometry::Point(VectorPointGeometry::default())
    }
}

/// BaseGeometry is the a generic geometry type
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct VectorBaseGeometry<G = VectorGeometry, O = VectorOffsets> {
    /// The geometry type
    #[serde(rename = "type")]
    pub _type: VectorGeometryType,
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
/// An offset defines how far the starting line is from the original starting point pre-slice
pub type VectorLineOffset = f64;
/// A collection of offsets
pub type VectorMultiLineOffset = Vec<VectorLineOffset>;
/// A collection of offsets
pub type VectorPolygonOffset = VectorMultiLineOffset;
/// A collection of collections of offsets
pub type VectorMultiPolygonOffset = Vec<VectorPolygonOffset>;

/// PointGeometry is a point
pub type VectorPointGeometry<M = MValue> = VectorBaseGeometry<VectorPoint<M>>;
/// MultiPointGeometry contains multiple points
pub type VectorMultiPointGeometry<M = MValue> =
    VectorBaseGeometry<VectorMultiPoint<M>, VectorLineOffset>;
/// LineStringGeometry is a line
pub type VectorLineStringGeometry<M = MValue> =
    VectorBaseGeometry<VectorLineString<M>, VectorLineOffset>;
/// MultiLineStringGeometry contains multiple lines
pub type VectorMultiLineStringGeometry<M = MValue> =
    VectorBaseGeometry<VectorMultiLineString<M>, VectorMultiLineOffset>;
/// PolygonGeometry is a polygon with potential holes
pub type VectorPolygonGeometry<M = MValue> =
    VectorBaseGeometry<VectorPolygon<M>, VectorPolygonOffset>;
/// MultiPolygonGeometry is a polygon with multiple polygons with their own potential holes
pub type VectorMultiPolygonGeometry<M = MValue> =
    VectorBaseGeometry<VectorMultiPolygon<M>, VectorMultiPolygonOffset>;
