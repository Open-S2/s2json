use crate::{
    Map, PrimitiveShape, PrimitiveShapeType, PrimitiveValue, Shape, ShapeType, Value,
    ValuePrimitiveType, ValueType,
};
use alloc::{string::String, vec};

// ? Primitive Shape

impl PrimitiveShape {
    /// returns true if the shape is a number type
    pub fn is_number(&self) -> bool {
        matches!(
            self,
            PrimitiveShape::F64 | PrimitiveShape::F32 | PrimitiveShape::I64 | PrimitiveShape::U64
        )
    }

    /// returns true if two shapes are the same. Numeric types are considered the same.
    pub fn matching_shape(&self, other: &PrimitiveShape) -> bool {
        self == other || self.is_number() == other.is_number()
    }

    /// returns the highest order number type
    pub fn get_highest_order_number(
        type_a: &PrimitiveShape,
        type_b: &PrimitiveShape,
    ) -> PrimitiveShape {
        if *type_a == PrimitiveShape::F64 || *type_b == PrimitiveShape::F64 {
            PrimitiveShape::F64
        } else if *type_a == PrimitiveShape::F32 || *type_b == PrimitiveShape::F32 {
            PrimitiveShape::F32
        } else if *type_a == PrimitiveShape::I64 || *type_b == PrimitiveShape::I64 {
            PrimitiveShape::I64
        } else {
            PrimitiveShape::U64
        }
    }

    fn merge(&mut self, other: &Self) {
        if self.is_number() && other.is_number() {
            *self = Self::get_highest_order_number(self, other);
        } else if !self.matching_shape(other) {
            panic!("shape mismatch: {:?} vs {:?}", self, other);
        }
        // othewrise, do nothing
    }
}
impl From<&PrimitiveShape> for usize {
    fn from(shape: &PrimitiveShape) -> Self {
        match shape {
            PrimitiveShape::String => 0,
            PrimitiveShape::U64 => 1,
            PrimitiveShape::I64 => 2,
            PrimitiveShape::F32 => 3,
            PrimitiveShape::F64 => 4,
            PrimitiveShape::Bool => 5,
            PrimitiveShape::Null => 6,
        }
    }
}
impl From<usize> for PrimitiveShape {
    fn from(num: usize) -> Self {
        match num {
            0 => PrimitiveShape::String,
            1 => PrimitiveShape::U64,
            2 => PrimitiveShape::I64,
            3 => PrimitiveShape::F32,
            4 => PrimitiveShape::F64,
            5 => PrimitiveShape::Bool,
            6 => PrimitiveShape::Null,
            _ => panic!("unknown value: {}", num),
        }
    }
}
impl From<&PrimitiveValue> for PrimitiveShape {
    fn from(val: &PrimitiveValue) -> Self {
        match val {
            PrimitiveValue::String(_) => PrimitiveShape::String,
            PrimitiveValue::U64(_) => PrimitiveShape::U64,
            PrimitiveValue::I64(_) => PrimitiveShape::I64,
            PrimitiveValue::F32(_) => PrimitiveShape::F32,
            PrimitiveValue::F64(_) => PrimitiveShape::F64,
            PrimitiveValue::Bool(_) => PrimitiveShape::Bool,
            PrimitiveValue::Null => PrimitiveShape::Null,
        }
    }
}

// ? Shape Primitive Type

