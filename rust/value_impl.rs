use alloc::{string::String, string::ToString, vec::Vec};

use libm::round;

use crate::*;

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
impl From<u64> for ValuePrimitiveType {
    fn from(v: u64) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::U64(v))
    }
}
impl From<i64> for ValuePrimitiveType {
    fn from(v: i64) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::I64(v))
    }
}
impl From<f32> for ValuePrimitiveType {
    fn from(v: f32) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::F32(v))
    }
}
impl From<f64> for ValuePrimitiveType {
    fn from(v: f64) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::F64(v))
    }
}
impl From<bool> for ValuePrimitiveType {
    fn from(v: bool) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::Bool(v))
    }
}
impl From<()> for ValuePrimitiveType {
    fn from(_: ()) -> Self {
        ValuePrimitiveType::Primitive(PrimitiveValue::Null)
    }
}
impl From<PrimitiveValue> for ValuePrimitiveType {
    fn from(v: PrimitiveValue) -> Self {
        ValuePrimitiveType::Primitive(v)
    }
}
impl From<ValuePrimitive> for ValuePrimitiveType {
    fn from(v: ValuePrimitive) -> Self {
        ValuePrimitiveType::NestedPrimitive(v)
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
impl From<String> for ValueType {
    fn from(s: String) -> Self {
        ValueType::Primitive(PrimitiveValue::String(s))
    }
}
impl From<u64> for ValueType {
    fn from(v: u64) -> Self {
        ValueType::Primitive(PrimitiveValue::U64(v))
    }
}
impl From<i64> for ValueType {
    fn from(v: i64) -> Self {
        ValueType::Primitive(PrimitiveValue::I64(v))
    }
}
impl From<f32> for ValueType {
    fn from(v: f32) -> Self {
        ValueType::Primitive(PrimitiveValue::F32(v))
    }
}
impl From<f64> for ValueType {
    fn from(v: f64) -> Self {
        ValueType::Primitive(PrimitiveValue::F64(v))
    }
}
impl From<bool> for ValueType {
    fn from(v: bool) -> Self {
        ValueType::Primitive(PrimitiveValue::Bool(v))
    }
}
impl From<()> for ValueType {
    fn from(_: ()) -> Self {
        ValueType::Primitive(PrimitiveValue::Null)
    }
}
impl<T> From<Vec<T>> for ValueType
where
    T: Into<ValuePrimitiveType>,
{
    fn from(v: Vec<T>) -> Self {
        ValueType::Array(v.into_iter().map(Into::into).collect())
    }
}
impl From<Value> for ValueType {
    fn from(v: Value) -> Self {
        ValueType::Nested(v)
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

#[cfg(test)]
mod tests {
    use alloc::vec;

    use crate::{MValue, MValueCompatible, VectorPoint};

    use super::*;

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
        impl From<MValue> for Rgba {
            fn from(mvalue: MValue) -> Self {
                let r: f64 = mvalue.get("r").unwrap().to_prim().unwrap().to_f64().unwrap();
                let g = mvalue.get("g").unwrap().to_prim().unwrap().to_f64().unwrap();
                let b = mvalue.get("b").unwrap().to_prim().unwrap().to_f64().unwrap();
                let a = mvalue.get("a").unwrap().to_prim().unwrap().to_f64().unwrap();
                Rgba::new(r, g, b, a)
            }
        }

        let rgba = Rgba::new(0.1, 0.2, 0.3, 0.4);
        let rgba_mvalue: MValue = rgba.into();
        assert_eq!(
            rgba_mvalue,
            MValue::from([
                ("r".into(), ValueType::Primitive(PrimitiveValue::F64(0.1))),
                ("g".into(), ValueType::Primitive(PrimitiveValue::F64(0.2))),
                ("b".into(), ValueType::Primitive(PrimitiveValue::F64(0.3))),
                ("a".into(), ValueType::Primitive(PrimitiveValue::F64(0.4))),
            ])
        );
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
}
