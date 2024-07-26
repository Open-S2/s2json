extern crate alloc;

use serde::{Serialize, Deserialize};
use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Primitive types supported by Properties
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum PrimitiveValue {
    /// String type utf8 encoded
    String(String),
    /// unsigned 64 bit integer
    U64(u64),
    /// signed 64 bit integer
    I64(i64),
    /// floating point number
    F32(f32),
    /// double precision floating point number
    F64(f64),
    /// boolean
    Bool(bool),
    /// null
    Null,
}

/// Arrays may contain either a primitive or an object whose values are primitives
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ValuePrimitiveType {
    /// Primitive type
    Primitive(PrimitiveValue),
    /// Nested shape that can only contain primitives
    NestedPrimitive(BTreeMap<String, PrimitiveValue>),
}

/// Supports primitive types `string`, `number`, `boolean`, `null`
/// May be an array of those types, or an object of those types
/// Object keys are always strings, values can be any basic type, an array, or a nested object.
/// Array values must all be the same type.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ValueType {
    /// A primitive value
    Primitive(PrimitiveValue),
    /// An array of values
    Array(Vec<ValuePrimitiveType>),
    /// A nested object
    Nested(Value),
}

/// Shape design
pub type Value = BTreeMap<String, ValueType>;
/// Shape of a features properties object
pub type Properties = Value;
/// Shape of a feature's M-Values object
pub type MValue = Value;

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
