use core::f64;

use serde::de::{self, SeqAccess, Visitor};
use serde::ser::SerializeTuple;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use alloc::fmt;
use alloc::vec::Vec;

/// Importing necessary types (equivalent to importing from 'values')
use crate::*;

/// The axis to apply an operation to
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    /// X axis
    X = 0,
    /// Y axis
    Y = 1,
}

/// A BBOX is defined in lon-lat space and helps with zooming motion to
/// see the entire line or polygon
/// The order is (left, bottom, right, top)
/// If WM, then the projection is lon-lat
/// If S2, then the projection is s-t
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct BBox<T = f64> {
    /// left most longitude (WM) or S (S2)
    pub left: T,
    /// bottom most latitude (WM) or T (S2)
    pub bottom: T,
    /// right most longitude (WM) or T (S2)
    pub right: T,
    /// top most latitude (WM) or S (S2)
    pub top: T,
}
impl<T> BBox<T> {
    /// Creates a new BBox
    pub fn new(left: T, bottom: T, right: T, top: T) -> Self
    where
        T: Copy,
    {
        BBox { left, bottom, right, top }
    }

    /// Checks if a point is within the BBox
    pub fn point_overlap<M: MValueCompatible>(&self, point: VectorPoint<M>) -> bool
    where
        T: Into<f64> + Copy, // Ensures that comparison operators work for type T
    {
        point.x >= self.left.into()
            && point.x <= self.right.into()
            && point.y >= self.bottom.into()
            && point.y <= self.top.into()
    }

    /// Merges another bounding box with this one
    pub fn merge(&self, other: &BBox<T>) -> BBox<T>
    where
        T: PartialOrd + Copy,
    {
        let mut new_bbox = *self;
        new_bbox.left = if new_bbox.left < other.left { new_bbox.left } else { other.left };
        new_bbox.bottom =
            if new_bbox.bottom < other.bottom { new_bbox.bottom } else { other.bottom };
        new_bbox.right = if new_bbox.right > other.right { new_bbox.right } else { other.right };
        new_bbox.top = if new_bbox.top > other.top { new_bbox.top } else { other.top };

        new_bbox
    }

    /// Checks if another bounding box overlaps with this one and returns the overlap
    pub fn overlap(&self, other: &BBox<T>) -> Option<BBox<T>>
    where
        T: PartialOrd + Copy,
    {
        if self.left > other.right
            || self.right < other.left
            || self.bottom > other.top
            || self.top < other.bottom
        {
            None
        } else {
            let left = if self.left > other.left { self.left } else { other.left };
            let bottom = if self.bottom > other.bottom { self.bottom } else { other.bottom };
            let right = if self.right < other.right { self.right } else { other.right };
            let top = if self.top < other.top { self.top } else { other.top };

            Some(BBox { left, bottom, right, top })
        }
    }

    /// Clips the bounding box along an axis
    pub fn clip(&self, axis: Axis, k1: T, k2: T) -> BBox<T>
    where
        T: PartialOrd + Copy,
    {
        let mut new_bbox = *self;
        if axis == Axis::X {
            new_bbox.left = if new_bbox.left > k1 { new_bbox.left } else { k1 };
            new_bbox.right = if new_bbox.right < k2 { new_bbox.right } else { k2 };
        } else {
            new_bbox.bottom = if new_bbox.bottom > k1 { new_bbox.bottom } else { k1 };
            new_bbox.top = if new_bbox.top < k2 { new_bbox.top } else { k2 };
        }

        new_bbox
    }
}
impl Default for BBox<f64> {
    fn default() -> Self {
        BBox::new(f64::INFINITY, f64::INFINITY, -f64::INFINITY, -f64::INFINITY)
    }
}
impl BBox<f64> {
    /// Creates a new BBox from a point
    pub fn from_point<M: MValueCompatible>(point: &VectorPoint<M>) -> Self {
        BBox::new(point.x, point.y, point.x, point.y)
    }

    /// Creates a new BBox from a linestring
    pub fn from_linestring<M: MValueCompatible>(line: &VectorLineString<M>) -> Self {
        let mut bbox = BBox::from_point(&line[0]);
        for point in line {
            bbox.extend_from_point(point);
        }
        bbox
    }

    /// Creates a new BBox from a multi-linestring
    pub fn from_multi_linestring<M: MValueCompatible>(lines: &VectorMultiLineString<M>) -> Self {
        let mut bbox = BBox::from_point(&lines[0][0]);
        for line in lines {
            for point in line {
                bbox.extend_from_point(point);
            }
        }
        bbox
    }

    /// Creates a new BBox from a polygon
    pub fn from_polygon<M: MValueCompatible>(polygon: &VectorPolygon<M>) -> Self {
        BBox::<f64>::from_multi_linestring(polygon)
    }

    /// Creates a new BBox from a multi-polygon
    pub fn from_multi_polygon<M: MValueCompatible>(polygons: &VectorMultiPolygon<M>) -> Self {
        let mut bbox = BBox::from_point(&polygons[0][0][0]);
        for polygon in polygons {
            for line in polygon {
                for point in line {
                    bbox.extend_from_point(point);
                }
            }
        }
        bbox
    }

    /// Extends the bounding box with a point
    pub fn extend_from_point<M: MValueCompatible>(&mut self, point: &VectorPoint<M>) {
        *self = self.merge(&BBox::from_point(point));
    }

