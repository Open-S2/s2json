use crate::*;
use alloc::vec::Vec;
use core::cmp::Ordering;
use serde::{Deserialize, Serialize};

/// Definition of a Point. May represent WebMercator Lon-Lat or S2Geometry S-T
#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Point(pub f64, pub f64);
impl<P: GetXY> From<&P> for Point {
    fn from(p: &P) -> Self {
        Point(p.x(), p.y())
    }
}
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
impl<P: GetXYZ> From<&P> for Point3D {
    fn from(p: &P) -> Self {
        Point3D(p.x(), p.y(), p.z().unwrap_or_default())
    }
}
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
impl<P: GetXYZ> From<&P> for PointOrPoint3D {
    fn from(p: &P) -> Self {
        PointOrPoint3D(p.x(), p.y(), p.z())
    }
}

// GET

impl GetXY for Point {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
}
impl GetZ for Point {
    fn z(&self) -> Option<f64> {
        None
    }
}
impl GetXY for Point3D {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
}
impl GetXY for PointOrPoint3D {
    fn x(&self) -> f64 {
        self.0
    }
    fn y(&self) -> f64 {
        self.1
    }
}
impl GetZ for Point3D {
    fn z(&self) -> Option<f64> {
        Some(self.2)
    }
}
impl GetZ for PointOrPoint3D {
    fn z(&self) -> Option<f64> {
        self.2
    }
}

// SET

impl SetXY for Point {
    fn set_x(&mut self, x: f64) {
        self.0 = x;
    }
    fn set_y(&mut self, y: f64) {
        self.1 = y;
    }
    fn set_xy(&mut self, x: f64, y: f64) {
        self.0 = x;
        self.1 = y;
    }
}
impl SetXY for Point3D {
    fn set_x(&mut self, x: f64) {
        self.0 = x;
    }
    fn set_y(&mut self, y: f64) {
        self.1 = y;
    }
}
impl SetZ for Point3D {
    fn set_z(&mut self, z: f64) {
        self.2 = z;
    }
}
impl SetXY for PointOrPoint3D {
    fn set_x(&mut self, x: f64) {
        self.0 = x;
    }
    fn set_y(&mut self, y: f64) {
        self.1 = y;
    }
}
impl SetZ for PointOrPoint3D {
    fn set_z(&mut self, z: f64) {
        self.2 = Some(z);
    }
}

// NEW

impl NewXY for Point {
    fn new_xy(x: f64, y: f64) -> Self {
        Self(x, y)
    }
}
impl NewXY for Point3D {
    fn new_xy(x: f64, y: f64) -> Self {
        Self(x, y, 0.0)
    }
}
impl NewXY for PointOrPoint3D {
    fn new_xy(x: f64, y: f64) -> Self {
        Self(x, y, None)
    }
}
impl<M: Clone> NewXYM<M> for Point {
    fn new_xym(x: f64, y: f64, _m: M) -> Self {
        Self(x, y)
    }
}
impl<M: Clone> NewXYM<M> for Point3D {
    fn new_xym(x: f64, y: f64, _m: M) -> Self {
        Self(x, y, 0.0)
    }
}
impl<M: Clone> NewXYM<M> for PointOrPoint3D {
    fn new_xym(x: f64, y: f64, _m: M) -> Self {
        Self(x, y, None)
    }
}
impl NewXYZ for Point3D {
    fn new_xyz(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }
}
impl NewXYZ for PointOrPoint3D {
    fn new_xyz(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, Some(z))
    }
}
impl<M: Clone> NewXYZM<M> for Point3D {
    fn new_xyzm(x: f64, y: f64, z: f64, _m: M) -> Self {
        Self(x, y, z)
    }
}
impl<M: Clone> NewXYZM<M> for PointOrPoint3D {
    fn new_xyzm(x: f64, y: f64, z: f64, _m: M) -> Self {
        Self(x, y, Some(z))
    }
}

// Equalities