impl From<&ValuePrimitiveType> for PrimitiveShapeType {
    fn from(val: &ValuePrimitiveType) -> Self {
        match val {
            ValuePrimitiveType::Primitive(prim) => PrimitiveShapeType::Primitive(prim.into()),
            ValuePrimitiveType::NestedPrimitive(nested) => {
                let mut nested_map = Map::new();
                for (key, value) in nested.iter() {
                    nested_map.insert(key.into(), value.into());
                }
                PrimitiveShapeType::NestedPrimitive(nested_map)
            }
        }
    }
}
impl PrimitiveShapeType {
    fn merge(&mut self, other: &Self) {
        match (self, other) {
            (
                PrimitiveShapeType::Primitive(self_prim),
                PrimitiveShapeType::Primitive(other_prim),
            ) => {
                self_prim.merge(other_prim);
            }
            (
                PrimitiveShapeType::NestedPrimitive(self_nested),
                PrimitiveShapeType::NestedPrimitive(other_nested),
            ) => {
                for (key, value) in other_nested.iter() {
                    if self_nested.contains_key(key) {
                        self_nested.get_mut(key).unwrap().merge(value);
                    } else {
                        self_nested.insert(key.clone(), value.clone());
                    }
                }
            }
            _ => panic!("shape mismatch"),
        }
    }
}

// ? Shape Type

impl Default for ShapeType {
    fn default() -> Self {
        ShapeType::Primitive(PrimitiveShape::Null)
    }
}
impl From<&ValueType> for ShapeType {
    fn from(val: &ValueType) -> Self {
        match val {
            ValueType::Primitive(prim) => ShapeType::Primitive(prim.into()),
            ValueType::Nested(nested) => {
                let mut nested_map: Map<String, ShapeType> = Map::new();
                for (key, value) in nested.iter() {
                    nested_map.insert(key.into(), value.into());
                }
                ShapeType::Nested(nested_map)
            }
            ValueType::Array(array) => {
                let validated = validate_types(array);
                ShapeType::Array(vec![validated])
            }
        }
    }
}
impl ShapeType {
    fn merge(&mut self, other: &Self) {
        match (self, other) {
            (Self::Primitive(a), Self::Primitive(b)) => a.merge(b),
            (Self::Array(a), Self::Array(b)) => {
                a.first_mut().unwrap().merge(b.first().unwrap());
            }
            (Self::Nested(a), Self::Nested(b)) => a.merge(b),
            _ => panic!("Can't merge"),
        };
    }
}

// ? Shape

impl From<&Value> for Shape {
    fn from(val: &Value) -> Self {
        let mut shape = Shape::new();
        for (key, value) in val.iter() {
            shape.insert(key.into(), value.into());
        }

        shape
    }
}
impl From<&[Value]> for Shape {
    fn from(val: &[Value]) -> Self {
        let mut shape = Shape::new();
        for v in val {
            shape.merge(&(v.into()));
        }
        shape
    }
}
impl Shape {
    /// Merge two shapes
    pub fn merge(&mut self, other: &Self) {
        for (key, value) in other.iter() {
            self.entry(key.clone())
                .and_modify(|val| val.merge(value))
                .or_insert_with(|| value.clone());
        }
    }
}

//? Primitive Value

impl PrimitiveValue {
    /// Get the default primitive value from a shape
    pub fn default_from_shape(shape: &PrimitiveShape) -> Self {
        match shape {
            PrimitiveShape::String => PrimitiveValue::String(String::new()),
            PrimitiveShape::U64 => PrimitiveValue::U64(0),
            PrimitiveShape::I64 => PrimitiveValue::I64(0),
            PrimitiveShape::F32 => PrimitiveValue::F32(0.0),
            PrimitiveShape::F64 => PrimitiveValue::F64(0.0),
            PrimitiveShape::Bool => PrimitiveValue::Bool(false),
            PrimitiveShape::Null => PrimitiveValue::Null,
        }
    }