    /// Creates a new BBox from zoom-uv coordinates
    pub fn from_uv_zoom(u: f64, v: f64, zoom: u8) -> Self {
        let division_factor = 2. / (1 << zoom) as f64;

        BBox {
            left: division_factor * u - 1.0,
            bottom: division_factor * v - 1.0,
            right: division_factor * (u + 1.0) - 1.0,
            top: division_factor * (v + 1.0) - 1.0,
        }
    }

    /// Creates a new BBox from zoom-st coordinates
    pub fn from_st_zoom(s: f64, t: f64, zoom: u8) -> Self {
        let division_factor = (2. / (1 << zoom) as f64) * 0.5;

        BBox {
            left: division_factor * s,
            bottom: division_factor * t,
            right: division_factor * (s + 1.),
            top: division_factor * (t + 1.),
        }
    }
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
                let left =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let bottom =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let right =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let top = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(3, &self))?;
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
    /// left most longitude (WM) or S (S2)
    pub left: T,
    /// bottom most latitude (WM) or T (S2)
    pub bottom: T,
    /// right most longitude (WM) or T (S2)
    pub right: T,
    /// top most latitude (WM) or S (S2)
    pub top: T,
    /// near most height (WM) or T (S2)
    /// generic height is relative to the surface of the earth in meters
    pub near: T,
    /// far most height (WM) or T (S2)
    /// generic height is relative to the surface of the earth in meters
    pub far: T,
}
impl<T> BBox3D<T> {
    /// Creates a new BBox3D
    pub fn new(left: T, bottom: T, right: T, top: T, near: T, far: T) -> Self
    where
        T: Copy,
    {
        BBox3D { left, bottom, right, top, near, far }
    }

    /// Checks if a point is within the BBox
    pub fn point_overlap<M: MValueCompatible>(&self, point: VectorPoint<M>) -> bool
    where
        T: Into<f64> + Copy, // Ensures that comparison operators work for type T
    {
        let z = point.z.unwrap_or_default();
        point.x >= self.left.into()
            && point.x <= self.right.into()
            && point.y >= self.bottom.into()
            && point.y <= self.top.into()
            && z >= self.near.into()
            && z <= self.far.into()
    }

    /// Merges another bounding box with this one
    pub fn merge(&self, other: &BBox3D<T>) -> BBox3D<T>
    where
        T: PartialOrd + Copy,
    {
        let mut new_bbox = *self;
        new_bbox.left = if new_bbox.left < other.left { new_bbox.left } else { other.left };
        new_bbox.bottom =
            if new_bbox.bottom < other.bottom { new_bbox.bottom } else { other.bottom };
        new_bbox.right = if new_bbox.right > other.right { new_bbox.right } else { other.right };
        new_bbox.top = if new_bbox.top > other.top { new_bbox.top } else { other.top };
        new_bbox.near = if new_bbox.near < other.near { new_bbox.near } else { other.near };
        new_bbox.far = if new_bbox.far > other.far { new_bbox.far } else { other.far };

        new_bbox
    }

    /// Checks if another bounding box overlaps with this one and returns the overlap
    pub fn overlap(&self, other: &BBox3D<T>) -> Option<BBox3D<T>>
    where
        T: PartialOrd + Copy,
    {
        if self.left > other.right
            || self.right < other.left
            || self.bottom > other.top
            || self.top < other.bottom
            || self.near > other.far
            || self.far < other.near
        {
            None
        } else {
            let left = if self.left > other.left { self.left } else { other.left };
            let bottom = if self.bottom > other.bottom { self.bottom } else { other.bottom };
            let right = if self.right < other.right { self.right } else { other.right };
            let top = if self.top < other.top { self.top } else { other.top };

            let near = if self.near > other.near { self.near } else { other.near };
            let far = if self.far < other.far { self.far } else { other.far };

            Some(BBox3D { left, bottom, right, top, near, far })
        }
    }

    /// Clips the bounding box along an axis
    pub fn clip(&self, axis: Axis, k1: T, k2: T) -> BBox3D<T>
    where
        T: PartialOrd + Copy,
    {
        let mut new_bbox = *self;
        if axis == Axis::X {
            new_bbox.left = if new_bbox.left > k1 { new_bbox.left } else { k1 };
            new_bbox.right = if new_bbox.right < k2 { new_bbox.right } else { k2 };
        } else {
            new_bbox.bottom = if new_bbox.bottom > k1 { new_bbox.bottom } else { k1 };
            new_bbox.top = if new_bbox.top < k2 { new_bbox.top } else { k2 };
        }

        new_bbox
    }
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
        seq.serialize_element(&self.near)?;
        seq.serialize_element(&self.far)?;
        seq.end()
    }
}
impl Default for BBox3D<f64> {
    fn default() -> Self {
        BBox3D::new(
            f64::INFINITY,
            f64::INFINITY,
            -f64::INFINITY,
            -f64::INFINITY,
            f64::INFINITY,
            -f64::INFINITY,
        )
    }
}
impl BBox3D<f64> {
    /// Creates a new BBox3D from a point
    pub fn from_point<M: MValueCompatible>(point: &VectorPoint<M>) -> Self {
        BBox3D::new(
            point.x,
            point.y,
            point.x,
            point.y,
            point.z.unwrap_or(f64::INFINITY),
            point.z.unwrap_or(-f64::INFINITY),
        )
    }

