use crate::*;
use alloc::fmt;
use core::{
    cmp::Ordering,
    ops::{Mul, MulAssign, Sub},
};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{self, SeqAccess, Visitor},
    ser::SerializeTuple,
};

/// Trait for types that have a min and max value. Used by [`BBox`] and [`BBox3D`]
pub trait Bounded {
    /// Get the minimum value
    fn min_value() -> Self;
    /// Get the maximum value
    fn max_value() -> Self;
}
macro_rules! impl_bounded {
    ($($t:ty),*) => {
        $(
            impl Bounded for $t {
                fn min_value() -> Self {
                    <$t>::MIN
                }
                fn max_value() -> Self {
                    <$t>::MAX
                }
            }
        )*
    };
}

// Implement for common numeric types
impl_bounded!(i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize, f32, f64);

/// # Bounding Box
///
/// ## Description
///
/// A Bounding Box ensures a min-max range of values
///
/// The order is (left, bottom, right, top) when storing as a flattened array.
///
/// If WM, then the projection is lon-lat. If S2, then the projection is s-t
///
/// Defaults to f64 as the base type.
///
/// Implements [`MValueCompatible`]
///
/// ## Usage
///
/// ## From/To
/// - [`MValue`]
/// - [`ValueType`]
///
/// ### Any Type BBox
/// - [`BBox::new`]: Creates a new BBox
/// - [`BBox::point_overlap`]: Checks if a point is within the BBox
/// - [`BBox::merge`]: Merges another bounding box with this one
/// - [`BBox::merge_in_place`]: Merges in place another bounding box with this one
/// - [`BBox::overlap`]: Checks if another bounding box overlaps with this one
/// - [`BBox::clip`]: Clips the bounding box along an axis
/// - [`BBox::inside`]: Checks if this bounding box is inside another
/// - [`BBox::area`]: Returns the area of the bounding box
///
/// ### `f64` Type BBox
/// Note that all the input geometry uses the [`GetXY`] trait.
/// - [`BBox::from_point`]: Creates a new BBox from a point
/// - [`BBox::from_linestring`]: Creates a new BBox from a linestring
/// - [`BBox::from_multi_linestring`]: Creates a new BBox from a multi-linestring
/// - [`BBox::from_polygon`]: Creates a new BBox from a polygon
/// - [`BBox::from_multi_polygon`]: Creates a new BBox from a multi-polygon
/// - [`BBox::extend_from_point`]: Extends the bounding box with a point
/// - [`BBox::from_uv_zoom`]: Creates a new BBox from zoom-uv coordinates
/// - [`BBox::from_st_zoom`]: Creates a new BBox from zoom-st coordinates
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
impl From<(f64, f64, f64, f64)> for BBox<f64> {
    fn from(bbox: (f64, f64, f64, f64)) -> Self {
        BBox { left: bbox.0, bottom: bbox.1, right: bbox.2, top: bbox.3 }
    }
}
impl From<BBox<f64>> for (f64, f64, f64, f64) {
    fn from(bbox: BBox<f64>) -> Self {
        (bbox.left, bbox.bottom, bbox.right, bbox.top)
    }
}
impl<T> From<BBox<T>> for MValue
where
    T: Into<ValueType>,
{
    fn from(bbox: BBox<T>) -> MValue {
        MValue::from([
            ("left".into(), bbox.left.into()),
            ("bottom".into(), bbox.bottom.into()),
            ("right".into(), bbox.right.into()),
            ("top".into(), bbox.top.into()),
        ])
    }
}
impl<T> From<&BBox<T>> for MValue
where
    T: Copy + Into<ValueType>,
{
    fn from(bbox: &BBox<T>) -> MValue {
        MValue::from([
            ("left".into(), bbox.left.into()),
            ("bottom".into(), bbox.bottom.into()),
            ("right".into(), bbox.right.into()),
            ("top".into(), bbox.top.into()),
        ])
    }
}
impl<T> From<MValue> for BBox<T>
where
    T: From<ValueType>,
{
    fn from(mvalue: MValue) -> Self {
        BBox {
            left: mvalue.get("left").unwrap().clone().into(),
            bottom: mvalue.get("bottom").unwrap().clone().into(),
            right: mvalue.get("right").unwrap().clone().into(),
            top: mvalue.get("top").unwrap().clone().into(),
        }
    }
}
impl<T> From<&MValue> for BBox<T>
where
    T: From<ValueType>,
{
    fn from(mvalue: &MValue) -> Self {
        BBox {
            left: mvalue.get("left").unwrap().clone().into(),
            bottom: mvalue.get("bottom").unwrap().clone().into(),
            right: mvalue.get("right").unwrap().clone().into(),
            top: mvalue.get("top").unwrap().clone().into(),
        }
    }
}
impl<T> MValueCompatible for BBox<T>
where
    ValueType: From<T>,
    T: From<ValueType> + Default + Bounded + Copy + Interpolate,
{
}
impl<T> Default for BBox<T>
where
    T: Default + Bounded + Copy,
{
    fn default() -> Self {
        BBox::new(T::max_value(), T::max_value(), T::min_value(), T::min_value())
    }
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
    pub fn point_overlap<P: GetXY>(&self, point: &P) -> bool
    where
        T: Into<f64> + Copy, // Ensures that comparison operators work for type T
    {
        point.x() >= self.left.into()
            && point.x() <= self.right.into()
            && point.y() >= self.bottom.into()
            && point.y() <= self.top.into()
    }

    /// Merges another bounding box with this one
    pub fn merge(&self, other: &Self) -> Self
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

    /// Merges in place another bounding box with this one
    pub fn merge_in_place(&mut self, other: &Self)
    where
        T: PartialOrd + Copy,
    {
        self.left = if self.left < other.left { self.left } else { other.left };
        self.bottom = if self.bottom < other.bottom { self.bottom } else { other.bottom };
        self.right = if self.right > other.right { self.right } else { other.right };
        self.top = if self.top > other.top { self.top } else { other.top };
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

    /// Checks if this bounding box is inside another
    pub fn inside(&self, other: &BBox<T>) -> bool
    where
        T: PartialOrd + Copy,
    {
        self.left >= other.left
            && self.right <= other.right
            && self.bottom >= other.bottom
            && self.top <= other.top
    }

    /// Returns the area of the bounding box
    pub fn area(&self) -> T
    where
        T: Sub<Output = T> + Mul<Output = T> + Copy,
    {
        (self.right - self.left) * (self.top - self.bottom)
    }
}
impl BBox<f64> {
    /// Creates a new BBox from a point
    pub fn from_point<P: GetXY>(point: &P) -> Self {
        BBox::new(point.x(), point.y(), point.x(), point.y())
    }

    /// Creates a new BBox from a linestring
    pub fn from_linestring<P: GetXY>(line: &[P]) -> Self {
        let mut bbox = BBox::default();
        for point in line {
            bbox.extend_from_point(point);
        }
        bbox
    }

    /// Creates a new BBox from a multi-linestring
    pub fn from_multi_linestring<P: GetXY>(lines: &[Vec<P>]) -> Self {
        let mut bbox = BBox::default();
        for line in lines {
            for point in line {
                bbox.extend_from_point(point);
            }
        }
        bbox
    }

    /// Creates a new BBox from a polygon
    pub fn from_polygon<P: GetXY>(polygon: &[Vec<P>]) -> Self {
        Self::from_multi_linestring(polygon)
    }

    /// Creates a new BBox from a multi-polygon
    pub fn from_multi_polygon<P: GetXY>(polygons: &[Vec<Vec<P>>]) -> Self {
        let mut bbox = BBox::default();
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
    pub fn extend_from_point<P: GetXY>(&mut self, point: &P) {
        self.merge_in_place(&BBox::from_point(point));
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
impl<T: Copy> From<BBox3D<T>> for BBox<T> {
    fn from(bbox: BBox3D<T>) -> Self {
        BBox::new(bbox.left, bbox.bottom, bbox.right, bbox.top)
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

/// # 3D Bounding Box
///
/// ## Description
///
/// A 3D Bounding Box ensures a min-max range of values and includes a near-far range
///
/// The order is (left, bottom, right, top, near, far) when storing as a flattened array.
///
/// If WM, then the projection is lon-lat. If S2, then the projection is s-t
///
/// Defaults to f64 as the base type.
///
/// Implements [`MValueCompatible`] as well as [`Interpolate`].
///
/// ## Usage
///
/// ## From/To
/// - [`MValue`]
/// - [`ValueType`]
///
/// ### Any Type BBox
/// - [`BBox3D::new`]: Creates a new BBox
/// - [`BBox3D::point_overlap`]: Checks if a point is within the BBox
/// - [`BBox3D::merge`]: Merges another bounding box with this one
/// - [`BBox3D::merge_in_place`]: Merges in place another bounding box with this one
/// - [`BBox3D::overlap`]: Checks if another bounding box overlaps with this one
/// - [`BBox3D::clip`]: Clips the bounding box along an axis
/// - [`BBox3D::inside`]: Checks if this bounding box is inside another
/// - [`BBox3D::from_bbox`]: Creates a new BBox3D from a BBox
/// - [`BBox3D::area`]: Returns the area of the bounding box
///
/// ### `f64` Type BBox
/// Note that all the input geometry uses the [`GetXY`] and [`GetZ`] traits.
/// - [`BBox3D::from_point`]: Creates a new BBox from a point
/// - [`BBox3D::from_linestring`]: Creates a new BBox from a linestring
/// - [`BBox3D::from_multi_linestring`]: Creates a new BBox from a multi-linestring
/// - [`BBox3D::from_polygon`]: Creates a new BBox from a polygon
/// - [`BBox3D::from_multi_polygon`]: Creates a new BBox from a multi-polygon
/// - [`BBox3D::extend_from_point`]: Extends the bounding box with a point
/// - [`BBox3D::from_uv_zoom`]: Creates a new BBox from zoom-uv coordinates
/// - [`BBox3D::from_st_zoom`]: Creates a new BBox from zoom-st coordinates
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
impl From<(f64, f64, f64, f64, f64, f64)> for BBox3D<f64> {
    fn from(bbox: (f64, f64, f64, f64, f64, f64)) -> Self {
        BBox3D {
            left: bbox.0,
            bottom: bbox.1,
            right: bbox.2,
            top: bbox.3,
            near: bbox.4,
            far: bbox.5,
        }
    }
}
impl From<BBox3D<f64>> for (f64, f64, f64, f64, f64, f64) {
    fn from(bbox: BBox3D<f64>) -> Self {
        (bbox.left, bbox.bottom, bbox.right, bbox.top, bbox.near, bbox.far)
    }
}
impl<T> From<BBox3D<T>> for MValue
where
    T: Into<ValueType>,
{
    fn from(bbox: BBox3D<T>) -> MValue {
        MValue::from([
            ("left".into(), bbox.left.into()),
            ("bottom".into(), bbox.bottom.into()),
            ("right".into(), bbox.right.into()),
            ("top".into(), bbox.top.into()),
            ("near".into(), bbox.near.into()),
            ("far".into(), bbox.far.into()),
        ])
    }
}
impl<T> From<&BBox3D<T>> for MValue
where
    T: Copy + Into<ValueType>,
{
    fn from(bbox: &BBox3D<T>) -> MValue {
        MValue::from([
            ("left".into(), bbox.left.into()),
            ("bottom".into(), bbox.bottom.into()),
            ("right".into(), bbox.right.into()),
            ("top".into(), bbox.top.into()),
            ("near".into(), bbox.near.into()),
            ("far".into(), bbox.far.into()),
        ])
    }
}
impl<T> From<MValue> for BBox3D<T>
where
    T: From<ValueType>,
{
    fn from(mvalue: MValue) -> Self {
        BBox3D {
            left: mvalue.get("left").unwrap().clone().into(),
            bottom: mvalue.get("bottom").unwrap().clone().into(),
            right: mvalue.get("right").unwrap().clone().into(),
            top: mvalue.get("top").unwrap().clone().into(),
            near: mvalue.get("near").unwrap().clone().into(),
            far: mvalue.get("far").unwrap().clone().into(),
        }
    }
}
impl<T> From<&MValue> for BBox3D<T>
where
    T: From<ValueType>,
{
    fn from(mvalue: &MValue) -> Self {
        BBox3D {
            left: mvalue.get("left").unwrap().clone().into(),
            bottom: mvalue.get("bottom").unwrap().clone().into(),
            right: mvalue.get("right").unwrap().clone().into(),
            top: mvalue.get("top").unwrap().clone().into(),
            near: mvalue.get("near").unwrap().clone().into(),
            far: mvalue.get("far").unwrap().clone().into(),
        }
    }
}
impl<T> MValueCompatible for BBox3D<T>
where
    ValueType: From<T>,
    T: From<ValueType> + Default + Bounded + Copy + Interpolate,
{
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
    pub fn point_overlap<P: GetXY + GetZ>(&self, point: &P) -> bool
    where
        T: Into<f64> + Copy, // Ensures that comparison operators work for type T
    {
        let z = point.z().unwrap_or_default();
        point.x() >= self.left.into()
            && point.x() <= self.right.into()
            && point.y() >= self.bottom.into()
            && point.y() <= self.top.into()
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

    /// Merges in place another bounding box with this one
    pub fn merge_in_place(&mut self, other: &Self)
    where
        T: PartialOrd + Copy,
    {
        self.left = if self.left < other.left { self.left } else { other.left };
        self.bottom = if self.bottom < other.bottom { self.bottom } else { other.bottom };
        self.right = if self.right > other.right { self.right } else { other.right };
        self.top = if self.top > other.top { self.top } else { other.top };
        self.near = if self.near < other.near { self.near } else { other.near };
        self.far = if self.far > other.far { self.far } else { other.far };
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

    /// Creates a new BBox3D from a BBox
    pub fn from_bbox(bbox: &BBox<T>) -> Self
    where
        T: Copy + Default,
    {
        BBox3D::new(bbox.left, bbox.bottom, bbox.right, bbox.top, T::default(), T::default())
    }

    /// Checks if this bounding box is inside another
    pub fn inside(&self, other: &BBox3D<T>) -> bool
    where
        T: PartialOrd + Copy,
    {
        self.left >= other.left
            && self.right <= other.right
            && self.bottom >= other.bottom
            && self.top <= other.top
            && self.near >= other.near
            && self.far <= other.far
    }

    /// Returns the area of the bounding box
    pub fn area(&self) -> T
    where
        T: Sub<Output = T> + Mul<Output = T> + MulAssign + Into<f64> + Copy,
    {
        let mut res = (self.right - self.left) * (self.top - self.bottom);
        if self.far.into() != 0. || self.near.into() != 0. {
            res *= self.far - self.near
        }

        res
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
impl<T> Default for BBox3D<T>
where
    T: Default + Bounded + Copy,
{
    fn default() -> Self {
        BBox3D::new(
            T::max_value(),
            T::max_value(),
            T::min_value(),
            T::min_value(),
            T::max_value(),
            T::min_value(),
        )
    }
}
impl BBox3D<f64> {
    /// Creates a new BBox3D from a point
    pub fn from_point<P: GetXY + GetZ>(point: &P) -> Self {
        BBox3D::new(
            point.x(),
            point.y(),
            point.x(),
            point.y(),
            point.z().unwrap_or(f64::MAX),
            point.z().unwrap_or(f64::MIN),
        )
    }

    /// Creates a new BBox from a linestring
    pub fn from_linestring<P: GetXY + GetZ>(line: &[P]) -> Self {
        let mut bbox = BBox3D::default();
        for point in line {
            bbox.extend_from_point(point);
        }
        bbox
    }

    /// Creates a new BBox from a multi-linestring
    pub fn from_multi_linestring<P: GetXY + GetZ>(lines: &[Vec<P>]) -> Self {
        let mut bbox = BBox3D::default();
        for line in lines {
            for point in line {
                bbox.extend_from_point(point);
            }
        }
        bbox
    }

    /// Creates a new BBox from a polygon
    pub fn from_polygon<P: GetXY + GetZ>(polygon: &[Vec<P>]) -> Self {
        Self::from_multi_linestring(polygon)
    }

    /// Creates a new BBox from a multi-polygon
    pub fn from_multi_polygon<P: GetXY + GetZ>(polygons: &[Vec<Vec<P>>]) -> Self {
        let mut bbox = BBox3D::default();
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
    pub fn extend_from_point<P: GetXY + GetZ>(&mut self, point: &P) {
        self.merge_in_place(&BBox3D::from_point(point));
    }

    /// Creates a new BBox3D from zoom-uv coordinates
    pub fn from_uv_zoom(u: f64, v: f64, zoom: u8) -> Self {
        let division_factor = 2. / (1 << zoom) as f64;

        BBox3D {
            left: division_factor * u - 1.0,
            bottom: division_factor * v - 1.0,
            right: division_factor * (u + 1.0) - 1.0,
            top: division_factor * (v + 1.0) - 1.0,
            near: f64::MAX,
            far: f64::MIN,
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
            near: f64::MAX,
            far: f64::MIN,
        }
    }
}
impl<T: Default + Copy> From<BBox<T>> for BBox3D<T> {
    fn from(bbox: BBox<T>) -> Self {
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
impl From<BBox> for BBOX {
    fn from(bbox: BBox) -> Self {
        BBOX::BBox(bbox)
    }
}
impl From<BBox3D> for BBOX {
    fn from(bbox: BBox3D) -> Self {
        BBOX::BBox3D(bbox)
    }
}
impl Eq for BBOX {}
impl PartialOrd for BBOX {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for BBOX {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (BBOX::BBox(a), BBOX::BBox(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (BBOX::BBox3D(a), BBOX::BBox3D(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            // Ensure that BBox and BBox3D are ordered correctly
            (BBOX::BBox(_), BBOX::BBox3D(_)) => Ordering::Less,
            (BBOX::BBox3D(_), BBOX::BBox(_)) => Ordering::Greater,
        }
    }
}
