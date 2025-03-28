use crate::Map;
use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

//? Shape

// Shapes exist solely to deconstruct and rebuild objects.
//
// Shape limitations:
// - all keys are strings.
// - all values are either:
// - - primitive types: strings, numbers (f32, f64, u64, i64), true, false, or null
// - - sub types: an array of a shape or a nested object which is itself a shape
// - - if the sub type is an array, ensure all elements are of the same type
// The interfaces below help describe how shapes are built by the user.

/// Primitive types that can be found in a shape
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq)]
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
    #[default]
    Null,
}

/// Arrays may contain either a primitive or an object whose values are primitives
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum PrimitiveShapeType {
    /// Primitive type
    Primitive(PrimitiveShape),
    /// Nested shape that can only contain primitives
    NestedPrimitive(ShapePrimitive),
}

/// Shape types that can be found in a shapes object.
/// Either a primitive, an array containing any type, or a nested shape.
/// If the type is an array, all elements must be the same type
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum ShapeType {
    /// Primitive type
    Primitive(PrimitiveShape),
    /// Nested shape that can only contain primitives
    Array(Vec<PrimitiveShapeType>),
    /// Nested shape
    Nested(Shape),
}

/// The Primitive Shape Object
pub type ShapePrimitive = Map<String, PrimitiveShape>;
/// The Shape Object
pub type Shape = Map<String, ShapeType>;