    /// Creates a new BBox from a linestring
    pub fn from_linestring<M: MValueCompatible>(line: &VectorLineString<M>) -> Self {
        let mut bbox = BBox3D::from_point(&line[0]);
        for point in line {
            bbox.extend_from_point(point);
        }
        bbox
    }

    /// Creates a new BBox from a multi-linestring
    pub fn from_multi_linestring<M: MValueCompatible>(lines: &VectorMultiLineString<M>) -> Self {
        let mut bbox = BBox3D::from_point(&lines[0][0]);
        for line in lines {
            for point in line {
                bbox.extend_from_point(point);
            }
        }
        bbox
    }

    /// Creates a new BBox from a polygon
    pub fn from_polygon<M: MValueCompatible>(polygon: &VectorPolygon<M>) -> Self {
        BBox3D::<f64>::from_multi_linestring(polygon)
    }

    /// Creates a new BBox from a multi-polygon
    pub fn from_multi_polygon<M: MValueCompatible>(polygons: &VectorMultiPolygon<M>) -> Self {
        let mut bbox = BBox3D::from_point(&polygons[0][0][0]);
        for polygon in polygons {
            for line in polygon {
                for point in line {
                    bbox.extend_from_point(point);
                }
            }
        }
        bbox
    }

    /// Creates a new BBox3D from a BBox
    pub fn from_bbox(bbox: &BBox) -> Self {
        BBox3D::new(bbox.left, bbox.bottom, bbox.right, bbox.top, 0., 0.)
    }

    /// Extends the bounding box with a point
    pub fn extend_from_point<M: MValueCompatible>(&mut self, point: &VectorPoint<M>) {
        *self = self.merge(&BBox3D::from_point(point));
    }

    /// Creates a new BBox3D from zoom-uv coordinates
    pub fn from_uv_zoom(u: f64, v: f64, zoom: u8) -> Self {
        let division_factor = 2. / (1 << zoom) as f64;

        BBox3D {
            left: division_factor * u - 1.0,
            bottom: division_factor * v - 1.0,
            right: division_factor * (u + 1.0) - 1.0,
            top: division_factor * (v + 1.0) - 1.0,
            near: f64::INFINITY,
            far: -f64::INFINITY,
        }
    }

    /// Creates a new BBox from zoom-st coordinates
    pub fn from_st_zoom(s: f64, t: f64, zoom: u8) -> Self {
        let division_factor = (2. / (1 << zoom) as f64) * 0.5;

        BBox3D {
            left: division_factor * s,
            bottom: division_factor * t,
            right: division_factor * (s + 1.),
            top: division_factor * (t + 1.),
            near: f64::INFINITY,
            far: -f64::INFINITY,
        }
    }
}
impl From<BBox> for BBox3D<f64> {
    fn from(bbox: BBox) -> Self {
        BBox3D::from_bbox(&bbox)
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
                let left =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let bottom =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let right =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let top = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(3, &self))?;
                let near =
                    seq.next_element()?.ok_or_else(|| de::Error::invalid_length(4, &self))?;
                let far = seq.next_element()?.ok_or_else(|| de::Error::invalid_length(5, &self))?;
                Ok(BBox3D { left, bottom, right, top, near, far })
            }
        }

        deserializer.deserialize_tuple(6, BBox3DVisitor { marker: core::marker::PhantomData })
    }
}

/// BBox or BBox3D
#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub enum BBOX {
    /// 2D bounding box
    BBox(BBox),
    /// 3D bounding box
    BBox3D(BBox3D),
}
impl Default for BBOX {
    fn default() -> Self {
        BBOX::BBox(BBox::default())
    }
}

/// A Point in S2 Space with a Face
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub struct STPoint<M: MValueCompatible = MValue> {
    /// The face of the point
    pub face: Face,
    /// The s coordinate
    pub s: f64,
    /// The t coordinate
    pub t: f64,
    /// The z coordinate
    pub z: Option<f64>,
    /// The m coordinate
    pub m: Option<M>,
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
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
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
}
impl Default for VectorGeometry {
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
/** An offset defines how far the starting line is from the original starting point pre-slice */
pub type VectorLineOffset = f64;
/** A collection of offsets */
pub type VectorMultiLineOffset = Vec<VectorLineOffset>;
/** A collection of offsets */
pub type VectorPolygonOffset = VectorMultiLineOffset;
/** A collection of collections of offsets */
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_vector_offset() {
        let offset = VectorOffsets::default();
        assert_eq!(offset, VectorOffsets::LineOffset(0.0));
        let offset: VectorOffsets = Default::default();
        assert_eq!(offset, VectorOffsets::LineOffset(0.0));
    }

