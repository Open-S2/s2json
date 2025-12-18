use libm::round;

use crate::{
    BBOX, BBox, BBox3D, GetXY, GetZ, JSONProperties, JSONValue, MValue, Point, Point3D,
    PointOrPoint3D, PrimitiveValue, STPoint, Value, ValuePrimitive, ValuePrimitiveType, ValueType,
    VectorPoint,
};

/// Easy access to interpolation tooling for All S2JSON Core Types
pub trait Interpolate {
    /// Interpolate between two of the same type
    fn interpolate(&self, other: &Self, t: f64) -> Self;
}

// Base types

impl Interpolate for () {
    fn interpolate(&self, _: &Self, _: f64) -> Self {}
}
macro_rules! impl_interpolate {
    // truncate
    ($($t:ty),* $(,)?) => {
        $(
            impl Interpolate for $t {
                fn interpolate(&self, other: &Self, t: f64) -> Self {
                    ((*self as f64) + ((*other as f64) - (*self as f64)) * t) as $t
                }
            }
        )*
    };

    // round
    (round: $($t:ty),* $(,)?) => {
        $(
            impl Interpolate for $t {
                fn interpolate(&self, other: &Self, t: f64) -> Self {
                    round((*self as f64) + ((*other as f64) - (*self as f64)) * t) as $t
                }
            }
        )*
    };
}
impl_interpolate!(f32);
impl_interpolate!(round: usize, u8, u16, u32, u64, isize, i8, i16, i32, i64);
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

// S2JSON Core Types

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

// Value

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

// JSONProperties

impl Interpolate for JSONProperties {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        let mut res = JSONProperties::new();

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
impl Interpolate for JSONValue {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        match (self, other) {
            (JSONValue::Primitive(a), JSONValue::Primitive(b)) => {
                JSONValue::Primitive(a.interpolate(b, t))
            }
            (JSONValue::Array(a), JSONValue::Array(b)) => {
                JSONValue::Array(a.iter().zip(b.iter()).map(|(a, b)| a.interpolate(b, t)).collect())
            }
            (JSONValue::Object(a), JSONValue::Object(b)) => JSONValue::Object(a.interpolate(b, t)),
            _ => JSONValue::Primitive(PrimitiveValue::Null),
        }
    }
}

// BBox

impl<T: Interpolate> Interpolate for BBox<T> {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        BBox {
            left: self.left.interpolate(&other.left, t),
            bottom: self.bottom.interpolate(&other.bottom, t),
            right: self.right.interpolate(&other.right, t),
            top: self.top.interpolate(&other.top, t),
        }
    }
}
impl<T: Interpolate> Interpolate for BBox3D<T> {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        BBox3D {
            left: self.left.interpolate(&other.left, t),
            bottom: self.bottom.interpolate(&other.bottom, t),
            right: self.right.interpolate(&other.right, t),
            top: self.top.interpolate(&other.top, t),
            near: self.near.interpolate(&other.near, t),
            far: self.far.interpolate(&other.far, t),
        }
    }
}
impl Interpolate for BBOX {
    fn interpolate(&self, other: &Self, t: f64) -> Self {
        match (self, other) {
            (BBOX::BBox(a), BBOX::BBox(b)) => a.interpolate(b, t).into(),
            (BBOX::BBox3D(a), BBOX::BBox3D(b)) => a.interpolate(b, t).into(),
            // Ensure that BBox and BBox3D are ordered correctly
            (BBOX::BBox(_), BBOX::BBox3D(_)) => self.clone(),
            (BBOX::BBox3D(_), BBOX::BBox(_)) => self.clone(),
        }
    }
}
