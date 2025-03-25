use crate::*;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use libm::round;

// PrimitiveValue
impl PrimitiveValue {
    /// Returns true if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, PrimitiveValue::Null)
    }

    /// Converts a primitive value to a string
    pub fn to_string(&self) -> Option<String> {
        match self {
            PrimitiveValue::String(v) => Some(v.clone()),
            _ => None,
        }
    }

    /// Converts a primitive value to a u64
    pub fn to_u64(&self) -> Option<u64> {
        match self {
            PrimitiveValue::U64(v) => Some(*v),
            PrimitiveValue::I64(v) => Some(*v as u64),
            PrimitiveValue::F64(v) => Some(round(*v) as u64),
            PrimitiveValue::F32(v) => Some(round((*v).into()) as u64),
            _ => None,
        }
    }

    /// Converts a primitive value to a i64
    pub fn to_i64(&self) -> Option<i64> {
        match self {
            PrimitiveValue::U64(v) => Some(*v as i64),
            PrimitiveValue::I64(v) => Some(*v),
            PrimitiveValue::F64(v) => Some(round(*v) as i64),
            PrimitiveValue::F32(v) => Some(round((*v).into()) as i64),
            _ => None,
        }
    }

    /// Converts a primitive value to a f64
    pub fn to_f64(&self) -> Option<f64> {
        match self {
            PrimitiveValue::U64(v) => Some(*v as f64),
            PrimitiveValue::I64(v) => Some(*v as f64),
            PrimitiveValue::F64(v) => Some(*v),
            PrimitiveValue::F32(v) => Some(*v as f64),
            _ => None,
        }
    }

    /// Converts a primitive value to a f32
    pub fn to_f32(&self) -> Option<f32> {
        match self {
            PrimitiveValue::U64(v) => Some(*v as f32),
            PrimitiveValue::I64(v) => Some(*v as f32),
            PrimitiveValue::F64(v) => Some(*v as f32),
            PrimitiveValue::F32(v) => Some(*v),
            _ => None,
        }
    }

    /// Converts a primitive value to a bool
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            PrimitiveValue::Bool(v) => Some(*v),
            _ => None,
        }
    }
}
impl From<&str> for PrimitiveValue {
    fn from(s: &str) -> Self {
        PrimitiveValue::String(s.to_string())
    }
}
impl From<String> for PrimitiveValue {
    fn from(s: String) -> Self {
        PrimitiveValue::String(s)
    }
}
impl From<u64> for PrimitiveValue {
    fn from(v: u64) -> Self {
        PrimitiveValue::U64(v)
    }
}
impl From<i64> for PrimitiveValue {
    fn from(v: i64) -> Self {
        PrimitiveValue::I64(v)
    }
}
impl From<f32> for PrimitiveValue {
    fn from(v: f32) -> Self {
        PrimitiveValue::F32(v)
    }
}
impl From<f64> for PrimitiveValue {
    fn from(v: f64) -> Self {
        PrimitiveValue::F64(v)
    }
}
impl From<bool> for PrimitiveValue {
    fn from(v: bool) -> Self {
        PrimitiveValue::Bool(v)
    }
}
impl From<()> for PrimitiveValue {
    fn from(_: ()) -> Self {
        PrimitiveValue::Null
    }
}
impl<T> From<Option<T>> for PrimitiveValue
where
    T: Into<PrimitiveValue>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => v.into(),
            None => PrimitiveValue::Null,
        }
    }
}
impl From<&PrimitiveValue> for JSONValue {
    fn from(v: &PrimitiveValue) -> Self {
        JSONValue::Primitive(v.clone())
    }
}
impl From<&JSONValue> for PrimitiveValue {
    fn from(v: &JSONValue) -> Self {
        match v {
            JSONValue::Primitive(v) => v.clone(),
            // DROPS VALUES THAT ARE NOT PRIMITIVES
            _ => PrimitiveValue::Null,
        }
    }
}

// ValuePrimitiveType
impl ValuePrimitiveType {
    /// Returns the value as a primitive
    pub fn to_prim(&self) -> Option<&PrimitiveValue> {
        match self {
            ValuePrimitiveType::Primitive(v) => Some(v),
            _ => None,
        }
    }

