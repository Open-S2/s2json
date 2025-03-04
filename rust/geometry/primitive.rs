use serde::{Deserialize, Serialize};

use alloc::vec::Vec;

/// Importing necessary types (equivalent to importing from 'values')
use crate::*;

/// Point type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum PointGeometryType {
    /// Point Type
    #[default]
    Point,
}
impl From<&str> for PointGeometryType {
    fn from(_: &str) -> Self {
        PointGeometryType::Point
    }
}

/// MultiPoint type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum MultiPointGeometryType {
    /// MultiPoint Type
    #[default]
    MultiPoint,
}
impl From<&str> for MultiPointGeometryType {
    fn from(_: &str) -> Self {
        MultiPointGeometryType::MultiPoint
    }
}

/// LineString type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum LineStringGeometryType {
    /// Point Type
    #[default]
    LineString,
}
impl From<&str> for LineStringGeometryType {
    fn from(_: &str) -> Self {
        LineStringGeometryType::LineString
    }
}

/// MultiLineString type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum MultiLineStringGeometryType {
    /// Point Type
    #[default]
    MultiLineString,
}
impl From<&str> for MultiLineStringGeometryType {
    fn from(_: &str) -> Self {
        MultiLineStringGeometryType::MultiLineString
    }
}

/// Polygon type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum PolygonGeometryType {
    /// Point Type
    #[default]
    Polygon,
}
impl From<&str> for PolygonGeometryType {
    fn from(_: &str) -> Self {
        PolygonGeometryType::Polygon
    }
}

/// MultiPolygon type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum MultiPolygonGeometryType {
    /// Point Type
    #[default]
    MultiPolygon,
}
impl From<&str> for MultiPolygonGeometryType {
    fn from(_: &str) -> Self {
        MultiPolygonGeometryType::MultiPolygon
    }
}

/// Point3D type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum Point3DGeometryType {
    /// Point Type
    #[default]
    Point3D,
}
impl From<&str> for Point3DGeometryType {
    fn from(_: &str) -> Self {
        Point3DGeometryType::Point3D
    }
}

/// MultiPoint3D type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum MultiPoint3DGeometryType {
    /// Point Type
    #[default]
    MultiPoint3D,
}
impl From<&str> for MultiPoint3DGeometryType {
    fn from(_: &str) -> Self {
        MultiPoint3DGeometryType::MultiPoint3D
    }
}

/// LineString3D type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum LineString3DGeometryType {
    /// Point Type
    #[default]
    LineString3D,
}
impl From<&str> for LineString3DGeometryType {
    fn from(_: &str) -> Self {
        LineString3DGeometryType::LineString3D
    }
}

/// MultiLineString3D type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum MultiLineString3DGeometryType {
    /// Point Type
    #[default]
    MultiLineString3D,
}
impl From<&str> for MultiLineString3DGeometryType {
    fn from(_: &str) -> Self {
        MultiLineString3DGeometryType::MultiLineString3D
    }
}

/// Polygon3D type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum Polygon3DGeometryType {
    /// Point Type
    #[default]
    Polygon3D,
}
impl From<&str> for Polygon3DGeometryType {
    fn from(_: &str) -> Self {
        Polygon3DGeometryType::Polygon3D
    }
}

/// MultiPolygon3D type for geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum MultiPolygon3DGeometryType {
    /// Point Type
    #[default]
    MultiPolygon3D,
}
impl From<&str> for MultiPolygon3DGeometryType {
    fn from(_: &str) -> Self {
        MultiPolygon3DGeometryType::MultiPolygon3D
    }
}

/// Definition of a Point. May represent WebMercator Lon-Lat or S2Geometry S-T
#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Point(pub f64, pub f64);
/// Definition of a MultiPoint
pub type MultiPoint = Vec<Point>;
/// Definition of a LineString
pub type LineString = Vec<Point>;
/// Definition of a MultiLineString
pub type MultiLineString = Vec<LineString>;
/// Definition of a Polygon
pub type Polygon = Vec<Vec<Point>>;
/// Definition of a MultiPolygon
pub type MultiPolygon = Vec<Polygon>;
/// Definition of a 3D Point. May represent WebMercator Lon-Lat or S2Geometry S-T with a z-value
#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Point3D(pub f64, pub f64, pub f64);
/// Definition of a 3D MultiPoint
pub type MultiPoint3D = Vec<Point3D>;
/// Definition of a 3D LineString
pub type LineString3D = Vec<Point3D>;
/// Definition of a 3D MultiLineString
pub type MultiLineString3D = Vec<LineString3D>;
/// Definition of a 3D Polygon
pub type Polygon3D = Vec<Vec<Point3D>>;
/// Definition of a 3D MultiPolygon
pub type MultiPolygon3D = Vec<Polygon3D>;
/// Define a Point or Point3D
#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct PointOrPoint3D(pub f64, pub f64, pub Option<f64>);

