use serde::{Deserialize, Serialize};

use alloc::vec::Vec;

/// Importing necessary types (equivalent to importing from 'values')
use crate::*;

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
#[doc(hidden)]
#[allow(unused_extern_crates, clippy::useless_attribute)]
extern crate serde as _serde;
#[automatically_derived]
impl<'de, M: MValueCompatible> _serde::Deserialize<'de> for VectorGeometry<M>
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
            <VectorPointGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            VectorGeometry::Point,
        ) {
            return _serde::__private::Ok(__ok);
        }
        // Attempt to deserialize as MultiPoint, then check for LineString
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <VectorMultiPointGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            VectorGeometry::MultiPoint,
        ) {
            // pull out the MultiPoint variant
            if let VectorGeometry::MultiPoint(multipoint) = &__ok {
                if multipoint._type == VectorGeometryType::LineString {
                    // If deserialization succeeds as MultiPoint, check if content is LineString
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <VectorLineStringGeometry<M> as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        VectorGeometry::LineString,
                    ) {
                        // If LineString is found, return LineString variant
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        // Attempt to deserialize as MultiLineString, then check for Polygon
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <VectorMultiLineStringGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            VectorGeometry::MultiLineString,
        ) {
            // pull out the MultiLineString variant
            if let VectorGeometry::MultiLineString(multilinestring) = &__ok {
                if multilinestring._type == VectorGeometryType::Polygon {
                    // If deserialization succeeds as MultiLineString, check if content is Polygon
                    if let _serde::__private::Ok(__ok2) = _serde::__private::Result::map(
                        <VectorPolygonGeometry<M> as _serde::Deserialize>::deserialize(
                            __deserializer,
                        ),
                        VectorGeometry::Polygon,
                    ) {
                        // If Polygon is found, return Polygon variant
                        return _serde::__private::Ok(__ok2);
                    }
                }
            }
            return _serde::__private::Ok(__ok);
        }
        if let _serde::__private::Ok(__ok) = _serde::__private::Result::map(
            <VectorMultiPolygonGeometry<M> as _serde::Deserialize>::deserialize(__deserializer),
            VectorGeometry::MultiPolygon,
        ) {
            return _serde::__private::Ok(__ok);
        }
        _serde::__private::Err(_serde::de::Error::custom(
            "data did not match any variant of untagged enum VectorGeometry",
        ))
    }
}
// impl<'de, M> Deserialize<'de> for VectorGeometry<M>
// where
//     M: MValueCompatible + Deserialize<'de>, // Ensure that M is deserializable
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         println!("BEGIN");
//         // First, try to deserialize each variant directly
//         // let base_geometry: VectorBaseGeometry = Deserialize::deserialize(deserializer)?;
//         let __content = <serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
//         println!("AAAAAAA");
//         let __deserializer =
//             serde::__private::de::ContentRefDeserializer::<D::Error>::new(&__content);
//         println!("BBBBBBB");
//         let base_geometry: VectorBaseGeometry = Deserialize::deserialize(__deserializer)?;
//         println!("base_geometry: {:#?}", base_geometry);

//         match base_geometry._type {
//             VectorGeometryType::Point => {
//                 let point: VectorPointGeometry<M> = Deserialize::deserialize(__deserializer)?;
//                 Ok(VectorGeometry::Point(point))
//             }
//             VectorGeometryType::MultiPoint => {
//                 let multipoint: VectorMultiPointGeometry<M> =
//                     Deserialize::deserialize(__deserializer)?;
//                 Ok(VectorGeometry::MultiPoint(multipoint))
//             }
//             VectorGeometryType::LineString => {
//                 // let linestring: VectorLineStringGeometry<M> =
//                 //     Deserialize::deserialize(__deserializer)?;
//                 // Ok(VectorGeometry::LineString(linestring))
//                 if let serde::__private::Ok(__ok) = serde::__private::Result::map(
//                     <VectorLineStringGeometry<M> as serde::Deserialize>::deserialize(
//                         __deserializer,
//                     ),
//                     VectorGeometry::LineString,
//                 ) {
//                     serde::__private::Ok(__ok)
//                 } else {
//                     serde::__private::Err(serde::de::Error::custom(
//                         "data did not match any variant of untagged enum VectorGeometry",
//                     ))
//                 }
//             }
//             VectorGeometryType::MultiLineString => {
//                 let multiline: VectorMultiLineStringGeometry<M> =
//                     Deserialize::deserialize(__deserializer)?;
//                 Ok(VectorGeometry::MultiLineString(multiline))
//             }
//             VectorGeometryType::Polygon => {
//                 let polygon: VectorPolygonGeometry<M> = Deserialize::deserialize(__deserializer)?;
//                 Ok(VectorGeometry::Polygon(polygon))
//             }
//             VectorGeometryType::MultiPolygon => {
//                 let multipolygon: VectorMultiPolygonGeometry<M> =
//                     Deserialize::deserialize(__deserializer)?;
//                 Ok(VectorGeometry::MultiPolygon(multipolygon))
//             }
//         }
//     }
// }
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
            is_3d: coordinates[0].z.is_some(),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new linestring
    pub fn new_linestring(coordinates: VectorLineString<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::LineString(VectorLineStringGeometry {
            _type: VectorGeometryType::LineString,
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
            _type: VectorGeometryType::MultiLineString,
            is_3d: coordinates[0][0].z.is_some(),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new polygon
    pub fn new_polygon(coordinates: VectorPolygon<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::Polygon(VectorPolygonGeometry {
            _type: VectorGeometryType::Polygon,
            is_3d: coordinates[0][0].z.is_some(),
            coordinates,
            bbox,
            ..Default::default()
        })
    }

    /// Create a new multipolygon
    pub fn new_multipolygon(coordinates: VectorMultiPolygon<M>, bbox: Option<BBox3D>) -> Self {
        VectorGeometry::MultiPolygon(VectorMultiPolygonGeometry {
            _type: VectorGeometryType::MultiPolygon,
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
