use crate::*;
use alloc::{string::String, vec, vec::Vec};
use core::cmp::Ordering;
use libm::round;
use pbf::{ProtoRead, ProtoWrite, Protobuf, Type};

// PrimitiveValue
impl PrimitiveValue {
    /// Returns true if the value is null
    pub fn is_null(&self) -> bool {
        matches!(self, PrimitiveValue::Null)
    }

    /// returns true if the shape is a number type
    pub fn is_number(&self) -> bool {
        matches!(
            self,
            PrimitiveValue::F64(_)
                | PrimitiveValue::F32(_)
                | PrimitiveValue::I64(_)
                | PrimitiveValue::U64(_)
        )
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
            PrimitiveValue::String(v) => v.parse().ok(),
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
            PrimitiveValue::String(v) => v.parse().ok(),
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
            PrimitiveValue::String(v) => v.parse().ok(),
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
            PrimitiveValue::String(v) => v.parse().ok(),
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
            PrimitiveValue::String(v) => {
                if v == "true" {
                    Some(true)
                } else if v == "false" {
                    Some(false)
                } else {
                    None
                }
            }
            PrimitiveValue::Bool(v) => Some(*v),
            _ => None,
        }
    }
}
impl From<&str> for PrimitiveValue {
    fn from(s: &str) -> Self {
        PrimitiveValue::String(s.into())
    }
}
impl From<String> for PrimitiveValue {
    fn from(s: String) -> Self {
        PrimitiveValue::String(s)
    }
}
impl From<&PrimitiveValue> for String {
    fn from(v: &PrimitiveValue) -> Self {
        v.to_string().unwrap_or_default()
    }
}
// Implement for u8, u16, u32, u64
macro_rules! impl_from_uint_to_prim_val {
    ($($t:ty),*) => {
        $(
            impl From<$t> for PrimitiveValue {
                fn from(v: $t) -> Self {
                    PrimitiveValue::U64(v as u64)
                }
            }
        )*
    };
}
impl_from_uint_to_prim_val!(u8, u16, u32, u64, usize);
macro_rules! impl_from_value_prim_to_uint {
    ($($t:ty),*) => {
        $(
            impl From<&PrimitiveValue> for $t {
                fn from(v: &PrimitiveValue) -> Self {
                    v.to_u64().unwrap_or_default() as $t
                }
            }
        )*
    };
}
impl_from_value_prim_to_uint!(u8, u16, u32, u64, usize);
// Implement for i8, i16, i32, i64, and isize
macro_rules! impl_from_sint_to_prim_val {
    ($($t:ty),*) => {
        $(
            impl From<$t> for PrimitiveValue {
                fn from(v: $t) -> Self {
                    PrimitiveValue::I64(v as i64)
                }
            }
        )*
    };
}
impl_from_sint_to_prim_val!(i8, i16, i32, i64, isize);
macro_rules! impl_from_value_prim_to_sint {
    ($($t:ty),*) => {
        $(
            impl From<&PrimitiveValue> for $t {
                fn from(v: &PrimitiveValue) -> Self {
                    v.to_i64().unwrap_or_default() as $t
                }
            }
        )*
    };
}
impl_from_value_prim_to_sint!(i8, i16, i32, i64, isize);
impl From<f32> for PrimitiveValue {
    fn from(v: f32) -> Self {
        PrimitiveValue::F32(v)
    }
}
impl From<&PrimitiveValue> for f32 {
    fn from(v: &PrimitiveValue) -> Self {
        v.to_f32().unwrap_or_default()
    }
}
impl From<f64> for PrimitiveValue {
    fn from(v: f64) -> Self {
        PrimitiveValue::F64(v)
    }
}
impl From<&PrimitiveValue> for f64 {
    fn from(v: &PrimitiveValue) -> Self {
        v.to_f64().unwrap_or_default()
    }
}
impl From<bool> for PrimitiveValue {
    fn from(v: bool) -> Self {
        PrimitiveValue::Bool(v)
    }
}
impl From<&PrimitiveValue> for bool {
    fn from(v: &PrimitiveValue) -> Self {
        v.to_bool().unwrap_or_default()
    }
}
impl From<()> for PrimitiveValue {
    fn from(_: ()) -> Self {
        PrimitiveValue::Null
    }
}
impl From<&PrimitiveValue> for () {
    fn from(_v: &PrimitiveValue) -> Self {}
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
impl PartialEq for PrimitiveValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PrimitiveValue::String(a), PrimitiveValue::String(b)) => a == b,
            (PrimitiveValue::U64(a), PrimitiveValue::U64(b)) => a == b,
            (PrimitiveValue::I64(a), PrimitiveValue::I64(b)) => a == b,
            (PrimitiveValue::F32(a), PrimitiveValue::F32(b)) => a.to_bits() == b.to_bits(),
            (PrimitiveValue::F64(a), PrimitiveValue::F64(b)) => a.to_bits() == b.to_bits(),
            (PrimitiveValue::Bool(a), PrimitiveValue::Bool(b)) => a == b,
            (PrimitiveValue::Null, PrimitiveValue::Null) => true,
            _ => false,
        }
    }
}
impl Eq for PrimitiveValue {}
impl PartialOrd for PrimitiveValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PrimitiveValue {
    fn cmp(&self, other: &Self) -> Ordering {
        fn type_order(value: &PrimitiveValue) -> u8 {
            match value {
                PrimitiveValue::Null => 0,
                PrimitiveValue::Bool(_) => 1,
                PrimitiveValue::I64(_) => 2,
                PrimitiveValue::U64(_) => 3,
                PrimitiveValue::F32(_) => 4,
                PrimitiveValue::F64(_) => 5,
                PrimitiveValue::String(_) => 6,
            }
        }

        match (self, other) {
            (PrimitiveValue::String(a), PrimitiveValue::String(b)) => a.cmp(b),
            (PrimitiveValue::U64(a), PrimitiveValue::U64(b)) => a.cmp(b),
            (PrimitiveValue::I64(a), PrimitiveValue::I64(b)) => a.cmp(b),
            (PrimitiveValue::F32(a), PrimitiveValue::F32(b)) => a.to_bits().cmp(&b.to_bits()),
            (PrimitiveValue::F64(a), PrimitiveValue::F64(b)) => a.to_bits().cmp(&b.to_bits()),
            (PrimitiveValue::Bool(a), PrimitiveValue::Bool(b)) => a.cmp(b),
            (PrimitiveValue::Null, PrimitiveValue::Null) => Ordering::Equal,
            // Different types: Order by predefined ranking
            _ => type_order(self).cmp(&type_order(other)),
        }
    }
}
impl ProtoRead for PrimitiveValue {
    fn read(&mut self, tag: u64, pb: &mut Protobuf) {
        *self = match tag {
            1 => PrimitiveValue::String(pb.read_string()),
            2 => PrimitiveValue::F32(pb.read_varint()),
            3 => PrimitiveValue::F64(pb.read_varint()),
            5 => PrimitiveValue::U64(pb.read_varint()),
            4 | 6 => PrimitiveValue::I64(pb.read_s_varint()),
            7 => PrimitiveValue::Bool(pb.read_varint()),
            _ => PrimitiveValue::Null,
        }
    }
}
impl ProtoWrite for PrimitiveValue {
    fn write(&self, pbf: &mut Protobuf) {
        match self {
            PrimitiveValue::Null => pbf.write_field(0, Type::None),
            PrimitiveValue::String(value) => pbf.write_string_field(1, value),
            PrimitiveValue::F32(value) => pbf.write_varint_field(2, *value),
            PrimitiveValue::F64(value) => pbf.write_varint_field(3, *value),
            PrimitiveValue::U64(value) => pbf.write_varint_field(5, *value),
            PrimitiveValue::I64(value) => pbf.write_s_varint_field(6, *value),
            PrimitiveValue::Bool(value) => pbf.write_varint_field(7, *value),
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
        ValuePrimitiveType::Primitive(PrimitiveValue::String(s.into()))
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
            ValuePrimitiveType::Primitive(PrimitiveValue::String(s)) => s.into(),
            _ => "".into(),
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
                        ValuePrimitiveType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
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
                        ValuePrimitiveType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
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
                        ValuePrimitiveType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
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
                        ValuePrimitiveType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
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
            ValuePrimitiveType::Primitive(PrimitiveValue::String(v)) => {
                v.parse().unwrap_or_default()
            }
            ValuePrimitiveType::Primitive(PrimitiveValue::F32(v)) => *v,
            _ => 0.0,
        }
    }
}
impl From<ValuePrimitiveType> for f32 {
    fn from(v: ValuePrimitiveType) -> Self {
        match v {
            ValuePrimitiveType::Primitive(PrimitiveValue::String(v)) => {
                v.parse().unwrap_or_default()
            }
            ValuePrimitiveType::Primitive(PrimitiveValue::F32(v)) => v,
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
            ValuePrimitiveType::Primitive(PrimitiveValue::String(v)) => {
                v.parse().unwrap_or_default()
            }
            ValuePrimitiveType::Primitive(PrimitiveValue::F64(v)) => *v,
            _ => 0.0,
        }
    }
}
impl From<ValuePrimitiveType> for f64 {
    fn from(v: ValuePrimitiveType) -> Self {
        match v {
            ValuePrimitiveType::Primitive(PrimitiveValue::String(v)) => {
                v.parse().unwrap_or_default()
            }
            ValuePrimitiveType::Primitive(PrimitiveValue::F64(v)) => v,
            _ => 0.0,
        }
    }
}
impl From<bool> for ValuePrimitiveType {
    fn from(v: bool) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::Bool(v))
    }
}
impl From<ValuePrimitiveType> for bool {
    fn from(v: ValuePrimitiveType) -> Self {
        match v {
            ValuePrimitiveType::Primitive(PrimitiveValue::Bool(v)) => v,
            _ => false,
        }
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
    /// Returns true if the value is a primitive
    pub fn is_prim(&self) -> bool {
        matches!(self, ValueType::Primitive(_))
    }
    /// Returns the value as a vector
    pub fn to_vec(&self) -> Option<&Vec<ValuePrimitiveType>> {
        match self {
            ValueType::Array(v) => Some(v),
            _ => None,
        }
    }
    /// Returns true if the value is a vector
    pub fn is_vec(&self) -> bool {
        matches!(self, ValueType::Array(_))
    }
    /// Returns the value as a nested object
    pub fn to_nested(&self) -> Option<&Value> {
        match self {
            ValueType::Nested(v) => Some(v),
            _ => None,
        }
    }
    /// Returns true if the value is a nested object
    pub fn is_nested(&self) -> bool {
        matches!(self, ValueType::Nested(_))
    }
}
impl From<&str> for ValueType {
    fn from(s: &str) -> Self {
        ValueType::Primitive(PrimitiveValue::String(s.into()))
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
impl From<&ValueType> for ValueType {
    fn from(v: &ValueType) -> Self {
        v.clone()
    }
}
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
            _ => "".into(),
        }
    }
}
impl From<&ValueType> for String {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::String(s)) => s.into(),
            _ => "".into(),
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
                        ValueType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
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
                        ValueType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
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
                        ValueType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
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
                        ValueType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
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
            ValueType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
            ValueType::Primitive(PrimitiveValue::F32(v)) => v,
            _ => 0.0,
        }
    }
}
impl From<&ValueType> for f32 {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
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
            ValueType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
            ValueType::Primitive(PrimitiveValue::F64(v)) => v,
            _ => 0.0,
        }
    }
}
impl From<&ValueType> for f64 {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::String(v)) => v.parse().unwrap_or_default(),
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
            ValueType::Primitive(PrimitiveValue::String(v)) => v == "true",
            ValueType::Primitive(PrimitiveValue::Bool(v)) => v,
            _ => false,
        }
    }
}
impl From<&ValueType> for bool {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Primitive(PrimitiveValue::String(v)) => v == "true",
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

// Geometry

impl From<&Point> for ValueType {
    fn from(v: &Point) -> Self {
        ValueType::Array(vec![v.0.into(), v.1.into()])
    }
}
impl From<&ValueType> for Point {
    fn from(v: &ValueType) -> Self {
        match v {
            ValueType::Array(arr) => {
                if let Some(x) = arr.first()
                    && let Some(y) = arr.get(1)
                {
                    Point(
                        x.to_prim()
                            .unwrap_or(&PrimitiveValue::default())
                            .to_f64()
                            .unwrap_or_default(),
                        y.to_prim()
                            .unwrap_or(&PrimitiveValue::default())
                            .to_f64()
                            .unwrap_or_default(),
                    )
                } else {
                    Point::default()
                }
            }
            _ => Point::default(),
        }
    }
}

// Serde compatibility for testing

impl From<&serde_json::Value> for Value {
    fn from(val: &serde_json::Value) -> Self {
        let mut res = Value::new();
        if let serde_json::Value::Object(o) = val {
            for (k, v) in o.iter() {
                res.insert(k.clone(), v.into());
            }
        }

        res
    }
}
impl From<&serde_json::Value> for ValueType {
    fn from(val: &serde_json::Value) -> Self {
        match val {
            serde_json::Value::Null => ValueType::Primitive(PrimitiveValue::Null),
            serde_json::Value::Bool(b) => ValueType::Primitive(PrimitiveValue::Bool(*b)),
            serde_json::Value::Number(num) => {
                ValueType::Primitive(PrimitiveValue::F64(num.as_f64().unwrap_or_default()))
            }
            serde_json::Value::String(s) => ValueType::Primitive(PrimitiveValue::String(s.clone())),
            serde_json::Value::Array(values) => {
                ValueType::Array(values.iter().map(Into::into).collect())
            }
            serde_json::Value::Object(map) => ValueType::Nested(map.into()),
        }
    }
}
impl From<&serde_json::Value> for ValuePrimitiveType {
    fn from(val: &serde_json::Value) -> Self {
        match val {
            serde_json::Value::Null => ValuePrimitiveType::Primitive(PrimitiveValue::Null),
            serde_json::Value::Bool(b) => ValuePrimitiveType::Primitive(PrimitiveValue::Bool(*b)),
            serde_json::Value::Number(num) => {
                ValuePrimitiveType::Primitive(PrimitiveValue::F64(num.as_f64().unwrap_or_default()))
            }
            serde_json::Value::String(s) => {
                ValuePrimitiveType::Primitive(PrimitiveValue::String(s.clone()))
            }
            _ => ValuePrimitiveType::Primitive(PrimitiveValue::Null),
        }
    }
}
impl From<&serde_json::Map<String, serde_json::Value>> for Value {
    fn from(val: &serde_json::Map<String, serde_json::Value>) -> Self {
        let mut res = Value::new();
        for (k, v) in val.iter() {
            res.insert(k.clone(), v.into());
        }
        res
    }
}