    #[test]
    fn test_bbox() {
        let bbox = BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 };
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 });
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0]");
        let str_bbox: BBox = serde_json::from_str(&bbox_str).unwrap();
        assert_eq!(str_bbox, bbox);

        let default_bbox = BBox::default();
        assert_eq!(
            default_bbox,
            BBox {
                left: f64::INFINITY,
                bottom: f64::INFINITY,
                right: -f64::INFINITY,
                top: -f64::INFINITY
            }
        );

        let default_bbox_2 = BBOX::default();
        assert_eq!(
            default_bbox_2,
            BBOX::BBox(BBox {
                left: f64::INFINITY,
                bottom: f64::INFINITY,
                right: -f64::INFINITY,
                top: -f64::INFINITY
            })
        );
    }

    #[test]
    fn test_bbox_serialize() {
        let bbox = BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 };
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0]");
    }

    #[test]
    fn test_bbox_deserialize() {
        let bbox_str = "[0.0,0.0,1.0,1.0]";
        let bbox: BBox = serde_json::from_str(bbox_str).unwrap();
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0 });
    }

    #[test]
    fn test_bbox_clip() {
        let bbox = BBox::new(0., 0., 1., 1.);
        let bbox2 = BBox { left: 0.5, bottom: 0., right: 0.75, top: 1. };
        assert_eq!(bbox.clip(Axis::X, 0.5, 0.75), bbox2);
        let bbox2 = BBox { left: 0., bottom: 0.5, right: 1., top: 0.75 };
        assert_eq!(bbox.clip(Axis::Y, 0.5, 0.75), bbox2);
    }

    #[test]
    fn test_bbox_overlap() {
        let bbox = BBox::new(0., 0., 1., 1.);
        assert!(bbox.point_overlap(VectorPoint::<MValue>::new(0.5, 0.5, None, None)));
        assert!(!bbox.point_overlap(VectorPoint::<MValue>::new(2.0, 2.0, None, None)));
        let bbox2 = BBox { left: 0.5, bottom: 0.5, right: 1.5, top: 1.5 };
        assert_eq!(
            bbox.overlap(&bbox2),
            Some(BBox { left: 0.5, bottom: 0.5, right: 1.0, top: 1.0 })
        );
        let bbox3 = BBox { left: 2.0, bottom: 2.0, right: 3.0, top: 3.0 };
        assert_eq!(bbox3.overlap(&bbox), None);
    }

    #[test]
    fn test_bbox_merge() {
        let bbox = BBox::new(0., 0., 1., 1.);
        let bbox2 = BBox { left: 0.5, bottom: 0.5, right: 1.5, top: 1.5 };
        assert_eq!(bbox.merge(&bbox2), BBox { left: 0.0, bottom: 0.0, right: 1.5, top: 1.5 });
        assert_eq!(bbox2.merge(&bbox), BBox { left: 0.0, bottom: 0.0, right: 1.5, top: 1.5 });
        let bbox3 = BBox { left: 2.0, bottom: 2.0, right: 3.0, top: 3.0 };
        assert_eq!(bbox.merge(&bbox3), BBox { left: 0.0, bottom: 0.0, right: 3.0, top: 3.0 });
    }

    #[test]
    fn test_bbox_from_st_uv() {
        let bbox = BBox::from_st_zoom(0., 0., 0);
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1., top: 1. });

        let bbox = BBox::from_st_zoom(1., 0., 1);
        assert_eq!(bbox, BBox { left: 0.5, bottom: 0.0, right: 1., top: 0.5 });

        let bbox = BBox::from_st_zoom(2., 0., 2);
        assert_eq!(bbox, BBox { left: 0.5, bottom: 0.0, right: 0.75, top: 0.25 });

        let bbox = BBox::from_uv_zoom(0., 0., 0);
        assert_eq!(bbox, BBox { left: -1.0, bottom: -1.0, right: 1., top: 1. });

        let bbox = BBox::from_uv_zoom(1., 0., 1);
        assert_eq!(bbox, BBox { left: 0., bottom: -1.0, right: 1., top: 0. });

        let bbox = BBox::from_uv_zoom(2., 0., 2);
        assert_eq!(bbox, BBox { left: 0., bottom: -1.0, right: 0.5, top: -0.5 });
    }

    #[test]
    fn test_bbox_from_point() {
        let bbox = BBox::from_point(&VectorPoint::<MValue>::new(0., 0., None, None));
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 0.0, top: 0.0 });
    }

    #[test]
    fn test_bbox_from_linestring() {
        let bbox = BBox::from_linestring(&vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(1., 1.5, None, None),
        ]);
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.5 });
    }

    #[test]
    fn test_bbox_from_multilinestring() {
        let bbox = BBox::from_multi_linestring(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(1., 1.5, None, None),
        ]]);
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 1.0, top: 1.5 });
    }

    #[test]
    fn test_bbox_from_polygon() {
        let bbox = BBox::from_polygon(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(2., 1.5, None, None),
        ]]);
        assert_eq!(bbox, BBox { left: 0.0, bottom: 0.0, right: 2.0, top: 1.5 });
    }

    #[test]
    fn test_bbox_from_multipolygon() {
        let bbox = BBox::from_multi_polygon(&vec![
            vec![vec![
                VectorPoint::<MValue>::new(0., 0., None, None),
                VectorPoint::new(2., 1.5, None, None),
            ]],
            vec![vec![
                VectorPoint::new(0., 0., None, None),
                VectorPoint::new(-1., 3.5, None, None),
            ]],
        ]);
        assert_eq!(bbox, BBox { left: -1.0, bottom: 0.0, right: 2.0, top: 3.5 });
    }

    #[test]
    fn test_bbox3d() {
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 1.0 };
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 1.0 }
        );
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0,0.0,1.0]");
        let str_bbox: BBox3D = serde_json::from_str(&bbox_str).unwrap();
        assert_eq!(str_bbox, bbox);

        let default_bbox = BBox3D::default();
        assert_eq!(
            default_bbox,
            BBox3D {
                left: f64::INFINITY,
                bottom: f64::INFINITY,
                right: -f64::INFINITY,
                top: -f64::INFINITY,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
    }

    #[test]
    fn test_bbox_3d_serialize() {
        let bbox = BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 1.0 };
        let bbox_str = serde_json::to_string(&bbox).unwrap();
        assert_eq!(bbox_str, "[0.0,0.0,1.0,1.0,0.0,1.0]");
    }

    #[test]
    fn test_bbox_3_d_deserialize() {
        let bbox_str = "[0.0,0.0,1.0,1.0,0.0,1.0]";
        let bbox: BBox3D = serde_json::from_str(bbox_str).unwrap();
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 1.0 }
        );
    }

    #[test]
    fn test_bbox_3_d_overlap() {
        let bbox = BBox3D::new(0., 0., 1., 1., 0., 1.);
        assert!(bbox.point_overlap(VectorPoint::<MValue>::new(0.5, 0.5, None, None)));
        assert!(!bbox.point_overlap(VectorPoint::<MValue>::new(2.0, 2.0, None, None)));
        let bbox2 = BBox3D { left: 0.5, bottom: 0.5, right: 1.5, top: 1.5, near: 0.5, far: 1.5 };
        assert_eq!(
            bbox.overlap(&bbox2),
            Some(BBox3D { left: 0.5, bottom: 0.5, right: 1.0, top: 1.0, near: 0.5, far: 1.0 })
        );
        let bbox3 = BBox3D { left: 2.0, bottom: 2.0, right: 3.0, top: 3.0, near: 2.0, far: 3.0 };
        assert_eq!(bbox3.overlap(&bbox), None);
    }

    #[test]
    fn test_bbox_3_d_clip() {
        let bbox = BBox3D::new(0., 0., 1., 1., -1., 5.);
        let bbox2 = BBox3D { left: 0.5, bottom: 0., right: 0.75, top: 1., near: -1., far: 5. };
        assert_eq!(bbox.clip(Axis::X, 0.5, 0.75), bbox2);
        let bbox2 = BBox3D { left: 0., bottom: 0.5, right: 1., top: 0.75, near: -1., far: 5. };
        assert_eq!(bbox.clip(Axis::Y, 0.5, 0.75), bbox2);
    }

    #[test]
    fn test_bbox_3_d_merge() {
        let bbox = BBox3D::new(0., 0., 1., 1., 0., 1.);
        let bbox2 = BBox3D { left: 0.5, bottom: 0.5, right: 1.5, top: 1.5, near: 0.5, far: 1.5 };
        assert_eq!(
            bbox.merge(&bbox2),
            BBox3D { left: 0.0, bottom: 0.0, right: 1.5, top: 1.5, near: 0.0, far: 1.5 }
        );
        assert_eq!(
            bbox2.merge(&bbox),
            BBox3D { left: 0.0, bottom: 0.0, right: 1.5, top: 1.5, near: 0.0, far: 1.5 }
        );
        let bbox3 = BBox3D { left: 2.0, bottom: 2.0, right: 3.0, top: 3.0, near: 2.0, far: 3.0 };
        assert_eq!(
            bbox.merge(&bbox3),
            BBox3D { left: 0.0, bottom: 0.0, right: 3.0, top: 3.0, near: 0.0, far: 3.0 }
        );
    }

    #[test]
    fn test_bbox_3_d_from_point() {
        let bbox = BBox3D::from_point(&VectorPoint::<MValue>::new(0., 0., None, None));
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 0.0,
                top: 0.0,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
    }

    #[test]
    fn test_bbox_3_d_from_linestring() {
        let bbox = BBox3D::from_linestring(&vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(1., 1.5, None, None),
        ]);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 1.0,
                top: 1.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
    }

    #[test]
    fn test_bbox_3_d_from_multilinestring() {
        let bbox = BBox3D::from_multi_linestring(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(1., 1.5, None, None),
        ]]);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 1.0,
                top: 1.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
    }

    #[test]
    fn test_bbox_3_d_from_polygon() {
        let bbox = BBox3D::from_polygon(&vec![vec![
            VectorPoint::<MValue>::new(0., 0., None, None),
            VectorPoint::new(2., 1.5, None, None),
        ]]);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 2.0,
                top: 1.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
    }

    #[test]
    fn test_bbox_3_d_from_multipolygon() {
        let bbox = BBox3D::from_multi_polygon(&vec![
            vec![vec![
                VectorPoint::<MValue>::new(0., 0., None, None),
                VectorPoint::new(2., 1.5, None, None),
            ]],
            vec![vec![
                VectorPoint::new(0., 0., None, None),
                VectorPoint::new(-1., 3.5, None, None),
            ]],
        ]);
        assert_eq!(
            bbox,
            BBox3D {
                left: -1.0,
                bottom: 0.0,
                right: 2.0,
                top: 3.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
    }

    #[test]
    fn test_bbox_3_d_extend_from_point() {
        let mut bbox = BBox3D::default();
        bbox.extend_from_point(&VectorPoint::<MValue>::new(20., -4., None, None));
        assert_eq!(
            bbox,
            BBox3D {
                left: 20.0,
                bottom: -4.0,
                right: 20.0,
                top: -4.0,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
    }

    #[test]
    fn test_bbox_3_d_from_st_uv() {
        let bbox = BBox3D::from_st_zoom(0., 0., 0);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 1.,
                top: 1.,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );

        let bbox = BBox3D::from_st_zoom(1., 0., 1);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.5,
                bottom: 0.0,
                right: 1.,
                top: 0.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );

        let bbox = BBox3D::from_st_zoom(2., 0., 2);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.5,
                bottom: 0.0,
                right: 0.75,
                top: 0.25,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );

        let bbox = BBox3D::from_uv_zoom(0., 0., 0);
        assert_eq!(
            bbox,
            BBox3D {
                left: -1.0,
                bottom: -1.0,
                right: 1.,
                top: 1.,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );

        let bbox = BBox3D::from_uv_zoom(1., 0., 1);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.,
                bottom: -1.0,
                right: 1.,
                top: 0.,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );

        let bbox = BBox3D::from_uv_zoom(2., 0., 2);
        assert_eq!(
            bbox,
            BBox3D {
                left: 0.,
                bottom: -1.0,
                right: 0.5,
                top: -0.5,
                near: f64::INFINITY,
                far: -f64::INFINITY
            }
        );
    }

    #[test]
    fn test_bbox_3_d_from_bbox() {
        let bbox: BBox3D = BBox::new(0., 0., 1., 1.).into();
        assert_eq!(
            bbox,
            BBox3D { left: 0.0, bottom: 0.0, right: 1.0, top: 1.0, near: 0.0, far: 0.0 }
        );
    }

    #[test]
    fn test_point_geometry() {
        let point = PointGeometry {
            _type: "Point".into(),
            coordinates: (0.0, 0.0),
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            point,
            PointGeometry {
                _type: "Point".into(),
                coordinates: (0.0, 0.0),
                m_values: None,
                bbox: None
            }
        );
        let point_str = serde_json::to_string(&point).unwrap();
        assert_eq!(point_str, "{\"type\":\"Point\",\"coordinates\":[0.0,0.0]}");
        let str_point: PointGeometry = serde_json::from_str(&point_str).unwrap();
        assert_eq!(str_point, point);
    }

    #[test]
    fn test_geometry_default() {
        let geo = Geometry::default();
        assert_eq!(geo, Geometry::Point(PointGeometry::default()));

        let default_instance: Geometry = Default::default();
        assert_eq!(geo, default_instance);
    }

    #[test]
    fn test_point3d_geometry() {
        let point = Point3DGeometry {
            _type: "Point3D".into(),
            coordinates: (0.0, 0.0, 0.0),
            m_values: None,
            bbox: Some(BBox3D {
                left: 0.0,
                bottom: 0.0,
                right: 1.0,
                top: 1.0,
                near: 0.0,
                far: 1.0,
            }),
        };
        assert_eq!(
            point,
            Point3DGeometry {
                _type: "Point3D".into(),
                coordinates: (0.0, 0.0, 0.0),
                m_values: None,
                bbox: Some(BBox3D {
                    left: 0.0,
                    bottom: 0.0,
                    right: 1.0,
                    top: 1.0,
                    near: 0.0,
                    far: 1.0
                })
            }
        );
        let point_str = serde_json::to_string(&point).unwrap();
        assert_eq!(
            point_str,
            "{\"type\":\"Point3D\",\"coordinates\":[0.0,0.0,0.0],\"bbox\":[0.0,0.0,1.0,1.0,0.0,1.\
             0]}"
        );
        let str_point: Point3DGeometry = serde_json::from_str(&point_str).unwrap();
        assert_eq!(str_point, point);
    }

    #[test]
    fn test_line_string_geometry() {
        let line = LineStringGeometry {
            _type: "LineString".into(),
            coordinates: vec![(0.0, 0.0), (1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            line,
            LineStringGeometry {
                _type: "LineString".into(),
                coordinates: vec![(0.0, 0.0), (1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let line_str = serde_json::to_string(&line).unwrap();
        assert_eq!(line_str, "{\"type\":\"LineString\",\"coordinates\":[[0.0,0.0],[1.0,1.0]]}");
        let str_line: LineStringGeometry = serde_json::from_str(&line_str).unwrap();
        assert_eq!(str_line, line);
    }

    #[test]
    fn test_line_string3d_geometry() {
        let line = LineString3DGeometry::<MValue> {
            _type: "LineString3D".into(),
            coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            line,
            LineString3DGeometry {
                _type: "LineString3D".into(),
                coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let line_str = serde_json::to_string(&line).unwrap();
        assert_eq!(
            line_str,
            "{\"type\":\"LineString3D\",\"coordinates\":[[0.0,0.0,0.0],[1.0,1.0,1.0]]}"
        );
        let str_line: LineString3DGeometry = serde_json::from_str(&line_str).unwrap();
        assert_eq!(str_line, line);
    }

    #[test]
    fn test_multi_point_geometry() {
        let multi_point = MultiPointGeometry {
            _type: "MultiPoint".into(),
            coordinates: vec![(0.0, 0.0), (1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_point,
            MultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![(0.0, 0.0), (1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let multi_point_str = serde_json::to_string(&multi_point).unwrap();
        assert_eq!(
            multi_point_str,
            "{\"type\":\"MultiPoint\",\"coordinates\":[[0.0,0.0],[1.0,1.0]]}"
        );
        let str_multi_point: MultiPointGeometry = serde_json::from_str(&multi_point_str).unwrap();
        assert_eq!(str_multi_point, multi_point);
    }

    #[test]
    fn test_multi_point3d_geometry() {
        let multi_point = MultiPoint3DGeometry {
            _type: "MultiPoint3D".into(),
            coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_point,
            MultiPoint3DGeometry {
                _type: "MultiPoint3D".into(),
                coordinates: vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0)],
                m_values: None,
                bbox: None
            }
        );
        let multi_point_str = serde_json::to_string(&multi_point).unwrap();
        assert_eq!(
            multi_point_str,
            "{\"type\":\"MultiPoint3D\",\"coordinates\":[[0.0,0.0,0.0],[1.0,1.0,1.0]]}"
        );
        let str_multi_point: MultiPoint3DGeometry = serde_json::from_str(&multi_point_str).unwrap();
        assert_eq!(str_multi_point, multi_point);
    }

    #[test]
    fn test_polygon_geometry() {
        let polygon = PolygonGeometry {
            _type: "Polygon".into(),
            coordinates: vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            polygon,
            PolygonGeometry {
                _type: "Polygon".into(),
                coordinates: vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
                m_values: None,
                bbox: None
            }
        );
        let polygon_str = serde_json::to_string(&polygon).unwrap();
        assert_eq!(
            polygon_str,
            "{\"type\":\"Polygon\",\"coordinates\":[[[0.0,0.0],[1.0,1.0],[0.0,1.0]]]}"
        );
        let str_polygon: PolygonGeometry = serde_json::from_str(&polygon_str).unwrap();
        assert_eq!(str_polygon, polygon);
    }

    #[test]
    fn test_polygon3d_geometry() {
        let polygon = Polygon3DGeometry {
            _type: "Polygon3D".into(),
            coordinates: vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            polygon,
            Polygon3DGeometry {
                _type: "Polygon3D".into(),
                coordinates: vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]],
                m_values: None,
                bbox: None
            }
        );
        let polygon_str = serde_json::to_string(&polygon).unwrap();
        assert_eq!(
            polygon_str,
            "{\"type\":\"Polygon3D\",\"coordinates\":[[[0.0,0.0,0.0],[1.0,1.0,1.0],[0.0,1.0,1.\
             0]]]}"
        );
        let str_polygon: Polygon3DGeometry = serde_json::from_str(&polygon_str).unwrap();
        assert_eq!(str_polygon, polygon);
    }

    #[test]
    fn test_multi_polygon_geometry() {
        let multi_polygon = MultiPolygonGeometry {
            _type: "MultiPolygon".into(),
            coordinates: vec![vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_polygon,
            MultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                coordinates: vec![vec![vec![(0.0, 0.0), (1.0, 1.0), (0.0, 1.0)]]],
                m_values: None,
                bbox: None
            }
        );
        let multi_polygon_str = serde_json::to_string(&multi_polygon).unwrap();
        assert_eq!(
            multi_polygon_str,
            "{\"type\":\"MultiPolygon\",\"coordinates\":[[[[0.0,0.0],[1.0,1.0],[0.0,1.0]]]]}"
        );
        let str_multi_polygon: MultiPolygonGeometry =
            serde_json::from_str(&multi_polygon_str).unwrap();
        assert_eq!(str_multi_polygon, multi_polygon);
    }

    #[test]
    fn test_multi_polygon3d_geometry() {
        let multi_polygon = MultiPolygon3DGeometry {
            _type: "MultiPolygon3D".into(),
            coordinates: vec![vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]]],
            m_values: None,
            bbox: None,
        };
        assert_eq!(
            multi_polygon,
            MultiPolygon3DGeometry {
                _type: "MultiPolygon3D".into(),
                coordinates: vec![vec![vec![(0.0, 0.0, 0.0), (1.0, 1.0, 1.0), (0.0, 1.0, 1.0)]]],
                m_values: None,
                bbox: None
            }
        );
        let multi_polygon_str = serde_json::to_string(&multi_polygon).unwrap();
        assert_eq!(
            multi_polygon_str,
            "{\"type\":\"MultiPolygon3D\",\"coordinates\":[[[[0.0,0.0,0.0],[1.0,1.0,1.0],[0.0,1.0,\
             1.0]]]]}"
        );
        let str_multi_polygon: MultiPolygon3DGeometry =
            serde_json::from_str(&multi_polygon_str).unwrap();
        assert_eq!(str_multi_polygon, multi_polygon);
    }

    #[test]
    fn test_vector_geometry_default() {
        let default = VectorGeometry::default();
        assert_eq!(default, VectorGeometry::Point(VectorPointGeometry::default()));

        let default_instance: VectorGeometry = Default::default();
        assert_eq!(default, default_instance);
    }

    #[test]
    fn test_vector_geometry_type() {
        let vgt_point: VectorGeometryType = "Point".into();
        assert_eq!(vgt_point, VectorGeometryType::Point);
        let vgt_line_string: VectorGeometryType = "LineString".into();
        assert_eq!(vgt_line_string, VectorGeometryType::LineString);
        let vgt_polygon: VectorGeometryType = "Polygon".into();
        assert_eq!(vgt_polygon, VectorGeometryType::Polygon);
        let vgt_multi_point: VectorGeometryType = "MultiPoint".into();
        assert_eq!(vgt_multi_point, VectorGeometryType::MultiPoint);
        let vgt_multi_line_string: VectorGeometryType = "MultiLineString".into();
        assert_eq!(vgt_multi_line_string, VectorGeometryType::MultiLineString);
        let vgt_multi_polygon: VectorGeometryType = "MultiPolygon".into();
        assert_eq!(vgt_multi_polygon, VectorGeometryType::MultiPolygon);

        let default = VectorGeometryType::default();
        assert_eq!(default, VectorGeometryType::Point);

        let default_instance: VectorGeometryType = Default::default();
        assert_eq!(default, default_instance);
    }

    #[test]
    #[should_panic(expected = "Invalid vector geometry type")]
    fn test_invalid_vector_geometry_type() {
        // This should panic when an invalid string is passed
        let _ = VectorGeometryType::from("Pant");
    }

    #[test]
    fn test_vector_geometry_bbox() {
        let vgt_point: VectorGeometry = VectorGeometry::Point(VectorPointGeometry {
            _type: "Point".into(),
            coordinates: VectorPoint { x: 0.0, y: 1.0, z: Some(2.0), m: None, t: None },
            bbox: None,
            is_3d: true,
            offset: None,
            vec_bbox: Some(BBox3D {
                left: 0.0,
                bottom: 1.0,
                right: 0.0,
                top: 1.0,
                near: 2.0,
                far: 2.0,
            }),
            indices: None,
            tesselation: None,
        });
        assert_eq!(vgt_point.vec_bbox().unwrap(), BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0));
        let vgt_multi_point: VectorGeometry =
            VectorGeometry::MultiPoint(VectorMultiPointGeometry {
                _type: "MultiPoint".into(),
                coordinates: vec![VectorPoint { x: 0.0, y: 1.0, z: Some(2.0), m: None, t: None }],
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: Some(BBox3D {
                    left: 0.0,
                    bottom: 1.0,
                    right: 0.0,
                    top: 1.0,
                    near: 2.0,
                    far: 2.0,
                }),
                indices: None,
                tesselation: None,
            });
        assert_eq!(vgt_multi_point.vec_bbox().unwrap(), BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0));
        let vgt_line_string: VectorGeometry =
            VectorGeometry::LineString(VectorLineStringGeometry {
                _type: "LineString".into(),
                coordinates: vec![VectorPoint { x: 0.0, y: 1.0, z: Some(2.0), m: None, t: None }],
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: Some(BBox3D {
                    left: 0.0,
                    bottom: 1.0,
                    right: 0.0,
                    top: 1.0,
                    near: 2.0,
                    far: 2.0,
                }),
                indices: None,
                tesselation: None,
            });
        assert_eq!(vgt_line_string.vec_bbox().unwrap(), BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0));
        let vgt_multi_line_string: VectorGeometry =
            VectorGeometry::MultiLineString(VectorMultiLineStringGeometry {
                _type: "MultiLineString".into(),
                coordinates: vec![vec![VectorPoint {
                    x: 0.0,
                    y: 1.0,
                    z: Some(2.0),
                    m: None,
                    t: None,
                }]],
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: Some(BBox3D {
                    left: 0.0,
                    bottom: 1.0,
                    right: 0.0,
                    top: 1.0,
                    near: 2.0,
                    far: 2.0,
                }),
                indices: None,
                tesselation: None,
            });
        assert_eq!(
            vgt_multi_line_string.vec_bbox().unwrap(),
            BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0)
        );
        let vgt_polygon: VectorGeometry = VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: "Polygon".into(),
            coordinates: vec![vec![VectorPoint { x: 0.0, y: 1.0, z: Some(2.0), m: None, t: None }]],
            bbox: None,
            is_3d: true,
            offset: None,
            vec_bbox: Some(BBox3D {
                left: 0.0,
                bottom: 1.0,
                right: 0.0,
                top: 1.0,
                near: 2.0,
                far: 2.0,
            }),
            indices: None,
            tesselation: None,
        });
        assert_eq!(vgt_polygon.vec_bbox().unwrap(), BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0));
        let vgt_multi_polygon: VectorGeometry =
            VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
                _type: "MultiPolygon".into(),
                coordinates: vec![vec![vec![VectorPoint {
                    x: 0.0,
                    y: 1.0,
                    z: Some(2.0),
                    m: None,
                    t: None,
                }]]],
                bbox: None,
                is_3d: true,
                offset: None,
                vec_bbox: Some(BBox3D {
                    left: 0.0,
                    bottom: 1.0,
                    right: 0.0,
                    top: 1.0,
                    near: 2.0,
                    far: 2.0,
                }),
                indices: None,
                tesselation: None,
            });
        assert_eq!(
            vgt_multi_polygon.vec_bbox().unwrap(),
            BBox3D::new(0.0, 1.0, 0.0, 1.0, 2.0, 2.0)
        );
    }
}
