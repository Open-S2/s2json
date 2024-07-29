extern crate alloc;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::ser::SerializeTuple;
use serde::de::{self, SeqAccess, Visitor};

use alloc::vec::Vec;
use alloc::fmt;

/// Importing necessary types (equivalent to importing from 'values')
use crate::values::*;

/// A BBOX is defined in lon-lat space and helps with zooming motion to
/// see the entire line or polygon
/// The order is (left, bottom, right, top)
/// If WG, then the projection is lon-lat
/// If S2, then the projection is s-t
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct BBox<T = f64> {
    /// left most longitude (WG) or S (S2)
    pub left: T,
    /// bottom most latitude (WG) or T (S2)
    pub bottom: T,
    /// right most longitude (WG) or T (S2)
    pub right: T,
    /// top most latitude (WG) or S (S2)
    pub top: T,
}
impl<T> Serialize for BBox<T>
where
    T: Serialize + Copy,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(4)?;
        seq.serialize_element(&self.left)?;
        seq.serialize_element(&self.bottom)?;
        seq.serialize_element(&self.right)?;
        seq.serialize_element(&self.top)?;
        seq.end()
    }
}

impl<'de, T> Deserialize<'de> for BBox<T>
where
    T: Deserialize<'de> + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BBoxVisitor<T> {
            marker: core::marker::PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for BBoxVisitor<T>
        where
            T: Deserialize<'de> + Copy,
        {
            type Value = BBox<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of four numbers")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<BBox<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let left = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let bottom = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let right = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let top = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                Ok(BBox { left, bottom, right, top })
            }
        }

        deserializer.deserialize_tuple(4, BBoxVisitor { marker: core::marker::PhantomData })
    }
}

/// A BBOX is defined in lon-lat space and helps with zooming motion to
/// see the entire 3D line or polygon
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct BBox3D<T = f64> {
    /// left most longitude (WG) or S (S2)
    pub left: T,
    /// bottom most latitude (WG) or T (S2)
    pub bottom: T,
    /// right most longitude (WG) or T (S2)
    pub right: T,
    /// top most latitude (WG) or S (S2)
    pub top: T,
    /// back most height (WG) or T (S2)
    /// generic height is relative to the surface of the earth in meters
    pub back: T,
    /// front most height (WG) or T (S2)
    /// generic height is relative to the surface of the earth in meters
    pub front: T,
}
impl<T> Serialize for BBox3D<T>
where
    T: Serialize + Copy,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_tuple(6)?;
        seq.serialize_element(&self.left)?;
        seq.serialize_element(&self.bottom)?;
        seq.serialize_element(&self.right)?;
        seq.serialize_element(&self.top)?;
        seq.serialize_element(&self.back)?;
        seq.serialize_element(&self.front)?;
        seq.end()
    }
}

impl<'de, T> Deserialize<'de> for BBox3D<T>
where
    T: Deserialize<'de> + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct BBox3DVisitor<T> {
            marker: core::marker::PhantomData<T>,
        }

        impl<'de, T> Visitor<'de> for BBox3DVisitor<T>
        where
            T: Deserialize<'de> + Copy,
        {
            type Value = BBox3D<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a sequence of six numbers")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<BBox3D<T>, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let left = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let bottom = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let right = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let top = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let back = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let front = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(5, &self))?;
                Ok(BBox3D { left, bottom, right, top, back, front })
            }
        }

        deserializer.deserialize_tuple(6, BBox3DVisitor { marker: core::marker::PhantomData })
    }
}