    /// Returns the value as a nested object
    pub fn to_nested(&self) -> Option<&ValuePrimitive> {
        match self {
            ValuePrimitiveType::NestedPrimitive(v) => Some(v),
            _ => None,
        }
    }
}
impl From<&str> for ValuePrimitiveType {
    fn from(s: &str) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::String(s.to_string()))
    }
}
impl From<String> for ValuePrimitiveType {
    fn from(s: String) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::String(s))
    }
}
impl From<&ValuePrimitiveType> for String {
    fn from(v: &ValuePrimitiveType) -> Self {
        match v {
            ValuePrimitiveType::Primitive(PrimitiveValue::String(s)) => s.to_string(),
            _ => "".to_string(),
        }
    }
}
// Implement for u8, u16, u32, u64
macro_rules! impl_from_uint_uint {
    ($($t:ty),*) => {
        $(
            impl From<$t> for ValuePrimitiveType {
                fn from(v: $t) -> Self {
                    ValuePrimitiveType::Primitive(PrimitiveValue::U64(v as u64))
                }
            }
        )*
    };
}
impl_from_uint_uint!(u8, u16, u32, u64, usize);
macro_rules! impl_from_uint_ref {
    ($($t:ty),*) => {
        $(
            impl<'a> From<&'a $t> for ValuePrimitiveType {
                fn from(v: &$t) -> Self {
                    ValuePrimitiveType::Primitive(PrimitiveValue::U64(*v as u64))
                }
            }
        )*
    };
}
impl_from_uint_ref!(u8, u16, u32, u64, usize);
// Implement for u8, u16, u32, u64
macro_rules! impl_from_prim_uint {
    ($($t:ty),*) => {
        $(
            impl From<ValuePrimitiveType> for $t {
                fn from(v: ValuePrimitiveType) -> Self {
                    match v {
                        ValuePrimitiveType::Primitive(PrimitiveValue::U64(v)) => v as $t,
                        _ => 0,
                    }
                }
            }
        )*
    };
}
impl_from_prim_uint!(u8, u16, u32, u64, usize);
macro_rules! impl_from_prim_ref_uint {
    ($($t:ty),*) => {
        $(
            impl From<&ValuePrimitiveType> for $t {
                fn from(v: &ValuePrimitiveType) -> Self {
                    match v {
                        ValuePrimitiveType::Primitive(PrimitiveValue::U64(v)) => *v as $t,
                        _ => 0,
                    }
                }
            }
        )*
    };
}
impl_from_prim_ref_uint!(u8, u16, u32, u64, usize);
// Implement for i8, i16, i32, i64, isize
macro_rules! impl_from_sint_sint {
    ($($t:ty),*) => {
        $(
            impl From<$t> for ValuePrimitiveType {
                fn from(v: $t) -> Self {
                    ValuePrimitiveType::Primitive(PrimitiveValue::I64(v as i64))
                }
            }
        )*
    };
}
impl_from_sint_sint!(i8, i16, i32, i64, isize);
macro_rules! impl_from_sint_ref {
    ($($t:ty),*) => {
        $(
            impl<'a> From<&'a $t> for ValuePrimitiveType {
                fn from(v: &$t) -> Self {
                    ValuePrimitiveType::Primitive(PrimitiveValue::I64(*v as i64))
                }
            }
        )*
    };
}
impl_from_sint_ref!(i8, i16, i32, i64, isize);
// Implement for i8, i16, i32, i64, isize
macro_rules! impl_from_prim_sint {
    ($($t:ty),*) => {
        $(
            impl From<ValuePrimitiveType> for $t {
                fn from(v: ValuePrimitiveType) -> Self {
                    match v {
                        ValuePrimitiveType::Primitive(PrimitiveValue::I64(v)) => v as $t,
                        _ => 0,
                    }
                }
            }
        )*
    };
}
impl_from_prim_sint!(i8, i16, i32, i64, isize);
macro_rules! impl_from_prim_ref_sint {
    ($($t:ty),*) => {
        $(
            impl From<&ValuePrimitiveType> for $t {
                fn from(v: &ValuePrimitiveType) -> Self {
                    match v {
                        ValuePrimitiveType::Primitive(PrimitiveValue::I64(v)) => *v as $t,
                        _ => 0,
                    }
                }
            }
        )*
    };
}
impl_from_prim_ref_sint!(i8, i16, i32, i64, isize);
impl From<f32> for ValuePrimitiveType {
    fn from(v: f32) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::F32(v))
    }
}
impl From<&ValuePrimitiveType> for f32 {
    fn from(v: &ValuePrimitiveType) -> Self {
        match v {
            ValuePrimitiveType::Primitive(PrimitiveValue::F32(v)) => *v,
            _ => 0.0,
        }
    }
}
impl From<f64> for ValuePrimitiveType {
    fn from(v: f64) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::F64(v))
    }
}
impl From<&ValuePrimitiveType> for f64 {
    fn from(v: &ValuePrimitiveType) -> Self {
        match v {
            ValuePrimitiveType::Primitive(PrimitiveValue::F64(v)) => *v,
            _ => 0.0,
        }
    }
}
impl From<bool> for ValuePrimitiveType {
    fn from(v: bool) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::Bool(v))
    }
}
impl From<&ValuePrimitiveType> for bool {
    fn from(v: &ValuePrimitiveType) -> Self {
        match v {
            ValuePrimitiveType::Primitive(PrimitiveValue::Bool(v)) => *v,
            _ => false,
        }
    }
}
impl From<()> for ValuePrimitiveType {
    fn from(_: ()) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::Null)
    }
}
impl From<&ValuePrimitiveType> for () {
    fn from(_: &ValuePrimitiveType) -> Self {}
}
impl From<PrimitiveValue> for ValuePrimitiveType {
    fn from(v: PrimitiveValue) -> Self {
        ValuePrimitiveType::Primitive(v)
    }
}
impl From<&ValuePrimitiveType> for PrimitiveValue {
    fn from(v: &ValuePrimitiveType) -> Self {
        match v {
            ValuePrimitiveType::Primitive(v) => v.clone(),
            _ => PrimitiveValue::Null,
        }
    }
}
impl From<ValuePrimitive> for ValuePrimitiveType {
    fn from(v: ValuePrimitive) -> Self {
        ValuePrimitiveType::NestedPrimitive(v)
    }
}
impl From<&ValuePrimitiveType> for ValuePrimitive {
    fn from(v: &ValuePrimitiveType) -> Self {
        match v {
            ValuePrimitiveType::NestedPrimitive(v) => v.clone(),
            _ => ValuePrimitive::new(),
        }
    }
}
impl<T> From<Option<T>> for ValuePrimitiveType
where
    T: Into<ValuePrimitiveType>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => v.into(),
            None => ValuePrimitiveType::Primitive(PrimitiveValue::Null),
        }
    }
}
impl From<&ValuePrimitiveType> for JSONValue {
    fn from(v: &ValuePrimitiveType) -> Self {
        match v {
            ValuePrimitiveType::Primitive(v) => JSONValue::Primitive(v.clone()),
            ValuePrimitiveType::NestedPrimitive(v) => {
                let mut map = Map::<String, JSONValue>::new();
                for (k, v) in v.iter() {
                    map.insert(k.clone(), v.into());
                }
                JSONValue::Object(map)
            }
        }
    }
}
impl From<&JSONValue> for ValuePrimitiveType {
    fn from(v: &JSONValue) -> Self {
        match v {
            JSONValue::Primitive(v) => ValuePrimitiveType::Primitive(v.clone()),
            JSONValue::Object(v) => {
                let mut map = ValuePrimitive::new();
                for (k, v) in v.iter() {
                    map.insert(k.clone(), v.into());
                }
                ValuePrimitiveType::NestedPrimitive(map)
            }
            // DROPS ALL ARRAY DATA AS IT IS NOT SUPPORTED INSIDE VALUE PRIMITIVES
            _ => ValuePrimitiveType::Primitive(PrimitiveValue::Null),
        }
    }
}

// ValueType
impl Default for ValueType {
    fn default() -> Self {
        ValueType::Primitive(PrimitiveValue::Null)
    }
}
impl ValueType {
    /// Returns the value as a primitive
    pub fn to_prim(&self) -> Option<&PrimitiveValue> {
        match self {
            ValueType::Primitive(v) => Some(v),
            _ => None,
        }
    }

    /// Returns the value as a vector
    pub fn to_vec(&self) -> Option<&Vec<ValuePrimitiveType>> {
        match self {
            ValueType::Array(v) => Some(v),
            _ => None,
        }
    }

    /// Returns the value as a nested object
    pub fn to_nested(&self) -> Option<&Value> {
        match self {
            ValueType::Nested(v) => Some(v),
            _ => None,
        }
    }
}
impl From<&str> for ValueType {
    fn from(s: &str) -> Self {
        ValueType::Primitive(PrimitiveValue::String(s.to_string()))
    }
}
// impl AsRef<str> for ValueType {
//     fn as_ref(&self) -> &str {
//         match self {
//             ValueType::Primitive(PrimitiveValue::String(s)) => s.as_str(),
//             _ => "",
//         }
//     }
// }
impl From<String> for ValueType {
    fn from(s: String) -> Self {
        ValueType::Primitive(PrimitiveValue::String(s))
    }
}
impl From<&String> for ValueType {
    fn from(s: &String) -> Self {
        ValueType::Primitive(PrimitiveValue::String(s.into()))
    }
}
impl From<ValueType> for String {
    fn from(v: ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::String(s)) => s,
            _ => "".to_string(),
        }
    }
}
impl From<&ValueType> for String {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::String(s)) => s.into(),
            _ => "".to_string(),
        }
    }
}

