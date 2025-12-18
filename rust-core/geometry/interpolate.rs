use libm::round;

use crate::{
    GetXY, GetZ, MValue, Point, Point3D, PointOrPoint3D, PrimitiveValue, STPoint, Value,
    ValuePrimitive, ValuePrimitiveType, ValueType, VectorPoint,
};

/// Easy access to interpolation tooling for All S2JSON Core Types
pub trait Interpolate {
    /// Interpolate between two of the same type
    fn interpolate(&self, other: &Self, t: f64) -> Self;
}
impl Interpolate for u64 {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        round((*self as f64) + ((*other as f64) - (*self as f64)) * t) as u64
    }
}
impl Interpolate for i64 {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        round((*self as f64) + ((*other as f64) - (*self as f64)) * t) as i64
    }
}
impl Interpolate for f32 {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        ((*self as f64) + ((*other as f64) - (*self as f64)) * t) as f32
    }
}
impl Interpolate for f64 {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        self + (other - self) * t
    }
}
impl<M: Interpolate> Interpolate for Option<M> {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        match (self, other) {
            (Some(a), Some(b)) => Some(a.interpolate(b, t)),
            _ => None,
        }
    }
}
impl Interpolate for Point {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        Point(self.x().interpolate(&other.x(), t), self.y().interpolate(&other.y(), t))
    }
}
impl Interpolate for Point3D {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        Point3D(
            self.0.interpolate(&other.0, t),
            self.1.interpolate(&other.1, t),
            self.2.interpolate(&other.2, t),
        )
    }
}
impl Interpolate for PointOrPoint3D {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        PointOrPoint3D(
            self.x().interpolate(&other.x(), t),
            self.y().interpolate(&other.y(), t),
            self.z().interpolate(&other.z(), t),
        )
    }
}
impl<M: Interpolate> Interpolate for STPoint<M> {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        Self {
            face: self.face,
            s: self.s.interpolate(&other.s, t),
            t: self.t.interpolate(&other.t, t),
            z: self.z.interpolate(&other.z, t),
            m: self.m.interpolate(&other.m, t),
        }
    }
}
impl<M: Interpolate + Clone> Interpolate for VectorPoint<M> {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        VectorPoint {
            x: self.x.interpolate(&other.x, t),
            y: self.y.interpolate(&other.y, t),
            z: self.z.interpolate(&other.z, t),
            m: self.m.interpolate(&other.m, t),
            t: self.t.interpolate(&other.t, t),
        }
    }
}
impl Interpolate for Value {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        let mut res = MValue::new();

        // pairs are interpolated
        for (a, b) in self.iter().zip(other.iter()) {
            res.insert(a.0.clone(), a.1.interpolate(b.1, t));
        }
        // keys not in other are just added
        for (k, v) in self.iter() {
            if !res.contains_key(k) {
                res.insert(k.clone(), v.clone());
            }
        }
        // keys not in self are just added
        for (k, v) in other.iter() {
            if !res.contains_key(k) {
                res.insert(k.clone(), v.clone());
            }
        }

        res
    }
}
impl Interpolate for ValueType {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        match (self, other) {
            (ValueType::Primitive(a), ValueType::Primitive(b)) => {
                ValueType::Primitive(a.interpolate(b, t))
            }
            (ValueType::Array(a), ValueType::Array(b)) => {
                ValueType::Array(a.iter().zip(b.iter()).map(|(a, b)| a.interpolate(b, t)).collect())
            }
            (ValueType::Nested(a), ValueType::Nested(b)) => ValueType::Nested(a.interpolate(b, t)),
            _ => ValueType::Primitive(PrimitiveValue::Null),
        }
    }
}
impl Interpolate for ValuePrimitiveType {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        match (self, other) {
            (ValuePrimitiveType::Primitive(a), ValuePrimitiveType::Primitive(b)) => {
                ValuePrimitiveType::Primitive(a.interpolate(b, t))
            }
            (ValuePrimitiveType::NestedPrimitive(a), ValuePrimitiveType::NestedPrimitive(b)) => {
                ValuePrimitiveType::NestedPrimitive(a.interpolate(b, t))
            }
            _ => ValuePrimitiveType::Primitive(PrimitiveValue::Null),
        }
    }
}
impl Interpolate for ValuePrimitive {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        let mut res = ValuePrimitive::new();

        // pairs are interpolated
        for (a, b) in self.iter().zip(other.iter()) {
            res.insert(a.0.clone(), a.1.interpolate(b.1, t));
        }
        // keys not in other are just added
        for (k, v) in self.iter() {
            if !res.contains_key(k) {
                res.insert(k.clone(), v.clone());
            }
        }
        // keys not in self are just added
        for (k, v) in other.iter() {
            if !res.contains_key(k) {
                res.insert(k.clone(), v.clone());
            }
        }

        res
    }
}
impl Interpolate for PrimitiveValue {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        match (self, other) {
            (PrimitiveValue::U64(a), PrimitiveValue::U64(b)) => {
                PrimitiveValue::U64(a.interpolate(b, t))
            }
            (PrimitiveValue::I64(a), PrimitiveValue::I64(b)) => {
                PrimitiveValue::I64(a.interpolate(b, t))
            }
            (PrimitiveValue::F32(a), PrimitiveValue::F32(b)) => {
                PrimitiveValue::F32(a.interpolate(b, t))
            }
            (PrimitiveValue::F64(a), PrimitiveValue::F64(b)) => {
                PrimitiveValue::F64(a.interpolate(b, t))
            }
            (PrimitiveValue::String(a), PrimitiveValue::String(b)) => {
                PrimitiveValue::String(if t <= 0.5 { a.clone() } else { b.clone() })
            }
            (PrimitiveValue::Bool(a), PrimitiveValue::Bool(b)) => {
                PrimitiveValue::Bool(if t <= 0.5 { *a } else { *b })
            }
            _ => self.clone(),
        }
    }
}
