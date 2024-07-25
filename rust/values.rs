extern crate alloc;

use serde::{Serialize, Deserialize};
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Primitive types supported by Properties
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrimitiveShape {
    /// String type utf8 encoded
    String,
    /// unsigned 64 bit integer
    U64,
    /// signed 64 bit integer
    I64,
    /// floating point number
    F32,
    /// double precision floating point number
    F64,
    /// boolean
    Bool,
    /// null
    Null,
}

/// Arrays may contain either a primitive or an object whose values are primitives
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShapePrimitiveType {
    /// Primitive type
    Primitive(PrimitiveShape),
    /// Nested shape that can only contain primitives
    NestedPrimitive(BTreeMap<String, PrimitiveShape>),
}

/// Supports primitive types `string`, `number`, `boolean`, `null`
/// May be an array of those types, or an object of those types
/// Object keys are always strings, values can be any basic type, an array, or a nested object.
/// Array values must all be the same type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShapeType {
    /// A primitive value
    Primitive(PrimitiveShape),
    /// An array of values
    Array(Vec<ShapePrimitiveType>),
    /// A nested object
    Nested(Shape),
}

/// Shape design
pub type Shape = BTreeMap<String, ShapeType>;
/// Shape of a features properties object
pub type Properties = Shape;
/// Shape of a feature's M-Values object
pub type MValue = Shape;

/// LineString Properties Shape
pub type LineStringMValues = Vec<MValue>;
/// MultiLineString MValues Shape
pub type MultiLineStringMValues = Vec<LineStringMValues>;
/// Polygon MValues Shape
pub type PolygonMValues = Vec<LineStringMValues>;
/// MultiPolygon MValues Shape
pub type MultiPolygonMValues = Vec<PolygonMValues>;

/// All possible M-Value shapes
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MValues {
    /// Single M-Value
    MValue(MValue),
    /// LineString M-Value
    LineStringMValues(LineStringMValues),
    /// MultiLineString M-Value
    MultiLineStringMValues(MultiLineStringMValues),
    /// Polygon M-Value
    PolygonMValues(PolygonMValues),
    /// MultiPolygon M-Value
    MultiPolygonMValues(MultiPolygonMValues),
}