// Implement for u8, u16, u32, u64
macro_rules! impl_from_uint {
    ($($t:ty),*) => {
        $(
            impl From<$t> for ValueType {
                fn from(v: $t) -> Self {
                    ValueType::Primitive(PrimitiveValue::U64(v as u64))
                }
            }

            impl From<ValueType> for $t {
                fn from(v: ValueType) -> Self {
                    match v {
                        ValueType::Primitive(PrimitiveValue::U64(v)) => v as $t,
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
            impl From<&ValueType> for $t {
                fn from(v: &ValueType) -> Self {
                    match v {
                        ValueType::Primitive(PrimitiveValue::U64(v)) => *v as $t,
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
            impl From<$t> for ValueType {
                fn from(v: $t) -> Self {
                    ValueType::Primitive(PrimitiveValue::I64(v as i64))
                }
            }

            impl From<ValueType> for $t {
                fn from(v: ValueType) -> Self {
                    match v {
                        ValueType::Primitive(PrimitiveValue::I64(v)) => v as $t,
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
            impl From<&ValueType> for $t {
                fn from(v: &ValueType) -> Self {
                    match v {
                        ValueType::Primitive(PrimitiveValue::I64(v)) => *v as $t,
                        _ => 0,
                    }
                }
            }
        )*
    };
}
impl_from_sint_ref!(i8, i16, i32, i64, isize);
impl From<f32> for ValueType {
    fn from(v: f32) -> Self {
        ValueType::Primitive(PrimitiveValue::F32(v))
    }
}
impl From<ValueType> for f32 {
    fn from(v: ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::F32(v)) => v,
            _ => 0.0,
        }
    }
}
impl From<&ValueType> for f32 {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::F32(v)) => *v,
            _ => 0.0,
        }
    }
}
impl From<f64> for ValueType {
    fn from(v: f64) -> Self {
        ValueType::Primitive(PrimitiveValue::F64(v))
    }
}
impl From<ValueType> for f64 {
    fn from(v: ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::F64(v)) => v,
            _ => 0.0,
        }
    }
}
impl From<&ValueType> for f64 {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::F64(v)) => *v,
            _ => 0.0,
        }
    }
}
impl From<bool> for ValueType {
    fn from(v: bool) -> Self {
        ValueType::Primitive(PrimitiveValue::Bool(v))
    }
}
impl From<ValueType> for bool {
    fn from(v: ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::Bool(v)) => v,
            _ => false,
        }
    }
}
impl From<&ValueType> for bool {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::Bool(v)) => *v,
            _ => false,
        }
    }
}
impl From<()> for ValueType {
    fn from(_: ()) -> Self {
        ValueType::Primitive(PrimitiveValue::Null)
    }
}
impl From<ValueType> for () {
    fn from(_: ValueType) -> Self {}
}
impl From<&ValueType> for () {
    fn from(_: &ValueType) -> Self {}
}
impl<T> From<Vec<T>> for ValueType
where
    T: Into<ValuePrimitiveType>,
{
    fn from(v: Vec<T>) -> Self {
        ValueType::Array(v.into_iter().map(Into::into).collect())
    }
}
impl<T> From<&Vec<T>> for ValueType
where
    T: Into<ValuePrimitiveType>,
    ValuePrimitiveType: for<'a> From<&'a T>,
{
    fn from(v: &Vec<T>) -> Self {
        ValueType::Array(v.iter().map(Into::into).collect())
    }
}
impl<T> From<ValueType> for Vec<T>
where
    T: From<ValuePrimitiveType>,
{
    fn from(v: ValueType) -> Self {
        match v {
            ValueType::Array(v) => v.into_iter().map(Into::into).collect(),
            _ => Vec::new(),
        }
    }
}
impl<T> From<&ValueType> for Vec<T>
where
    T: for<'a> From<&'a ValuePrimitiveType>,
{
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Array(v) => v.iter().map(Into::into).collect(),
            _ => Vec::new(),
        }
    }
}
impl From<Value> for ValueType {
    fn from(v: Value) -> Self {
        ValueType::Nested(v)
    }
}
impl From<ValueType> for Value {
    fn from(v: ValueType) -> Self {
        match v {
            ValueType::Nested(v) => v,
            _ => Value::default(),
        }
    }
}
impl From<&ValueType> for Value {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Nested(v) => v.clone(),
            _ => Value::default(),
        }
    }
}
impl<T> From<Option<T>> for ValueType
where
    T: Into<ValueType>,
{
    fn from(v: Option<T>) -> Self {
        match v {
            Some(v) => v.into(),
            None => ValueType::Primitive(PrimitiveValue::Null),
        }
    }
}
impl From<&JSONValue> for ValueType {
    fn from(v: &JSONValue) -> Self {
        match v {
            JSONValue::Primitive(v) => ValueType::Primitive(v.clone()),
            JSONValue::Array(v) => ValueType::Array(v.iter().map(Into::into).collect()),
            JSONValue::Object(v) => {
                let mut res = Value::new();
                for (k, v) in v.iter() {
                    res.insert(k.clone(), v.into());
                }
                ValueType::Nested(res)
            }
        }
    }
}
impl From<&ValueType> for JSONValue {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Primitive(v) => JSONValue::Primitive(v.clone()),
            ValueType::Array(v) => JSONValue::Array(v.iter().map(Into::into).collect()),
            ValueType::Nested(v) => {
                let mut res = Map::<String, JSONValue>::new();
                for (k, v) in v.iter() {
                    res.insert(k.clone(), v.into());
                }
                JSONValue::Object(res)
            }
        }
    }
}

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

impl MValueCompatible for JSONProperties {}
impl From<JSONProperties> for MValue {
    fn from(json: JSONProperties) -> MValue {
        let mut res = MValue::new();
        for (k, v) in json.iter() {
            res.insert(k.clone(), v.into());
        }
        res
    }
}
impl From<&JSONProperties> for MValue {
    fn from(json: &JSONProperties) -> MValue {
        let mut res = MValue::new();
        for (k, v) in json.iter() {
            res.insert(k.clone(), v.into());
        }
        res
    }
}
impl From<MValue> for JSONProperties {
    fn from(v: MValue) -> JSONProperties {
        let mut res = JSONProperties::new();
        for (k, v) in v.iter() {
            res.insert(k.clone(), v.into());
        }
        res
    }
}
impl From<&MValue> for JSONProperties {
    fn from(v: &MValue) -> JSONProperties {
        let mut res = JSONProperties::new();
        for (k, v) in v.iter() {
            res.insert(k.clone(), v.into());
        }
        res
    }
}

