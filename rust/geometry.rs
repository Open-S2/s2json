extern crate alloc;

use alloc::vec::Vec;

/// Importing necessary types (equivalent to importing from 'values')
use crate::values::*;

/// A BBOX is defined in lon-lat space and helps with zooming motion to
/// see the entire line or polygon
#[derive(Debug, PartialEq)]
pub struct BBox {
    /// left most longitude (WG) or S (S2)
    pub left: f64,
    /// bottom most latitude (WG) or T (S2)
    pub bottom: f64,
    /// right most longitude (WG) or T (S2)
    pub right: f64,
    /// top most latitude (WG) or S (S2)
    pub top: f64,
}

/// A BBOX is defined in lon-lat space and helps with zooming motion to
/// see the entire 3D line or polygon
#[derive(Debug, PartialEq)]
pub struct BBox3D {
    /// left most longitude (WG) or S (S2)
    pub left: f64,
    /// bottom most latitude (WG) or T (S2)
    pub bottom: f64,
    /// right most longitude (WG) or T (S2)
    pub right: f64,
    /// top most latitude (WG) or S (S2)
    pub top: f64,
    /// front most height (WG) or T (S2)
    /// generic height is relative to the surface of the earth in meters
    pub front: f64,
    /// back most height (WG) or T (S2)
    /// generic height is relative to the surface of the earth in meters
    pub back: f64,
}


/// BBox or BBox3D
#[derive(Debug, PartialEq)]
pub enum BBOX {
    /// 2D bounding box
    BBox(BBox),
    /// 3D bounding box
    BBox3D(BBox3D),
}


/// Definition of a Point. May represent WebMercator Lon-Lat or S2Geometry S-T
pub type Point = (f64, f64);
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
pub type Point3D = (f64, f64, f64);
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

/// All possible geometry shapes
#[derive(Debug, PartialEq)]
pub enum Geometry {
    /// Point Shape
    Point(PointGeometry),
    /// MultiPoint Shape
    MultiPoint(MultiPointGeometry),
    /// LineString Shape
    LineString(LineStringGeometry, Option<LineStringMValues>),
    /// MultiLineString Shape
    MultiLineString(MultiLineStringGeometry, Option<MultiLineStringMValues>),
    /// Polygon Shape
    Polygon(PolygonGeometry, Option<PolygonMValues>),
    /// MultiPolygon Shape
    MultiPolygon(MultiPolygonGeometry, Option<MultiPolygonMValues>),
    /// Point3D Shape
    Point3D(Point3DGeometry),
    /// MultiPoint3D Shape
    MultiPoint3D(MultiPoint3DGeometry),
    /// LineString3D Shape
    LineString3D(LineString3DGeometry, Option<LineStringMValues>),
    /// MultiLineString3D Shape
    MultiLineString3D(MultiLineString3DGeometry, Option<MultiLineStringMValues>),
    /// Polygon3D Shape
    Polygon3D(Polygon3DGeometry, Option<PolygonMValues>),
    /// MultiPolygon3D Shape
    MultiPolygon3D(MultiPolygon3DGeometry, Option<MultiPolygonMValues>),
}

/// BaseGeometry is the a generic geometry type
#[derive(Debug, PartialEq)]
pub struct BaseGeometry<G = Geometry, M = MValues, B = BBOX> {
    /// The geometry shape
    pub coordinates: G,
    /// The M-Values shape
    pub m_values: Option<M>,
    /// The BBox shape
    pub bbox: Option<B>,
}

/// PointGeometry is a point
pub type PointGeometry = BaseGeometry<Point, MValue, BBox>;
/// MultiPointGeometry contains multiple points
pub type MultiPointGeometry = BaseGeometry<MultiPoint, LineStringMValues, BBox>;
/// LineStringGeometry is a line
pub type LineStringGeometry = BaseGeometry<LineString, LineStringMValues, BBox>;
/// MultiLineStringGeometry contains multiple lines
pub type MultiLineStringGeometry = BaseGeometry<MultiLineString, MultiLineStringMValues, BBox>;
/// PolygonGeometry is a polygon with potential holes
pub type PolygonGeometry = BaseGeometry<Polygon, PolygonMValues, BBox>;
/// MultiPolygonGeometry is a polygon with multiple polygons with their own potential holes
pub type MultiPolygonGeometry = BaseGeometry<MultiPolygon, MultiPolygonMValues, BBox>;
/// Point3DGeometry is a 3D point
pub type Point3DGeometry = BaseGeometry<Point3D, MValue, BBox3D>;
/// MultiPoint3DGeometry contains multiple 3D points
pub type MultiPoint3DGeometry = BaseGeometry<MultiPoint3D, LineStringMValues, BBox3D>;
/// LineString3DGeometry is a 3D line
pub type LineString3DGeometry = BaseGeometry<LineString3D, LineStringMValues, BBox3D>;
/// MultiLineString3DGeometry contains multiple 3D lines
pub type MultiLineString3DGeometry = BaseGeometry<MultiLineString3D, MultiLineStringMValues, BBox3D>;
/// Polygon3DGeometry is a 3D polygon with potential holes
pub type Polygon3DGeometry = BaseGeometry<Polygon3D, PolygonMValues, BBox3D>;
/// MultiPolygon3DGeometry is a 3D polygon with multiple polygons with their own potential holes
pub type MultiPolygon3DGeometry = BaseGeometry<MultiPolygonMValues, MultiPolygonMValues, BBox3D>;
