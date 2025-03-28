use crate::{JSONProperties, JSONValue, Map, PrimitiveValue};
use alloc::{string::String, vec::Vec};

// JSONValue
impl Default for JSONValue {
    fn default() -> Self {
        JSONValue::Primitive(PrimitiveValue::Null)
    }
}
impl JSONValue {
    /// Returns the value as a primitive
    pub fn to_prim(&self) -> Option<&PrimitiveValue> {
        match self {
            JSONValue::Primitive(v) => Some(v),
            _ => None,
        }
    }

    /// Returns the value as a vector
    pub fn to_vec(&self) -> Option<&Vec<JSONValue>> {
        match self {
            JSONValue::Array(v) => Some(v),
            _ => None,
        }
    }

    /// Returns the value as a nested object
    pub fn to_nested(&self) -> Option<&Map<String, JSONValue>> {
        match self {
            JSONValue::Object(v) => Some(v),
            _ => None,
        }
    }
}
impl From<&str> for JSONValue {
    fn from(s: &str) -> Self {
        JSONValue::Primitive(PrimitiveValue::String(s.into()))
    }
}
// impl AsRef<str> for JSONValue {
//     fn as_ref(&self) -> &str {
//         match self {
//             JSONValue::Primitive(PrimitiveValue::String(s)) => s.as_str(),
//             _ => "",
//         }
//     }
// }
impl From<String> for JSONValue {
    fn from(s: String) -> Self {
        JSONValue::Primitive(PrimitiveValue::String(s))
    }
}
impl From<&String> for JSONValue {
    fn from(s: &String) -> Self {
        JSONValue::Primitive(PrimitiveValue::String(s.into()))
    }
}
impl From<JSONValue> for String {
    fn from(v: JSONValue) -> Self {
        match v {
            JSONValue::Primitive(PrimitiveValue::String(s)) => s,
            _ => "".into(),
        }
    }
}
impl From<&JSONValue> for String {
    fn from(v: &JSONValue) -> Self {
        match v {
            JSONValue::Primitive(PrimitiveValue::String(s)) => s.into(),
            _ => "".into(),
        }
    }
}

