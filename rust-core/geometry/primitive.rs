use crate::*;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

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
            _ => unreachable!(),
        }
    }
}

/// All possible geometry shapes
#[derive(Clone, Serialize, Debug, PartialEq)]
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
// We have to manually implement the Deserialize trait to fix a compilation error
#[doc(hidden)]
#[allow(unused_extern_crates, clippy::useless_attribute)]
extern crate serde as _serde;
#[automatically_derived]
#[coverage(off)]
impl<'de, M: MValueCompatible> _serde::Deserialize<'de> for Geometry<M>
where
    M: _serde::Deserialize<'de>,
{
    fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
    where
        __D: _serde::Deserializer<'de>,
    {
        let __content =
            <_serde::__private::de::Content as _serde::Deserialize>::deserialize(__deserializer)?;
        let __deserializer =
            _serde::__private::de::ContentRefDeserializer::<__D::Error>::new(&__content);
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <PointGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::Point,
        ) {
            return _serde::__private::Ok(__ok);
        }
        // Attempt to deserialize as MultiPoint then check for LineString
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiPointGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiPoint,
        ) {
            // pull out the MultiPoint variant
            if let Geometry::MultiPoint(multipoint) = &__ok {
                if multipoint._type == GeometryType::LineString {
                    // If deserialization succeeds as MultiPoint, check if content is LineString
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <LineStringGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
                        Geometry::LineString,
                    ) {
                        // If LineString is found, return LineString variant
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiLineStringGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiLineString,
        ) {
            // pull out the MultiLineString variant
            if let Geometry::MultiLineString(multilinestring) = &__ok {
                if multilinestring._type == GeometryType::Polygon {
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <PolygonGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
                        Geometry::Polygon,
                    ) {
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiPolygonGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiPolygon,
        ) {
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <Point3DGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::Point3D,
        ) {
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiPoint3DGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiPoint3D,
        ) {
            // pull out the MultiPoint3D variant
            if let Geometry::MultiPoint3D(multipoint3d) = &__ok {
                if multipoint3d._type == GeometryType::LineString3D {
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <LineString3DGeometry<M> as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        Geometry::LineString3D,
                    ) {
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiLineString3DGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiLineString3D,
        ) {
            // pull out the MultiLineString3D variant
            if let Geometry::MultiLineString3D(multilinestring3d) = &__ok {
                if multilinestring3d._type == GeometryType::Polygon3D {
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <Polygon3DGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
                        Geometry::Polygon3D,
                    ) {
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <MultiPolygon3DGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            Geometry::MultiPolygon3D,
        ) {
            return _serde::__private::Ok(__ok);
        }
        _serde::__private::Err(_serde::de::Error::custom(
            "data did not match any variant of untagged enum Geometry",
        ))
    }
}
impl<M: MValueCompatible> Default for Geometry<M> {
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