impl Eq for Point {}
impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => {}
            other => return other.unwrap_or(Ordering::Greater), /* Handle cases where `x` comparison is not equal */
        }
        match self.1.partial_cmp(&other.1) {
            Some(Ordering::Equal) => Ordering::Equal,
            other => other.unwrap_or(Ordering::Greater), /* Handle cases where `y` comparison is not equal */
        }
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Point3D {}
impl Ord for Point3D {
    fn cmp(&self, other: &Point3D) -> Ordering {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => {}
            other => return other.unwrap_or(Ordering::Greater), /* Handle cases where `x` comparison is not equal */
        }
        match self.1.partial_cmp(&other.1) {
            Some(Ordering::Equal) => {}
            other => return other.unwrap_or(Ordering::Greater), /* Handle cases where `y` comparison is not equal */
        }
        match self.2.partial_cmp(&other.2) {
            Some(order) => order,
            None => Ordering::Equal, // This handles the NaN case safely
        }
    }
}
impl PartialOrd for Point3D {
    fn partial_cmp(&self, other: &Point3D) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for PointOrPoint3D {}
impl Ord for PointOrPoint3D {
    fn cmp(&self, other: &PointOrPoint3D) -> Ordering {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => {}
            other => return other.unwrap_or(Ordering::Greater), /* Handle cases where `x` comparison is not equal */
        }
        match self.1.partial_cmp(&other.1) {
            Some(Ordering::Equal) => {}
            other => return other.unwrap_or(Ordering::Greater), /* Handle cases where `y` comparison is not equal */
        }
        match self.2.partial_cmp(&other.2) {
            Some(order) => order,
            None => Ordering::Equal, // This handles the NaN case safely
        }
    }
}
impl PartialOrd for PointOrPoint3D {
    fn partial_cmp(&self, other: &PointOrPoint3D) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Enum to represent specific geometry types as strings
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Default)]
pub enum GeometryType {
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
    /// 3D Point
    Point3D,
    /// 3D MultiPoint
    MultiPoint3D,
    /// 3D LineString
    LineString3D,
    /// 3D MultiLineString
    MultiLineString3D,
    /// 3D Polygon
    Polygon3D,
    /// 3D MultiPolygon
    MultiPolygon3D,
}
impl From<&str> for GeometryType {
    fn from(s: &str) -> Self {
        match s {
            "Point" => GeometryType::Point,
            "MultiPoint" => GeometryType::MultiPoint,
            "LineString" => GeometryType::LineString,
            "MultiLineString" => GeometryType::MultiLineString,
            "Polygon" => GeometryType::Polygon,
            "MultiPolygon" => GeometryType::MultiPolygon,
            "Point3D" => GeometryType::Point3D,
            "MultiPoint3D" => GeometryType::MultiPoint3D,
            "LineString3D" => GeometryType::LineString3D,
            "MultiLineString3D" => GeometryType::MultiLineString3D,
            "Polygon3D" => GeometryType::Polygon3D,
            "MultiPolygon3D" => GeometryType::MultiPolygon3D,
            _ => panic!("Invalid geometry type: {}", s),
        }
    }
}

/// All possible geometry shapes
#[derive(Clone, Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum Geometry<M: Clone + Default = MValue> {
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
impl<M: Clone + Default> Default for Geometry<M> {
    fn default() -> Self {
        Geometry::Point(PointGeometry::<M>::default())
    }
}

/// BaseGeometry is the a generic geometry type
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Default)]
pub struct BaseGeometry<M = MValue, G = Geometry<M>, B = BBOX> {
    /// The geometry type
    #[serde(rename = "type")]
    pub _type: GeometryType,
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
pub type PointGeometry<M = MValue> = BaseGeometry<M, Point, BBox>;
/// MultiPointGeometry contains multiple points
pub type MultiPointGeometry<M = MValue> = BaseGeometry<LineStringMValues<M>, MultiPoint, BBox>;
/// LineStringGeometry is a line
pub type LineStringGeometry<M = MValue> = BaseGeometry<LineStringMValues<M>, LineString, BBox>;
/// MultiLineStringGeometry contains multiple lines
pub type MultiLineStringGeometry<M = MValue> =
    BaseGeometry<MultiLineStringMValues<M>, MultiLineString, BBox>;
/// PolygonGeometry is a polygon with potential holes
pub type PolygonGeometry<M = MValue> = BaseGeometry<PolygonMValues<M>, Polygon, BBox>;
/// MultiPolygonGeometry is a polygon with multiple polygons with their own potential holes
pub type MultiPolygonGeometry<M = MValue> =
    BaseGeometry<MultiPolygonMValues<M>, MultiPolygon, BBox>;
/// Point3DGeometry is a 3D point
pub type Point3DGeometry<M = MValue> = BaseGeometry<M, Point3D, BBox3D>;
/// MultiPoint3DGeometry contains multiple 3D points
pub type MultiPoint3DGeometry<M = MValue> =
    BaseGeometry<LineStringMValues<M>, MultiPoint3D, BBox3D>;
/// LineString3DGeometry is a 3D line
pub type LineString3DGeometry<M = MValue> =
    BaseGeometry<LineStringMValues<M>, LineString3D, BBox3D>;
/// MultiLineString3DGeometry contains multiple 3D lines
pub type MultiLineString3DGeometry<M = MValue> =
    BaseGeometry<MultiLineStringMValues<M>, MultiLineString3D, BBox3D>;
/// Polygon3DGeometry is a 3D polygon with potential holes
pub type Polygon3DGeometry<M = MValue> = BaseGeometry<PolygonMValues<M>, Polygon3D, BBox3D>;
/// MultiPolygon3DGeometry is a 3D polygon with multiple polygons with their own potential holes
pub type MultiPolygon3DGeometry<M = MValue> =
    BaseGeometry<MultiPolygonMValues<M>, MultiPolygon3D, BBox3D>;
