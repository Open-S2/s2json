extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;

/// Primitive types supported by Properties
#[derive(Debug, PartialEq)]
pub enum Primitive {
    /// String type utf8 encoded
    String(String),
    /// unsigned 64 bit integer
    Unsigned(u64),
    /// signed 64 bit integer
    Signed(i64),
    /// floating point number
    Float(f32),
    /// double precision floating point number
    Double(f64),
    /// boolean
    Boolean(bool),
    /// null
    Null,
}

/// When an array is used, it must be an array of the same type.
/// Arrays are also limited to primitives and objects of primitives
#[derive(Debug, PartialEq)]
pub enum ValueArray {
    /// Array of primitives
    Primitives(Vec<Primitive>),
    /// Array of objects that may only contain primitives
    Objects(Vec<BTreeMap<String, Primitive>>),
}

/// Supports primitive types `string`, `number`, `boolean`, `null`
/// May be an array of those types, or an object of those types
/// Object keys are always strings, values can be any basic type, an array, or a nested object.
/// Array values must all be the same type.
#[derive(Debug, PartialEq)]
pub enum Value {
    /// A primitive value
    Primitive(Primitive),
    /// An array of values
    Array(ValueArray),
    /// A nested object
    Object(BTreeMap<String, Value>),
}

/// Shape of a features properties object
pub type Properties = BTreeMap<String, Value>;
/// Shape of a feature's M-Values object
pub type MValue = Properties;

/// LineString Properties Shape
pub type LineStringMValues = Vec<MValue>;
/// MultiLineString MValues Shape
pub type MultiLineStringMValues = Vec<LineStringMValues>;
/// Polygon MValues Shape
pub type PolygonMValues = Vec<LineStringMValues>;
/// MultiPolygon MValues Shape
pub type MultiPolygonMValues = Vec<PolygonMValues>;

/// All possible M-Value shapes
#[derive(Debug, PartialEq)]
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
