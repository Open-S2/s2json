use crate::Map;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

/// Primitive types supported by Properties
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(untagged)]
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
    #[default]
    Null,
}

/// Arrays may contain either a primitive or an object whose values are primitives
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ValuePrimitiveType {
    /// Primitive type
    Primitive(PrimitiveValue),
    /// Nested shape that can only contain primitives
    NestedPrimitive(ValuePrimitive),
}

/// Supports primitive types `string`, `number`, `boolean`, `null`
/// May be an array of those primitive types, or an object whose values are only primitives
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

/// Shape of a ValuePrimitiveType Nested object
pub type ValuePrimitive = Map<String, PrimitiveValue>;
/// Shape design
pub type Value = Map<String, ValueType>;
/// Shape of a features properties object
pub type Properties = Value;
/// Shape of a feature's M-Values object
pub type MValue = Value;

/// Ensure M implements MValueCompatible
pub trait MValueCompatible:
    for<'a> From<&'a MValue> + From<MValue> + Into<MValue> + Clone + Default
{
}
impl From<&MValue> for MValue {
    fn from(mvalue: &MValue) -> MValue {
        mvalue.clone()
    }
}
impl MValueCompatible for MValue {}

/// LineString Properties Shape
pub type LineStringMValues<M = MValue> = Vec<M>;
/// MultiLineString MValues Shape
pub type MultiLineStringMValues<M = MValue> = Vec<LineStringMValues<M>>;
/// Polygon MValues Shape
pub type PolygonMValues<M = MValue> = Vec<LineStringMValues<M>>;
/// MultiPolygon MValues Shape
pub type MultiPolygonMValues<M = MValue> = Vec<PolygonMValues<M>>;

/// All possible M-Value shapes
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum MValues<M: Clone = MValue> {
    /// Single M-Value
    MValue(M),
    /// LineString M-Value
    LineStringMValues(LineStringMValues<M>),
    /// MultiLineString M-Value
    MultiLineStringMValues(MultiLineStringMValues<M>),
    /// Polygon M-Value
    PolygonMValues(PolygonMValues<M>),
    /// MultiPolygon M-Value
    MultiPolygonMValues(MultiPolygonMValues<M>),
}

/// All possible JSON shapes
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(untagged)]
pub enum JSONValue {
    /// Represents a JSON primitive
    Primitive(PrimitiveValue),
    /// Represents a JSON array.
    Array(Vec<JSONValue>),
    /// Represents a JSON object.
    Object(JSONProperties),
}

/// Shape of an un-restricted features properties object
pub type JSONProperties = Map<String, JSONValue>;
/// Ensure M implements MValueCompatible
pub trait JSONPropertiesCompatible:
    for<'a> From<&'a JSONProperties> + From<JSONProperties> + Into<JSONProperties> + Clone + Default
{
}
impl From<&JSONProperties> for JSONProperties {
    fn from(json: &JSONProperties) -> JSONProperties {
        json.clone()
    }
}
impl JSONPropertiesCompatible for JSONProperties {}

/// Shape of the restricted Mapbox properties object
pub type MapboxProperties = ValuePrimitive;