    fn matching_shape(&self, other: &PrimitiveValue) -> bool {
        matches!(
            (self, other),
            (PrimitiveValue::String(_), PrimitiveValue::String(_))
                | (PrimitiveValue::U64(_), PrimitiveValue::U64(_))
                | (PrimitiveValue::I64(_), PrimitiveValue::I64(_))
                | (PrimitiveValue::F32(_), PrimitiveValue::F32(_))
                | (PrimitiveValue::F64(_), PrimitiveValue::F64(_))
                | (PrimitiveValue::Bool(_), PrimitiveValue::Bool(_))
                | (PrimitiveValue::Null, PrimitiveValue::Null)
        )
    }
}
impl From<&PrimitiveValue> for PrimitiveValue {
    fn from(mval: &PrimitiveValue) -> Self {
        match mval {
            PrimitiveValue::String(string) => PrimitiveValue::String(string.clone()),
            PrimitiveValue::U64(usigned) => PrimitiveValue::U64(*usigned),
            PrimitiveValue::I64(signed) => PrimitiveValue::I64(*signed),
            PrimitiveValue::F32(float) => PrimitiveValue::F32(*float),
            PrimitiveValue::F64(double) => PrimitiveValue::F64(*double),
            PrimitiveValue::Bool(boolean) => PrimitiveValue::Bool(*boolean),
            PrimitiveValue::Null => PrimitiveValue::Null,
        }
    }
}

// ? Value Primitive Type

impl ValuePrimitiveType {
    fn same_nested(&self, nested: &Map<String, PrimitiveValue>) -> bool {
        match self {
            ValuePrimitiveType::Primitive(_) => false,
            ValuePrimitiveType::NestedPrimitive(val) => {
                for (key, val) in val.iter() {
                    if !val.matching_shape(nested.get(key).unwrap()) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

// ? ValueType

impl ValueType {
    /// Get the default value type from a shape
    pub fn default_from_shape(shape: &ShapeType) -> Self {
        match shape {
            ShapeType::Primitive(shape) => {
                ValueType::Primitive(PrimitiveValue::default_from_shape(shape))
            }
            ShapeType::Array(_) => ValueType::Array(vec![]),
            ShapeType::Nested(shape) => ValueType::Nested(Value::default_from_shape(shape)),
        }
    }
}
impl From<&PrimitiveValue> for ValueType {
    fn from(mval: &PrimitiveValue) -> Self {
        ValueType::Primitive(mval.into())
    }
}
impl From<&ValueType> for PrimitiveValue {
    fn from(val: &ValueType) -> Self {
        match val {
            ValueType::Primitive(val) => val.into(),
            _ => PrimitiveValue::Null,
        }
    }
}

// ? Value

impl Value {
    /// Get the default value from a shape
    pub fn default_from_shape(shape: &Shape) -> Self {
        let mut value = Value::new();
        for (key, shape_type) in shape.iter() {
            value.insert(key.into(), ValueType::default_from_shape(shape_type));
        }
        value
    }
}

//? The Following are utility functions when the user doesn't pre-define the Properties/M-Value
//? Shapes to store:

/// This is primarily to check if the type is a primitive.
/// If the primitive is a number, find the "depth", the most complex is f64, then i64, then u64.
/// Otherwise, if the primitives don't match, throw an error.
/// If the type is NOT a primitive, ensure that all types in the array match
/// returns - a single type from the list to validate the correct type to be parsed from values later
pub fn validate_types(types: &[ValuePrimitiveType]) -> PrimitiveShapeType {
    match types.first() {
        Some(ValuePrimitiveType::Primitive(primitive)) => {
            let mut base: PrimitiveShape = primitive.into();
            let is_number = base.is_number();
            for t in types {
                match t {
                    ValuePrimitiveType::Primitive(t_prim) => {
                        let prim_shape = t_prim.into();
                        if !base.matching_shape(&prim_shape) {
                            panic!("All types must be the same");
                        } else if is_number {
                            base = PrimitiveShape::get_highest_order_number(&base, &prim_shape);
                        }
                        // otherwise do nothing
                    }
                    _ => panic!("All types must be the same"),
                }
            }

            PrimitiveShapeType::Primitive(base)
        }
        Some(ValuePrimitiveType::NestedPrimitive(nested)) => {
            // iterate and check if each following types match
            for t in types[1..].iter() {
                if !t.same_nested(nested) {
                    panic!("All types must be the same");
                }
            }

            (&ValuePrimitiveType::NestedPrimitive(nested.clone())).into()
        }
        None => PrimitiveShapeType::Primitive(PrimitiveShape::Null),
    }
}
