#![no_std]
#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # S2 JSON 🌎 🗺️
//!
//! ```text                                     
//!                 ________              
//!                / ___/__ \             
//!                \__ \__/ /             
//!               ___/ / __/              
//!              /____/____/              
//!                                       
//!             _______ ____  _   __      
//!            / / ___// __ \/ | / /      
//!       __  / /\__ \/ / / /  |/ /       
//!      / /_/ /___/ / /_/ / /|  /        
//!      \____//____/\____/_/ |_/         
//! ```
//!
//! ## Install
//!
//! ```bash
//! cargo add s2json
//! ```
//!
//! ## About
//!
//! A collection of geospatial base types primarily designed for WGS84, Web Mercator, and S2.
//!
//! This library is designed to be **lightweight** with minimal dependencies. All code is `no_std`
//! complient, denies unsafe code, and can be used in any environment.
//!
//! ## Usage
//!
//! The Documentation is very thorough in this library. Therefore the best thing to do is follow
//! the links provided as needed.
//!
//! This library wraps both a GeoJSON spec and the newer S2 JSON spec. For tooling that builds ontop
//! of this library, see gis-tools.
//!
//! ### Core
//!
//! #### Top Level Topology
//!
//! - [`crate::FeatureCollection`]: WG FeatureCollection following the GeoJSON spec
//! - [`crate::S2FeatureCollection`]: S2 FeatureCollection following the S2JSON spec
//! - [`crate::Feature`]: WG Feature following the GeoJSON spec
//! - [`crate::VectorFeature`]: S2 Feature following both the S2JSON spec
//! - [`crate::Attributions`]: A list of attributions. Used by [`crate::FeatureCollection`] and [`crate::S2FeatureCollection`]
//! - [`crate::FeatureCollections`]: An enum of either [`crate::FeatureCollection`] or [`crate::S2FeatureCollection`]
//! - [`crate::Features`]: An enum of either [`crate::Feature`] or [`crate::VectorFeature`]
//! - [`crate::JSONCollection`]: A JSON FeatureCollection can be any of [`crate::FeatureCollection`], [`crate::S2FeatureCollection`], [`crate::Feature`], or [`crate::VectorFeature`]
//!
//! #### Bounding Boxes
//!
//! - [`crate::BBox`]: A BBOX is defined in lon-lat space and helps with zooming motion to see the entire line or polygon The order is (left, bottom, right, top) If WM, then the projection is lon-lat If S2, then the projection is s-t
//! - [`crate::BBox3D`]: A BBOX is defined in lon-lat space and helps with zooming motion to see the entire 3D line or polygon
//! - [`crate::BBOX`]: BBox or BBox3D enum
//!
//! #### Primitive Types
//!
//! Note that [`crate::MValue`] is the key type used by the rest of the library, gis-tools, s2tiles, s2maps, etc.
//! Allows a lot of cool encoding/decoding mechanics along with better rendering tools.
//!
//! - [`crate::Axis`]: The axis to apply an operation to
//! - [`crate::Projection`]: Enum for the WG or S2 Projections
//! - [`crate::Face`]: Cube-face on the S2 sphere. Optionss are [0, 5] inclusive.
//! - [`crate::PrimitiveValue`]:
//! - [`crate::ValuePrimitiveType`]:
//! - [`crate::ValueType`]:
//! - [`crate::ValuePrimitive`] / [`crate::MapboxProperties`]:
//! - [`crate::Value`] / [`crate::Properties`] / [`crate::MValue`]:
//! - [`crate::MValues`]:
//! - [`crate::JSONValue`]:
//! - [`crate::JSONProperties`]:
//!
//! #### Geoemtry
//!
//! - [`crate::STPoint`]: An Point in S2 Space with a Face
//!
//! **Primitives**
//!
//! - [`crate::Point`]: A very basic cheap X-Y point
//! - [`crate::MultiPoint`]: An array of [`crate::Point`]
//! - [`crate::LineString`]: An array of [`crate::Point`]
//! - [`crate::MultiLineString`]: An array of [`crate::LineString`]
//! - [`crate::Polygon`]: An array of [`crate::LineString`]
//! - [`crate::MultiPolygon`]: An array of [`crate::Polygon`]
//! - [`crate::Point3D`]: A very basic 3D X-Y-Z point
//! - [`crate::MultiPoint3D`]: An array of [`crate::Point3D`]
//! - [`crate::LineString3D`]: An array of [`crate::Point3D`]
//! - [`crate::MultiLineString3D`]: An array of [`crate::LineString3D`]
//! - [`crate::Polygon3D`]: An array of [`crate::LineString3D`]
//! - [`crate::MultiPolygon3D`]: An array of [`crate::Polygon3D`]
//! - [`crate::PointOrPoint3D`]: Either a [`crate::Point`] or [`crate::Point3D`]
//! - [`crate::GeometryType`]: The type of geometry
//! - [`crate::Geometry`]: The geometry. An enum of all primtive geometry types
//!
//! **Vector Primitives**
//!
//! - [`crate::VectorPoint`]: A Vector Point uses a structure for both 2D or 3D points. Useful for geometry that also has m-values
//! - [`crate::VectorMultiPoint`]: An array of [`crate::VectorPoint`]
//! - [`crate::VectorLineString`]: An array of [`crate::VectorPoint`]
//! - [`crate::VectorMultiLineString`]: An array of [`crate::VectorLineString`]
//! - [`crate::VectorPolygon`]: An array of [`crate::VectorLineString`]
//! - [`crate::VectorMultiPolygon`]: An array of [`crate::VectorPolygon`]
//! - [`crate::VectorGeometryType`]: The type of vector geometry
//! - [`crate::VectorGeometry`]: The vector geometry. An enum of all vector geometry types
//! - [`crate::VectorOffsets`]: The offsets for a vector geometry (rarely used)
//!
//! ### Traits
//!
//! These traits are the fundamental building box for all geometry tooling. If you need your own
//! custom data structure you can implement these traits and use all the tools built not only in
//! this library but gis-tools as well.
//!
//! **Getters**
//!
//! - [`crate::GetXY`]: Get the X and Y coordinates of a point
//! - [`crate::GetZ`]: Get the Z coordinate of a point
//! - [`crate::GetXYZ`]: Get the X, Y, and Z coordinates of a point
//! - [`crate::GetM`]: Get the M coordinate of a point
//! - [`crate::GetXYZM`]: Get the X, Y, Z, and M coordinates of a point
//!
//! **Setters**
//!
//! - [`crate::SetXY`]: Set the X and Y coordinates of a point
//! - [`crate::SetZ`]: Set the Z coordinate of a point
//! - [`crate::SetXYZ`]: Set the X, Y, and Z coordinates of a point
//! - [`crate::SetM`]: Set the M coordinate of a point
//! - [`crate::SetXYZM`]: Set the X, Y, Z, and M coordinates of a point
//!
//! **New**
//!
//! - [`crate::NewXY`]: Get and Set the X and Y coordinates of a point
//! - [`crate::NewXYM`]: Get and Set the X, Y, and M coordinates of a point
//! - [`crate::NewXYZ`]: Get and Set the X, Y, and Z coordinates of a point
//! - [`crate::NewXYZM`]: Get and Set the X, Y, Z, and M coordinates of a point
//!
//! **Full Get, New, and Set**
//!
//! - [`crate::FullXY`]: Get, New, and Set the X and Y coordinates of a point
//! - [`crate::FullXYM`]: Get, New, and Set the X, Y, and M coordinates of a point
//! - [`crate::FullXYZ`]: Get, New, and Set the X, Y, and Z coordinates of a point
//! - [`crate::FullXYZM`]: Get, New, and Set the X, Y, Z, and M coordinates of a point
//!
//! **Utilities**
//!
//! You probably won't have to implement these. Created for convenience.
//!
//! - [`crate::MValueCompatible`]: Descriptor of all the traits that make using M-Value tools simple
//! - [`crate::Bounded`]: Used by [`BBox`] and [`BBox3D`]. Defines a min and max value.
//! - [`crate::Interpolate`]: Easy access to interpolation tooling for All S2JSON Core Types
//!
//! ### Derives
//!
//! - [`crate::MValueCompatible`]: Ensure M implements All MValue Traits used by VectorPoints
//! - [`crate::JSONPropertiesCompatible`]: Ensure M implements All MValue Traits used by complex GeoJSON properties. Not used by gis-tools.
//! - [`crate::Properties`]: Ensure M implements All MValue Traits used by VectorPoints
//! - [`crate::MValue`]: Ensure M implements All MValue Traits used by VectorPoints
//! - [`crate::ValuePrimitive`]: Sub type used by [`crate::MValue`]
//! - [`crate::JSONProperties`]: JSON Properties specification to-from mechanics

extern crate s2json_core;
#[cfg(feature = "derive")]
extern crate s2json_derive;

pub use s2json_core::*;
#[cfg(feature = "derive")]
pub use s2json_derive::*;