// Implement for u8, u16, u32, u64
macro_rules! impl_from_uint {
    ($($t:ty),*) => {
        $(
            impl From<$t> for JSONValue {
                fn from(v: $t) -> Self {
                    JSONValue::Primitive(PrimitiveValue::U64(v as u64))
                }
            }

            impl From<JSONValue> for $t {
                fn from(v: JSONValue) -> Self {
                    match v {
                        JSONValue::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
                        JSONValue::Primitive(PrimitiveValue::U64(v)) => v as $t,
                        _ => 0,
                    }
                }
            }
        )*
    };
}
impl_from_uint!(u8, u16, u32, u64, usize);
macro_rules! impl_from_uint_ref {
    ($($t:ty),*) => {
        $(
            impl From<&JSONValue> for $t {
                fn from(v: &JSONValue) -> Self {
                    match v {
                        JSONValue::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
                        JSONValue::Primitive(PrimitiveValue::U64(v)) => *v as $t,
                        _ => 0,
                    }
                }
            }
        )*
    };
}
impl_from_uint_ref!(u8, u16, u32, u64, usize);
// Implement for i8, i16, i32, i64
macro_rules! impl_from_sint {
    ($($t:ty),*) => {
        $(
            impl From<$t> for JSONValue {
                fn from(v: $t) -> Self {
                    JSONValue::Primitive(PrimitiveValue::I64(v as i64))
                }
            }

            impl From<JSONValue> for $t {
                fn from(v: JSONValue) -> Self {
                    match v {
                        JSONValue::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
                        JSONValue::Primitive(PrimitiveValue::I64(v)) => v as $t,
                        _ => 0,
                    }
                }
            }
        )*
    };
}
impl_from_sint!(i8, i16, i32, i64, isize);
macro_rules! impl_from_sint_ref {
    ($($t:ty),*) => {
        $(
            impl From<&JSONValue> for $t {
                fn from(v: &JSONValue) -> Self {
                    match v {
                        JSONValue::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
                        JSONValue::Primitive(PrimitiveValue::I64(v)) => *v as $t,
                        _ => 0,
                    }
                }
            }
        )*
    };
}
impl_from_sint_ref!(i8, i16, i32, i64, isize);
impl From<f32> for JSONValue {
    fn from(v: f32) -> Self {
        JSONValue::Primitive(PrimitiveValue::F32(v))
    }
}
impl From<JSONValue> for f32 {
    fn from(v: JSONValue) -> Self {
        match v {
            JSONValue::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
            JSONValue::Primitive(PrimitiveValue::F32(v)) => v,
            _ => 0.0,
        }
    }
}
impl From<&JSONValue> for f32 {
    fn from(v: &JSONValue) -> Self {
        match v {
            JSONValue::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
            JSONValue::Primitive(PrimitiveValue::F32(v)) => *v,
            _ => 0.0,
        }
    }
}
impl From<f64> for JSONValue {
    fn from(v: f64) -> Self {
        JSONValue::Primitive(PrimitiveValue::F64(v))
    }
}
impl From<JSONValue> for f64 {
    fn from(v: JSONValue) -> Self {
        match v {
            JSONValue::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
            JSONValue::Primitive(PrimitiveValue::F64(v)) => v,
            _ => 0.0,
        }
    }
}
impl From<&JSONValue> for f64 {
    fn from(v: &JSONValue) -> Self {
        match v {
            JSONValue::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
            JSONValue::Primitive(PrimitiveValue::F64(v)) => *v,
            _ => 0.0,
        }
    }
}
impl From<bool> for JSONValue {
    fn from(v: bool) -> Self {
        JSONValue::Primitive(PrimitiveValue::Bool(v))
    }
}
impl From<JSONValue> for bool {
    fn from(v: JSONValue) -> Self {
        match v {
            JSONValue::Primitive(PrimitiveValue::String(v)) => v == "true",
            JSONValue::Primitive(PrimitiveValue::Bool(v)) => v,
            _ => false,
        }
    }
}
impl From<&JSONValue> for bool {
    fn from(v: &JSONValue) -> Self {
        match v {
            JSONValue::Primitive(PrimitiveValue::String(v)) => v == "true",
            JSONValue::Primitive(PrimitiveValue::Bool(v)) => *v,
            _ => false,
        }
    }
}
impl From<()> for JSONValue {
    fn from(_: ()) -> Self {
        JSONValue::Primitive(PrimitiveValue::Null)
    }
}
impl From<JSONValue> for () {
    fn from(_: JSONValue) -> Self {}
}
impl From<&JSONValue> for () {
    fn from(_: &JSONValue) -> Self {}
}
impl<T> From<Vec<T>> for JSONValue
where
    T: Into<JSONValue>,
{
    fn from(v: Vec<T>) -> Self {
        JSONValue::Array(v.into_iter().map(Into::into).collect())
    }
}
impl<T> From<&Vec<T>> for JSONValue
where
    T: Into<JSONValue>,
    JSONValue: for<'a> From<&'a T>,
{
    fn from(v: &Vec<T>) -> Self {
        JSONValue::Array(v.iter().map(Into::into).collect())
    }
}
impl<T> From<JSONValue> for Vec<T>
where
    T: From<JSONValue>,
{
    fn from(v: JSONValue) -> Self {
        match v {
            JSONValue::Array(v) => v.into_iter().map(Into::into).collect(),
            _ => Vec::new(),
        }
    }
}
impl<T> From<&JSONValue> for Vec<T>
where
    T: for<'a> From<&'a JSONValue>,
{
    fn from(v: &JSONValue) -> Self {
        match v {
            JSONValue::Array(v) => v.iter().map(Into::into).collect(),
            _ => Vec::new(),
        }
    }
}
impl<T> From<Option<T>> for JSONValue
where
    T: Into<JSONValue>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => v.into(),
            None => JSONValue::Primitive(PrimitiveValue::Null),
        }
    }
}
impl From<&JSONValue> for JSONValue {
    fn from(v: &JSONValue) -> Self {
        v.clone()
    }
}
impl From<JSONValue> for JSONProperties {
    fn from(v: JSONValue) -> Self {
        match v {
            JSONValue::Object(v) => v.clone(),
            _ => Self::new(),
        }
    }
}
impl From<&JSONValue> for JSONProperties {
    fn from(v: &JSONValue) -> Self {
        match v {
            JSONValue::Object(v) => v.clone(),
            _ => Self::new(),
        }
    }
}
impl From<JSONProperties> for JSONValue {
    fn from(v: JSONProperties) -> Self {
        JSONValue::Object(v)
    }
}
impl From<&JSONProperties> for JSONValue {
    fn from(v: &JSONProperties) -> Self {
        JSONValue::Object(v.clone())
    }
}