impl MValueCompatible for MapboxProperties {}
impl From<MapboxProperties> for MValue {
    fn from(json: MapboxProperties) -> MValue {
        let mut res = MValue::new();
        for (k, v) in json.iter() {
            res.insert(k.clone(), ValueType::Primitive(v.clone()));
        }
        res
    }
}
impl From<&MapboxProperties> for MValue {
    fn from(json: &MapboxProperties) -> MValue {
        let mut res = MValue::new();
        for (k, v) in json.iter() {
            res.insert(k.clone(), ValueType::Primitive(v.clone()));
        }
        res
    }
}
impl From<MValue> for MapboxProperties {
    fn from(v: MValue) -> MapboxProperties {
        let mut res = MapboxProperties::new();
        // Only copy over primitive values
        for (k, v) in v.iter() {
            let value = v.clone();
            if let Some(p) = value.to_prim() {
                res.insert(k.clone(), p.clone());
            }
        }
        res
    }
}
impl From<&MValue> for MapboxProperties {
    fn from(v: &MValue) -> MapboxProperties {
        let mut res = MapboxProperties::new();
        // Only copy over primitive values
        for (k, v) in v.iter() {
            let value = v.clone();
            if let Some(p) = value.to_prim() {
                res.insert(k.clone(), p.clone());
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MValue, MValueCompatible, VectorPoint};
    use alloc::vec;

    #[test]
    fn value_default() {
        let default = ValueType::default();
        assert_eq!(default, ValueType::Primitive(PrimitiveValue::Null));
    }

    #[test]
    fn primitive_value_funcs() {
        // &str
        let prim_value: PrimitiveValue = "test".into();
        assert_eq!(PrimitiveValue::String("test".into()), prim_value);
        assert_eq!(prim_value.to_u64(), None);
        assert_eq!(prim_value.to_i64(), None);
        assert_eq!(prim_value.to_f32(), None);
        assert_eq!(prim_value.to_f64(), None);
        assert_eq!(prim_value.to_bool(), None);
        assert!(!prim_value.is_null());
        // String
        let prim_value_str: String = "test".into();
        let prim_value: PrimitiveValue = prim_value_str.clone().into();
        assert_eq!(PrimitiveValue::String("test".into()), prim_value);
        assert_eq!(prim_value.to_string(), Some("test".into()));
        // u64
        let prim_value: PrimitiveValue = 1_u64.into();
        assert_eq!(PrimitiveValue::U64(1), prim_value);
        assert_eq!(prim_value.to_string(), None);
        assert_eq!(prim_value.to_u64(), Some(1));
        assert_eq!(prim_value.to_i64(), Some(1));
        assert_eq!(prim_value.to_f32(), Some(1.0));
        assert_eq!(prim_value.to_f64(), Some(1.0));
        // i64
        let prim_value: PrimitiveValue = (-1_i64).into();
        assert_eq!(PrimitiveValue::I64(-1), prim_value);
        assert_eq!(prim_value.to_u64(), Some(18446744073709551615));
        assert_eq!(prim_value.to_i64(), Some(-1));
        assert_eq!(prim_value.to_f32(), Some(-1.0));
        assert_eq!(prim_value.to_f64(), Some(-1.0));
        // f32
        let prim_value: PrimitiveValue = (1.0_f32).into();
        assert_eq!(PrimitiveValue::F32(1.0), prim_value);
        assert_eq!(prim_value.to_u64(), Some(1));
        assert_eq!(prim_value.to_i64(), Some(1));
        assert_eq!(prim_value.to_f32(), Some(1.0));
        assert_eq!(prim_value.to_f64(), Some(1.0));
        // f64
        let prim_value: PrimitiveValue = (1.0_f64).into();
        assert_eq!(PrimitiveValue::F64(1.0), prim_value);
        assert_eq!(prim_value.to_u64(), Some(1));
        assert_eq!(prim_value.to_i64(), Some(1));
        assert_eq!(prim_value.to_f32(), Some(1.0));
        assert_eq!(prim_value.to_f64(), Some(1.0));
        // bool
        let prim_value: PrimitiveValue = true.into();
        assert_eq!(PrimitiveValue::Bool(true), prim_value);
        assert_eq!(prim_value.to_bool(), Some(true));
        // ()
        let prim_value: PrimitiveValue = ().into();
        assert_eq!(PrimitiveValue::Null, prim_value);
        assert!(prim_value.is_null());
        // Option
        let prim_value: PrimitiveValue = Some(true).into();
        assert_eq!(PrimitiveValue::Bool(true), prim_value);
        assert_eq!(prim_value.to_bool(), Some(true));
        let prim_value: PrimitiveValue = None::<bool>.into();
        assert_eq!(PrimitiveValue::Null, prim_value);
        assert!(prim_value.is_null());
    }

    #[test]
    fn value_prim_type_funcs() {
        // &str
        let prim_value: ValuePrimitiveType = "test".into();
        assert_eq!(
            ValuePrimitiveType::Primitive(PrimitiveValue::String("test".into())),
            prim_value
        );
        assert_eq!(prim_value.to_prim(), Some(PrimitiveValue::String("test".into())).as_ref());
        assert_eq!(prim_value.to_nested(), None);
        // String
        let prim_value_str: String = "test".into();
        let prim_value: ValuePrimitiveType = prim_value_str.clone().into();
        assert_eq!(
            ValuePrimitiveType::Primitive(PrimitiveValue::String("test".into())),
            prim_value
        );
        // u64
        let prim_value: ValuePrimitiveType = 1_u64.into();
        assert_eq!(ValuePrimitiveType::Primitive(PrimitiveValue::U64(1)), prim_value);
        // i64
        let prim_value: ValuePrimitiveType = (-1_i64).into();
        assert_eq!(ValuePrimitiveType::Primitive(PrimitiveValue::I64(-1)), prim_value);
        // f32
        let prim_value: ValuePrimitiveType = (1.0_f32).into();
        assert_eq!(ValuePrimitiveType::Primitive(PrimitiveValue::F32(1.0)), prim_value);
        // f64
        let prim_value: ValuePrimitiveType = (1.0_f64).into();
        assert_eq!(ValuePrimitiveType::Primitive(PrimitiveValue::F64(1.0)), prim_value);
        // bool
        let prim_value: ValuePrimitiveType = true.into();
        assert_eq!(ValuePrimitiveType::Primitive(PrimitiveValue::Bool(true)), prim_value);
        // ()
        let prim_value: ValuePrimitiveType = ().into();
        assert_eq!(ValuePrimitiveType::Primitive(PrimitiveValue::Null), prim_value);

        // from prim
        let nested: ValuePrimitiveType = PrimitiveValue::Bool(true).into();
        assert_eq!(nested.to_prim().unwrap().to_bool(), Some(true));

        // nested
        let nested: ValuePrimitiveType =
            ValuePrimitive::from([("a".into(), "b".into()), ("c".into(), 2.0_f32.into())]).into();
        assert_eq!(nested.to_prim(), None);
        assert_eq!(
            nested.to_nested(),
            Some(ValuePrimitive::from([("a".into(), "b".into()), ("c".into(), 2.0_f32.into()),]))
                .as_ref()
        );

        // option
        let prim_value: ValuePrimitiveType = Some(true).into();
        assert_eq!(ValuePrimitiveType::Primitive(PrimitiveValue::Bool(true)), prim_value);
        let prim_value: ValuePrimitiveType = None::<bool>.into();
        assert_eq!(ValuePrimitiveType::Primitive(PrimitiveValue::Null), prim_value);
    }

    #[test]
    fn value_funcs() {
        // &str
        let prim_value: ValueType = "test".into();
        assert_eq!(ValueType::Primitive(PrimitiveValue::String("test".into())), prim_value);
        let prim = prim_value.to_prim().unwrap();
        assert_eq!(*prim, PrimitiveValue::String("test".into()));
        assert_eq!(prim_value.to_nested(), None);
        assert_eq!(prim_value.to_vec(), None);
        // String
        let prim_value_str: String = "test".into();
        let prim_value: ValueType = prim_value_str.into();
        assert_eq!(ValueType::Primitive(PrimitiveValue::String("test".into())), prim_value);
        // u64
        let prim_value: ValueType = 1_u64.into();
        assert_eq!(ValueType::Primitive(PrimitiveValue::U64(1)), prim_value);
        // i64
        let prim_value: ValueType = (-1_i64).into();
        assert_eq!(ValueType::Primitive(PrimitiveValue::I64(-1)), prim_value);
        // f32
        let prim_value: ValueType = (1.0_f32).into();
        assert_eq!(ValueType::Primitive(PrimitiveValue::F32(1.0)), prim_value);
        // f64
        let prim_value: ValueType = (1.0_f64).into();
        assert_eq!(ValueType::Primitive(PrimitiveValue::F64(1.0)), prim_value);
        // bool
        let prim_value: ValueType = true.into();
        assert_eq!(ValueType::Primitive(PrimitiveValue::Bool(true)), prim_value);
        // ()
        let prim_value: ValueType = ().into();
        assert_eq!(ValueType::Primitive(PrimitiveValue::Null), prim_value);

        // vec
        let prim_value: ValueType = vec!["test", "test2"].into();
        assert_eq!(prim_value.to_prim(), None);
        assert_eq!(
            ValueType::Array(vec![
                ValuePrimitiveType::Primitive(PrimitiveValue::String("test".into())),
                ValuePrimitiveType::Primitive(PrimitiveValue::String("test2".into())),
            ]),
            prim_value
        );
        let back_to_vec: Vec<String> =
            prim_value.to_vec().unwrap().iter().filter_map(|v| v.to_prim()?.to_string()).collect();
        assert_eq!(back_to_vec, vec!["test", "test2"]);

        // nested
        let nested: ValueType =
            Value::from([("a".into(), "b".into()), ("c".into(), 2.0_f32.into())]).into();
        assert_eq!(nested.to_vec(), None);
        assert_eq!(
            nested.to_nested(),
            Some(Value::from([("a".into(), "b".into()), ("c".into(), 2.0_f32.into()),])).as_ref()
        );

        // option
        let prim_value: ValueType = Some(true).into();
        assert_eq!(ValueType::Primitive(PrimitiveValue::Bool(true)), prim_value);
        let prim_value: ValueType = None::<bool>.into();
        assert_eq!(ValueType::Primitive(PrimitiveValue::Null), prim_value);
    }

    #[test]
    fn test_rgba_struct() {
        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Rgba {
            /// Gamma corrected Red between 0 and 1
            pub r: f64,
            /// Gamma corrected Green between 0 and 1
            pub g: f64,
            /// Gamma corrected Blue between 0 and 1
            pub b: f64,
            /// Opacity between 0 and 1 (not gamma corrected as opacity is linear)
            pub a: f64,
        }
        impl Rgba {
            /// Create a new RGBA value
            pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
                Self { r, g, b, a }
            }
        }
        impl MValueCompatible for Rgba {}
        impl From<Rgba> for MValue {
            fn from(rgba: Rgba) -> MValue {
                MValue::from([
                    ("r".into(), (rgba.r).into()),
                    ("g".into(), (rgba.g).into()),
                    ("b".into(), (rgba.b).into()),
                    ("a".into(), (rgba.a).into()),
                ])
            }
        }
        impl From<&Rgba> for MValue {
            fn from(rgba: &Rgba) -> MValue {
                MValue::from([
                    ("r".into(), (rgba.r).into()),
                    ("g".into(), (rgba.g).into()),
                    ("b".into(), (rgba.b).into()),
                    ("a".into(), (rgba.a).into()),
                ])
            }
        }
        impl From<MValue> for Rgba {
            fn from(mvalue: MValue) -> Self {
                let r: f64 = mvalue.get("r").unwrap().to_prim().unwrap().to_f64().unwrap();
                let g = mvalue.get("g").unwrap().to_prim().unwrap().to_f64().unwrap();
                let b = mvalue.get("b").unwrap().to_prim().unwrap().to_f64().unwrap();
                let a = mvalue.get("a").unwrap().to_prim().unwrap().to_f64().unwrap();
                Rgba::new(r, g, b, a)
            }
        }
        impl From<&MValue> for Rgba {
            fn from(mvalue: &MValue) -> Self {
                let r: f64 = mvalue.get("r").unwrap().to_prim().unwrap().to_f64().unwrap();
                let g = mvalue.get("g").unwrap().to_prim().unwrap().to_f64().unwrap();
                let b = mvalue.get("b").unwrap().to_prim().unwrap().to_f64().unwrap();
                let a = mvalue.get("a").unwrap().to_prim().unwrap().to_f64().unwrap();
                Rgba::new(r, g, b, a)
            }
        }

        let rgba = Rgba::new(0.1, 0.2, 0.3, 0.4);
        let rgba_mvalue: MValue = (&rgba).into();
        assert_eq!(
            rgba_mvalue,
            MValue::from([
                ("r".into(), ValueType::Primitive(PrimitiveValue::F64(0.1))),
                ("g".into(), ValueType::Primitive(PrimitiveValue::F64(0.2))),
                ("b".into(), ValueType::Primitive(PrimitiveValue::F64(0.3))),
                ("a".into(), ValueType::Primitive(PrimitiveValue::F64(0.4))),
            ])
        );
        let back_to_rgba: Rgba = (&rgba_mvalue).into();
        assert_eq!(rgba, back_to_rgba);
        let back_to_rgba: Rgba = rgba_mvalue.clone().into();
        assert_eq!(rgba, back_to_rgba);

        let vp: VectorPoint<Rgba> = VectorPoint { x: 1.0, y: 2.0, z: None, m: Some(rgba), t: None };
        let vp_mvalue: MValue = vp.m.unwrap().into();
        assert_eq!(vp_mvalue, rgba_mvalue);

        // distance
        let a: VectorPoint<Rgba> = VectorPoint { x: 1.0, y: 2.0, z: None, m: Some(rgba), t: None };
        let b: VectorPoint = VectorPoint::new(3.0, 4.0, None, None);
        let dist = a.distance(&b);
        assert_eq!(dist, 2.8284271247461903);
    }

    #[test]
    fn to_mapbox() {
        let value: MValue = MValue::from([
            ("a".into(), "b".into()),
            ("c".into(), 2.0_f32.into()),
            (
                "d".into(),
                MValue::from([("2".into(), "3".into()), ("4".into(), 2.0_f32.into())]).into(),
            ),
        ]);
        let mapbox_value: MapboxProperties = value.clone().into();
        assert_eq!(
            mapbox_value,
            MapboxProperties::from([("a".into(), "b".into()), ("c".into(), 2.0_f32.into()),])
        );
        let mapbox_value: MapboxProperties = (&value).into();
        assert_eq!(
            mapbox_value,
            MapboxProperties::from([("a".into(), "b".into()), ("c".into(), 2.0_f32.into()),])
        );
    }

    #[test]
    fn from_mapbox() {
        let mapbox_value: MapboxProperties = MapboxProperties::from([("a".into(), "b".into())]);
        let value: MValue = mapbox_value.clone().into();
        assert_eq!(value, MValue::from([("a".into(), "b".into()),]));
        let value: MValue = (&mapbox_value).into();
        assert_eq!(value, MValue::from([("a".into(), "b".into()),]));
    }

    #[test]
    fn to_json_obj() {
        let value: MValue = MValue::from([
            ("a".into(), "b".into()),
            ("c".into(), 2.0_f32.into()),
            (
                "d".into(),
                MValue::from([("2".into(), "3".into()), ("4".into(), 2.0_f32.into())]).into(),
            ),
            (
                "e".into(),
                Vec::<ValuePrimitiveType>::from(["a".into(), "b".into(), "c".into()]).into(),
            ),
        ]);
        let json_value: JSONProperties = value.clone().into();
        assert_eq!(
            json_value,
            JSONProperties::from([
                ("a".into(), JSONValue::Primitive(PrimitiveValue::String("b".into()))),
                ("c".into(), JSONValue::Primitive(PrimitiveValue::F32(2.0))),
                (
                    "d".into(),
                    JSONValue::Object(JSONProperties::from([
                        ("2".into(), JSONValue::Primitive(PrimitiveValue::String("3".into()))),
                        ("4".into(), JSONValue::Primitive(PrimitiveValue::F32(2.0))),
                    ]))
                ),
                (
                    "e".into(),
                    JSONValue::Array(Vec::from([
                        JSONValue::Primitive(PrimitiveValue::String("a".into())),
                        JSONValue::Primitive(PrimitiveValue::String("b".into())),
                        JSONValue::Primitive(PrimitiveValue::String("c".into())),
                    ]))
                ),
            ])
        );
        let json_value: JSONProperties = (&value).into();
        assert_eq!(
            json_value,
            JSONProperties::from([
                ("a".into(), JSONValue::Primitive(PrimitiveValue::String("b".into()))),
                ("c".into(), JSONValue::Primitive(PrimitiveValue::F32(2.0))),
                (
                    "d".into(),
                    JSONValue::Object(JSONProperties::from([
                        ("2".into(), JSONValue::Primitive(PrimitiveValue::String("3".into()))),
                        ("4".into(), JSONValue::Primitive(PrimitiveValue::F32(2.0))),
                    ]))
                ),
                (
                    "e".into(),
                    JSONValue::Array(Vec::from([
                        JSONValue::Primitive(PrimitiveValue::String("a".into())),
                        JSONValue::Primitive(PrimitiveValue::String("b".into())),
                        JSONValue::Primitive(PrimitiveValue::String("c".into())),
                    ]))
                ),
            ])
        );

        // get prim
        let prim_a = json_value.get("a").unwrap().to_prim().unwrap().to_string().unwrap();
        assert_eq!(prim_a, "b");
        let failed_to_prim = json_value.get("d").unwrap().to_prim();
        assert_eq!(failed_to_prim, None);

        // get array
        let array_e = json_value.get("e").unwrap().to_vec().unwrap();
        assert_eq!(
            *array_e,
            Vec::from([
                JSONValue::Primitive(PrimitiveValue::String("a".into())),
                JSONValue::Primitive(PrimitiveValue::String("b".into())),
                JSONValue::Primitive(PrimitiveValue::String("c".into())),
            ])
        );
        let array_fail = json_value.get("a").unwrap().to_vec();
        assert_eq!(array_fail, None);

        // get obj
        let obj_d = json_value.get("d").unwrap().to_nested().unwrap();
        assert_eq!(
            *obj_d,
            JSONProperties::from([
                ("2".into(), JSONValue::Primitive(PrimitiveValue::String("3".into()))),
                ("4".into(), JSONValue::Primitive(PrimitiveValue::F32(2.0))),
            ])
        );
        let obj_fail = json_value.get("a").unwrap().to_nested();
        assert_eq!(obj_fail, None);
    }

    #[test]
    fn from_json_obj() {
        let json_value = JSONProperties::from([
            ("a".into(), JSONValue::Primitive(PrimitiveValue::String("b".into()))),
            ("c".into(), JSONValue::Primitive(PrimitiveValue::F32(2.0))),
            (
                "d".into(),
                JSONValue::Object(JSONProperties::from([
                    ("2".into(), JSONValue::Primitive(PrimitiveValue::String("3".into()))),
                    ("4".into(), JSONValue::Primitive(PrimitiveValue::F32(2.0))),
                ])),
            ),
            (
                "e".into(),
                JSONValue::Array(Vec::from([
                    JSONValue::Primitive(PrimitiveValue::String("a".into())),
                    JSONValue::Primitive(PrimitiveValue::String("b".into())),
                    JSONValue::Primitive(PrimitiveValue::String("c".into())),
                ])),
            ),
        ]);
        let value: MValue = json_value.clone().into();
        assert_eq!(
            value,
            MValue::from([
                ("a".into(), "b".into()),
                ("c".into(), 2.0_f32.into()),
                (
                    "d".into(),
                    MValue::from([("2".into(), "3".into()), ("4".into(), 2.0_f32.into())]).into(),
                ),
                (
                    "e".into(),
                    Vec::<ValuePrimitiveType>::from(["a".into(), "b".into(), "c".into()]).into(),
                ),
            ])
        );
        let value: MValue = (&json_value).into();
        assert_eq!(
            value,
            MValue::from([
                ("a".into(), "b".into()),
                ("c".into(), 2.0_f32.into()),
                (
                    "d".into(),
                    MValue::from([("2".into(), "3".into()), ("4".into(), 2.0_f32.into())]).into(),
                ),
                (
                    "e".into(),
                    Vec::<ValuePrimitiveType>::from(["a".into(), "b".into(), "c".into()]).into(),
                ),
            ])
        );
    }

    #[test]
    fn test_prim_to_json() {
        let json: JSONValue = (&PrimitiveValue::String("test".into())).into();
        assert_eq!(json, JSONValue::Primitive(PrimitiveValue::String("test".into())));

        let prim: PrimitiveValue = (&json).into();
        assert_eq!(prim, PrimitiveValue::String("test".into()));

        // to prim but json is not a prim
        let json = JSONValue::Array(Vec::new());
        let prim: PrimitiveValue = (&json).into();
        assert_eq!(prim, PrimitiveValue::Null);
    }

    #[test]
    fn test_value_prim_type_to_json() {
        let prim = ValuePrimitiveType::NestedPrimitive(Map::from([
            ("a".into(), "b".into()),
            ("c".into(), 2.0_f32.into()),
        ]));
        let json: JSONValue = (&prim).into();
        assert_eq!(
            json,
            JSONValue::Object(JSONProperties::from([
                ("a".into(), JSONValue::Primitive(PrimitiveValue::String("b".into()))),
                ("c".into(), JSONValue::Primitive(PrimitiveValue::F32(2.0))),
            ]))
        );

        let json = JSONValue::Object(JSONProperties::from([
            ("2".into(), JSONValue::Primitive(PrimitiveValue::String("3".into()))),
            ("4".into(), JSONValue::Primitive(PrimitiveValue::F32(2.0))),
        ]));

        let prim: ValuePrimitiveType = (&json).into();
        assert_eq!(
            prim,
            ValuePrimitiveType::NestedPrimitive(Map::from([
                ("2".into(), "3".into()),
                ("4".into(), 2.0_f32.into()),
            ]))
        );

        // Array Fails
        let json = JSONValue::Array(Vec::from([
            JSONValue::Primitive(PrimitiveValue::String("c".into())),
            JSONValue::Primitive(PrimitiveValue::String("d".into())),
        ]));

        let prim: ValuePrimitiveType = (&json).into();
        assert_eq!(prim, ValuePrimitiveType::Primitive(PrimitiveValue::Null));
    }

    #[test]
    fn test_prim_to_value_prim_type() {
        // f32
        let value: ValuePrimitiveType = 2.0_f32.into();
        assert_eq!(value, ValuePrimitiveType::Primitive(2.0_f32.into()));
        let back_to_num: f32 = (&value).into();
        assert_eq!(back_to_num, 2.0_f32);
        // f64
        let value: ValuePrimitiveType = (-2.2_f64).into();
        assert_eq!(value, ValuePrimitiveType::Primitive((-2.2_f64).into()));
        let back_to_num: f64 = (&value).into();
        assert_eq!(back_to_num, (-2.2_f64));
        // u64
        let value: ValuePrimitiveType = 2_u64.into();
        assert_eq!(value, ValuePrimitiveType::Primitive(2_u64.into()));
        let back_to_num: u64 = (&value).into();
        assert_eq!(back_to_num, 2_u64);
        // i64
        let value: ValuePrimitiveType = 2_i64.into();
        assert_eq!(value, ValuePrimitiveType::Primitive(2_i64.into()));
        let back_to_num: i64 = (&value).into();
        assert_eq!(back_to_num, 2_i64);
        let back_to_str: String = (&value).into();
        assert_eq!(back_to_str, "");

        // string
        let value: ValuePrimitiveType = "test".into();
        assert_eq!(value, ValuePrimitiveType::Primitive("test".into()));
        let back_to_str: String = (&value).into();
        assert_eq!(back_to_str, "test");
        let back_to_num: i64 = (&value).into();
        assert_eq!(back_to_num, 0);
        let back_to_num: u64 = (&value).into();
        assert_eq!(back_to_num, 0);
        let back_to_num: f32 = (&value).into();
        assert_eq!(back_to_num, 0.);
        let back_to_num: f64 = (&value).into();
        assert_eq!(back_to_num, 0.);
        let back_to_bool: bool = (&value).into();
        assert!(!back_to_bool);
        // bool
        let value: ValuePrimitiveType = true.into();
        assert_eq!(value, ValuePrimitiveType::Primitive(true.into()));
        let back_to_bool: bool = (&value).into();
        assert!(back_to_bool);
        // ()
        let value: ValuePrimitiveType = ().into();
        assert_eq!(value, ValuePrimitiveType::Primitive(PrimitiveValue::Null));
        let _back_to_base: () = (&value).into();

        // PrimitiveValue
        let value: ValuePrimitiveType = PrimitiveValue::Null.into();
        assert_eq!(value, ValuePrimitiveType::Primitive(PrimitiveValue::Null));
        let back_to_base: PrimitiveValue = (&value).into();
        assert_eq!(back_to_base, PrimitiveValue::Null);
        let value: ValuePrimitiveType = Map::new().into();
        let bac_to_base: PrimitiveValue = (&value).into();
        assert_eq!(bac_to_base, PrimitiveValue::Null);

        // ValuePrimitive
        let value: ValuePrimitiveType = ValuePrimitive::new().into();
        let back_to_base: ValuePrimitive = (&value).into();
        assert_eq!(back_to_base, ValuePrimitive::new());
        let value: ValuePrimitiveType = PrimitiveValue::Null.into();
        let back_to_base: ValuePrimitive = (&value).into();
        assert_eq!(back_to_base, ValuePrimitive::new());
    }

    #[test]
    fn test_value_type() {
        // ref string value
        let val_type: ValueType = "test".into();
        assert_eq!(val_type, ValueType::Primitive(PrimitiveValue::String("test".into())));
        let back_to_str: String = (&val_type).into();
        assert_eq!(back_to_str, "test");
        // direct string value
        let string: String = "test".into();
        let val_type: ValueType = (&string).into();
        assert_eq!(val_type, ValueType::Primitive(PrimitiveValue::String("test".into())));
        let back_to_str: String = val_type.into();
        assert_eq!(back_to_str, "test");

        // direct fake a string
        let value: ValueType = 2_i64.into();
        let back_to_str: String = (&value).into();
        assert_eq!(back_to_str, "");
        // ref fake a string
        let value: ValueType = 2_i64.into();
        let back_to_str: String = value.into();
        assert_eq!(back_to_str, "");

        // f32
        let value: ValueType = 2.0_f32.into();
        assert_eq!(value, ValueType::Primitive(2.0_f32.into()));
        let back_to_num: f32 = (&value).into();
        assert_eq!(back_to_num, 2.0_f32);
        // f32 no ref
        let value: ValueType = 2.0_f32.into();
        assert_eq!(value, ValueType::Primitive(2.0_f32.into()));
        let back_to_num: f32 = value.into();
        assert_eq!(back_to_num, 2.0_f32);
        // f64
        let value: ValueType = (-2.2_f64).into();
        assert_eq!(value, ValueType::Primitive((-2.2_f64).into()));
        let back_to_num: f64 = (&value).into();
        assert_eq!(back_to_num, (-2.2_f64));
        // f64 no ref
        let value: ValueType = (-2.2_f64).into();
        assert_eq!(value, ValueType::Primitive((-2.2_f64).into()));
        let back_to_num: f64 = value.into();
        assert_eq!(back_to_num, (-2.2_f64));
        // u64
        let value: ValueType = 2_u64.into();
        assert_eq!(value, ValueType::Primitive(2_u64.into()));
        let back_to_num: u64 = (&value).into();
        assert_eq!(back_to_num, 2_u64);
        // u64 no ref
        let value: ValueType = 2_u64.into();
        assert_eq!(value, ValueType::Primitive(2_u64.into()));
        let back_to_num: u64 = value.into();
        assert_eq!(back_to_num, 2_u64);
        // i64
        let value: ValueType = 2_i64.into();
        assert_eq!(value, ValueType::Primitive(2_i64.into()));
        let back_to_num: i64 = (&value).into();
        assert_eq!(back_to_num, 2_i64);
        let back_to_str: String = (&value).into();
        assert_eq!(back_to_str, "");
        // i64 no ref
        let value: ValueType = 2_i64.into();
        assert_eq!(value, ValueType::Primitive(2_i64.into()));
        let back_to_num: i64 = value.into();
        assert_eq!(back_to_num, 2_i64);

        // string
        let value: ValueType = "test".into();
        assert_eq!(value, ValueType::Primitive("test".into()));
        let back_to_str: String = (&value).into();
        assert_eq!(back_to_str, "test");
        let back_to_num: i64 = (&value).into();
        assert_eq!(back_to_num, 0);
        let back_to_num: u64 = (&value).into();
        assert_eq!(back_to_num, 0);
        let back_to_num: usize = (&value).into();
        assert_eq!(back_to_num, 0);
        let back_to_num: f32 = (&value).into();
        assert_eq!(back_to_num, 0.);
        let back_to_num: f64 = (&value).into();
        assert_eq!(back_to_num, 0.);
        let back_to_bool: bool = (&value).into();
        assert!(!back_to_bool);
        // string no ref
        let value: ValueType = "test".into();
        assert_eq!(value, ValueType::Primitive("test".into()));
        let back_to_str: String = value.clone().into();
        assert_eq!(back_to_str, "test");
        let back_to_num: i64 = value.clone().into();
        assert_eq!(back_to_num, 0);
        let back_to_num: u64 = value.clone().into();
        assert_eq!(back_to_num, 0);
        let back_to_num: usize = value.clone().into();
        assert_eq!(back_to_num, 0);
        let back_to_num: f32 = value.clone().into();
        assert_eq!(back_to_num, 0.);
        let back_to_num: f64 = value.clone().into();
        assert_eq!(back_to_num, 0.);
        let back_to_bool: bool = value.into();
        assert!(!back_to_bool);
        // bool
        let value: ValueType = true.into();
        assert_eq!(value, ValueType::Primitive(true.into()));
        let back_to_bool: bool = (&value).into();
        assert!(back_to_bool);
        // bool no ref
        let value: ValueType = true.into();
        assert_eq!(value, ValueType::Primitive(true.into()));
        let back_to_bool: bool = value.into();
        assert!(back_to_bool);
        // ()
        let value: ValueType = ().into();
        assert_eq!(value, ValueType::Primitive(PrimitiveValue::Null));
        let _back_to_base: () = (&value).into();
        // () no ref
        let value: ValueType = ().into();
        assert_eq!(value, ValueType::Primitive(PrimitiveValue::Null));
        let _back_to_base: () = value.into();

        // vec
        let data: Vec<u64> = vec![1, 2, 3];
        let value: ValueType = data.into();
        let back_to_vec: Vec<u64> = (&value).into();
        assert_eq!(back_to_vec, vec![1, 2, 3]);
        // vec ref
        let data: Vec<u64> = vec![1, 2, 3];
        let value: ValueType = (&data).into();
        let back_to_vec: Vec<u64> = value.into();
        assert_eq!(back_to_vec, vec![1, 2, 3]);
        // vec ref i
        let data: Vec<isize> = vec![1, 2, 3];
        let value: ValueType = (&data).into();
        let back_to_vec: Vec<isize> = value.into();
        assert_eq!(back_to_vec, vec![1, 2, 3]);
        // vec from ValueType not array
        let value: ValueType = ValueType::Primitive("test".into());
        let back_to_vec: Vec<u64> = (&value).into();
        assert_eq!(back_to_vec, Vec::<u64>::new());

        // Value
        let data: Value = Value::from([("a".into(), "b".into())]);
        let value: ValueType = data.into();
        let back_to_value: Value = value.into();
        assert_eq!(back_to_value, Value::from([("a".into(), "b".into())]));
        // Value ref
        let data: Value = Value::from([("a".into(), "b".into())]);
        let value: ValueType = data.into();
        let back_to_value: Value = value.into();
        assert_eq!(back_to_value, Value::from([("a".into(), "b".into())]));

        // ValueType not nested
        let value: ValueType = ValueType::Primitive("test".into());
        let to_value: Value = value.into();
        assert_eq!(to_value, Value::default());
        // ValueType not nested ref
        let value: ValueType = ValueType::Primitive("test".into());
        let to_value: Value = (&value).into();
        assert_eq!(to_value, Value::default());
    }
}