impl From<Point> for PointOrPoint3D {
    fn from(p: Point) -> Self {
        PointOrPoint3D(p.0, p.1, None)
    }
}

impl From<Point3D> for PointOrPoint3D {
    fn from(p: Point3D) -> Self {
        PointOrPoint3D(p.0, p.1, Some(p.2))
    }
}

/// All possible geometry shapes
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Geometry<M: MValueCompatible = MValue> {
    /// Point Shape
    Point(PointGeometry<M>),
    /// MultiPoint Shape
    MultiPoint(MultiPointGeometry<M>),
    /// LineString Shape
    LineString(LineStringGeometry<M>),
    /// MultiLineString Shape
    MultiLineString(MultiLineStringGeometry<M>),
    /// Polygon Shape
    Polygon(PolygonGeometry<M>),
    /// MultiPolygon Shape
    MultiPolygon(MultiPolygonGeometry<M>),
    /// Point3D Shape
    Point3D(Point3DGeometry<M>),
    /// MultiPoint3D Shape
    MultiPoint3D(MultiPoint3DGeometry<M>),
    /// LineString3D Shape
    LineString3D(LineString3DGeometry<M>),
    /// MultiLineString3D Shape
    MultiLineString3D(MultiLineString3DGeometry<M>),
    /// Polygon3D Shape
    Polygon3D(Polygon3DGeometry<M>),
    /// MultiPolygon3D Shape
    MultiPolygon3D(MultiPolygon3DGeometry<M>),
}
impl<M: MValueCompatible> Default for Geometry<M> {
    fn default() -> Self {
        Geometry::Point(PointGeometry::<M>::default())
    }
}

/// BaseGeometry is the a generic geometry type
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Default)]
pub struct BaseGeometry<T = PointGeometryType, M = MValue, G = Geometry<M>, B = BBOX> {
    /// The geometry type
    #[serde(rename = "type")]
    pub _type: T,
    /// The geometry shape
    pub coordinates: G,
    /// The M-Values shape
    #[serde(rename = "mValues", skip_serializing_if = "Option::is_none")]
    pub m_values: Option<M>,
    /// The BBox shape
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbox: Option<B>,
}

/// PointGeometry is a point
pub type PointGeometry<M = MValue> = BaseGeometry<PointGeometryType, M, Point, BBox>;
/// MultiPointGeometry contains multiple points
pub type MultiPointGeometry<M = MValue> =
    BaseGeometry<MultiPointGeometryType, LineStringMValues<M>, MultiPoint, BBox>;
/// LineStringGeometry is a line
pub type LineStringGeometry<M = MValue> =
    BaseGeometry<LineStringGeometryType, LineStringMValues<M>, LineString, BBox>;
/// MultiLineStringGeometry contains multiple lines
pub type MultiLineStringGeometry<M = MValue> =
    BaseGeometry<MultiLineStringGeometryType, MultiLineStringMValues<M>, MultiLineString, BBox>;
/// PolygonGeometry is a polygon with potential holes
pub type PolygonGeometry<M = MValue> =
    BaseGeometry<PolygonGeometryType, PolygonMValues<M>, Polygon, BBox>;
/// MultiPolygonGeometry is a polygon with multiple polygons with their own potential holes
pub type MultiPolygonGeometry<M = MValue> =
    BaseGeometry<MultiPolygonGeometryType, MultiPolygonMValues<M>, MultiPolygon, BBox>;
/// Point3DGeometry is a 3D point
pub type Point3DGeometry<M = MValue> = BaseGeometry<Point3DGeometryType, M, Point3D, BBox3D>;
/// MultiPoint3DGeometry contains multiple 3D points
pub type MultiPoint3DGeometry<M = MValue> =
    BaseGeometry<MultiPoint3DGeometryType, LineStringMValues<M>, MultiPoint3D, BBox3D>;
/// LineString3DGeometry is a 3D line
pub type LineString3DGeometry<M = MValue> =
    BaseGeometry<LineString3DGeometryType, LineStringMValues<M>, LineString3D, BBox3D>;
/// MultiLineString3DGeometry contains multiple 3D lines
pub type MultiLineString3DGeometry<M = MValue> = BaseGeometry<
    MultiLineString3DGeometryType,
    MultiLineStringMValues<M>,
    MultiLineString3D,
    BBox3D,
>;
/// Polygon3DGeometry is a 3D polygon with potential holes
pub type Polygon3DGeometry<M = MValue> =
    BaseGeometry<Polygon3DGeometryType, PolygonMValues<M>, Polygon3D, BBox3D>;
/// MultiPolygon3DGeometry is a 3D polygon with multiple polygons with their own potential holes
pub type MultiPolygon3DGeometry<M = MValue> =
    BaseGeometry<MultiPolygon3DGeometryType, MultiPolygonMValues<M>, MultiPolygon3D, BBox3D>;