/// BBox or BBox3D
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
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
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BaseGeometry<G = Geometry, M = MValues, B = BBOX> {
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
pub type MultiPolygon3DGeometry = BaseGeometry<MultiPolygon3D, MultiPolygonMValues, BBox3D>;

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_bbox() {
        let bbox = BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 };
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 });
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0]");
        let str_bbox: BBox = serde_json::from_str(&bbox_str).unwrap();
        assert_eq!(str_bbox, bbox);
    }

    #[test]
    fn test_bbox3d() {
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, back: 0.0, front: 1.0 };
        assert_eq!(bbox, BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, back: 0.0, front: 1.0 });
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0,0.0,1.0]");
        let str_bbox: BBox3D = serde_json::from_str(&bbox_str).unwrap();
        assert_eq!(str_bbox, bbox);
    }

    #[test]
    fn test_point_geometry() {
        let point = PointGeometry { coordinates: (0.0, 0.0), m_values: None, bbox: None };
        assert_eq!(point, PointGeometry { coordinates: (0.0, 0.0), m_values: None, bbox: None });
        let point_str = serde_json::to_string(&point).unwrap();
        assert_eq!(point_str, "{\"coordinates\":[0.0,0.0]}");
        let str_point: PointGeometry = serde_json::from_str(&point_str).unwrap();
        assert_eq!(str_point, point);
    }

    #[test]
    fn test_point3d_geometry() {
        let point = Point3DGeometry { coordinates: (0.0, 0.0, 0.0), m_values: None, bbox: Some(BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, back: 0.0, front: 1.0 }) };
        assert_eq!(point, Point3DGeometry { coordinates: (0.0, 0.0, 0.0), m_values: None, bbox: Some(BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, back: 0.0, front: 1.0 }) });
        let point_str = serde_json::to_string(&point).unwrap();
        assert_eq!(point_str, "{\"coordinates\":[0.0,0.0,0.0],\"bbox\":[0.0,0.0,1.0,1.0,0.0,1.0]}");
        let str_point: Point3DGeometry = serde_json::from_str(&point_str).unwrap();
        assert_eq!(str_point, point);
    }

    #[test]
    fn test_line_string_geometry() {
        let line = LineStringGeometry { coordinates: vec![(0.0, 0.0), (1.0, 1.0)], m_values: None, bbox: None };
        assert_eq!(line, LineStringGeometry { coordinates: vec![(0.0, 0.0), (1.0, 1.0)], m_values: None, bbox: None });
        let line_str = serde_json::to_string(&line).unwrap();
        assert_eq!(line_str, "{\"coordinates\":[[0.0,0.0],[1.0,1.0]]}");
        let str_line: LineStringGeometry = serde_json::from_str(&line_str).unwrap();
        assert_eq!(str_line, line);
    }

    #[test]
    fn test_line_string3d_geometry() {
        let line = LineString3DGeometry { coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)], m_values: None, bbox: None };
        assert_eq!(line, LineString3DGeometry { coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)], m_values: None, bbox: None });
        let line_str = serde_json::to_string(&line).unwrap();
        assert_eq!(line_str, "{\"coordinates\":[[0.0,0.0,0.0],[1.0,1.0,1.0]]}");
        let str_line: LineString3DGeometry = serde_json::from_str(&line_str).unwrap();
        assert_eq!(str_line, line);
    }

    #[test]
    fn test_multi_point_geometry() {
        let multi_point = MultiPointGeometry { coordinates: vec![(0.0, 0.0), (1.0, 1.0)], m_values: None, bbox: None };
        assert_eq!(multi_point, MultiPointGeometry { coordinates: vec![(0.0, 0.0), (1.0, 1.0)], m_values: None, bbox: None });
        let multi_point_str = serde_json::to_string(&multi_point).unwrap();
        assert_eq!(multi_point_str, "{\"coordinates\":[[0.0,0.0],[1.0,1.0]]}");
        let str_multi_point: MultiPointGeometry = serde_json::from_str(&multi_point_str).unwrap();
        assert_eq!(str_multi_point, multi_point);
    }

    #[test]
    fn test_multi_point3d_geometry() {
        let multi_point = MultiPoint3DGeometry { coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)], m_values: None, bbox: None };
        assert_eq!(multi_point, MultiPoint3DGeometry { coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)], m_values: None, bbox: None });
        let multi_point_str = serde_json::to_string(&multi_point).unwrap();
        assert_eq!(multi_point_str, "{\"coordinates\":[[0.0,0.0,0.0],[1.0,1.0,1.0]]}");
        let str_multi_point: MultiPoint3DGeometry = serde_json::from_str(&multi_point_str).unwrap();
        assert_eq!(str_multi_point, multi_point);
    }

    #[test]
    fn test_polygon_geometry() {
        let polygon = PolygonGeometry { coordinates: vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]], m_values: None, bbox: None };
        assert_eq!(polygon, PolygonGeometry { coordinates: vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]], m_values: None, bbox: None });
        let polygon_str = serde_json::to_string(&polygon).unwrap();
        assert_eq!(polygon_str, "{\"coordinates\":[[[0.0,0.0],[1.0,1.0],[0.0,1.0]]]}");
        let str_polygon: PolygonGeometry = serde_json::from_str(&polygon_str).unwrap();
        assert_eq!(str_polygon, polygon);
    }

    #[test]
    fn test_polygon3d_geometry() {
        let polygon = Polygon3DGeometry { coordinates: vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]], m_values: None, bbox: None };
        assert_eq!(polygon, Polygon3DGeometry { coordinates: vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]], m_values: None, bbox: None });
        let polygon_str = serde_json::to_string(&polygon).unwrap();
        assert_eq!(polygon_str, "{\"coordinates\":[[[0.0,0.0,0.0],[1.0,1.0,1.0],[0.0,1.0,1.0]]]}");
        let str_polygon: Polygon3DGeometry = serde_json::from_str(&polygon_str).unwrap();
        assert_eq!(str_polygon, polygon);
    }

    #[test]
    fn test_multi_polygon_geometry() {
        let multi_polygon = MultiPolygonGeometry { coordinates: vec![vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]]], m_values: None, bbox: None };
        assert_eq!(multi_polygon, MultiPolygonGeometry { coordinates: vec![vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]]], m_values: None, bbox: None });
        let multi_polygon_str = serde_json::to_string(&multi_polygon).unwrap();
        assert_eq!(multi_polygon_str, "{\"coordinates\":[[[[0.0,0.0],[1.0,1.0],[0.0,1.0]]]]}");
        let str_multi_polygon: MultiPolygonGeometry = serde_json::from_str(&multi_polygon_str).unwrap();
        assert_eq!(str_multi_polygon, multi_polygon);
    }

    #[test]
    fn test_multi_polygon3d_geometry() {
        let multi_polygon = MultiPolygon3DGeometry { coordinates: vec![vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]]], m_values: None, bbox: None };
        assert_eq!(multi_polygon, MultiPolygon3DGeometry { coordinates: vec![vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]]], m_values: None, bbox: None });
        let multi_polygon_str = serde_json::to_string(&multi_polygon).unwrap();
        assert_eq!(multi_polygon_str, "{\"coordinates\":[[[[0.0,0.0,0.0],[1.0,1.0,1.0],[0.0,1.0,1.0]]]]}");
        let str_multi_polygon: MultiPolygon3DGeometry = serde_json::from_str(&multi_polygon_str).unwrap();
        assert_eq!(str_multi_polygon, multi_polygon);
    }
}