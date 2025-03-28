#[cfg(test)]
mod tests {
    extern crate alloc;

    use alloc::vec;
    use s2json_core::*;
    use s2json_derive::{
        JSONProperties, MValue, MValueCompatible as MValueDerive, Properties, ValuePrimitive,
    };
    use serde::{Deserialize, Serialize};

    #[test]
    fn basic_test() {
        #[derive(Properties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub name: String,
            pub value: u32,
        }

        let test_struct = TestStruct { name: "example".into(), value: 42 };

        let mvalue: MValue = test_struct.clone().into(); // Ensure this method exists
                                                         // println!("{:?}", mvalue); // Debug output
        assert_eq!(
            mvalue,
            Value::from([
                ("name".into(), ValueType::Primitive(PrimitiveValue::String("example".into()))),
                ("value".into(), ValueType::Primitive(PrimitiveValue::U64(42))),
            ])
        );

        let back_to_struct: TestStruct = mvalue.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn unsigned_test() {
        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: u8,
            pub b: u16,
            pub c: u32,
            pub d: u64,
            pub e: usize,
        }

        let test_struct = TestStruct { a: 1, b: 2, c: 3, d: 4, e: 5 };

        let mvalue: MValue = test_struct.clone().into(); // Ensure this method exists
                                                         // println!("{:?}", mvalue); // Debug output
        assert_eq!(
            mvalue,
            Value::from([
                ("a".into(), ValueType::Primitive(PrimitiveValue::U64(1))),
                ("b".into(), ValueType::Primitive(PrimitiveValue::U64(2))),
                ("c".into(), ValueType::Primitive(PrimitiveValue::U64(3))),
                ("d".into(), ValueType::Primitive(PrimitiveValue::U64(4))),
                ("e".into(), ValueType::Primitive(PrimitiveValue::U64(5))),
            ])
        );

        let back_to_struct: TestStruct = mvalue.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn signed_test() {
        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: i8,
            pub b: i16,
            pub c: i32,
            pub d: i64,
            pub e: isize,
        }

        let test_struct = TestStruct { a: -1, b: 2, c: -3, d: 4, e: -5 };

        let mvalue: MValue = test_struct.clone().into(); // Ensure this method exists
                                                         // println!("{:?}", mvalue); // Debug output
        assert_eq!(
            mvalue,
            Value::from([
                ("a".into(), ValueType::Primitive(PrimitiveValue::I64(-1))),
                ("b".into(), ValueType::Primitive(PrimitiveValue::I64(2))),
                ("c".into(), ValueType::Primitive(PrimitiveValue::I64(-3))),
                ("d".into(), ValueType::Primitive(PrimitiveValue::I64(4))),
                ("e".into(), ValueType::Primitive(PrimitiveValue::I64(-5))),
            ])
        );

        let back_to_struct: TestStruct = mvalue.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn float_test() {
        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: f32,
            pub b: f64,
        }

        let test_struct = TestStruct { a: 1.0, b: 2.0 };

        let mvalue: MValue = test_struct.clone().into(); // Ensure this method exists
                                                         // println!("{:?}", mvalue); // Debug output
        assert_eq!(
            mvalue,
            Value::from([
                ("a".into(), ValueType::Primitive(PrimitiveValue::F32(1.0))),
                ("b".into(), ValueType::Primitive(PrimitiveValue::F64(2.0))),
            ])
        );

        let back_to_struct: TestStruct = mvalue.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn bool_test() {
        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: bool,
        }

        let test_struct = TestStruct { a: true };

        let mvalue: MValue = test_struct.clone().into(); // Ensure this method exists
                                                         // println!("{:?}", mvalue); // Debug output
        assert_eq!(
            mvalue,
            Value::from([("a".into(), ValueType::Primitive(PrimitiveValue::Bool(true)))]),
        );

        let back_to_struct: TestStruct = mvalue.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn nested_object_test() {
        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct NestedStruct {
            a: String,
            b: u32,
        }
        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: NestedStruct,
            pub b: u32,
        }

        let test_struct = TestStruct { a: NestedStruct { a: "a".into(), b: 1 }, b: 2 };

        let mvalue: MValue = test_struct.clone().into(); // Ensure this method exists
                                                         // println!("{:?}", mvalue); // Debug output
        assert_eq!(
            mvalue,
            Value::from([
                (
                    "a".into(),
                    ValueType::Nested(Value::from([
                        ("a".into(), ValueType::Primitive(PrimitiveValue::String("a".into()))),
                        ("b".into(), ValueType::Primitive(PrimitiveValue::U64(1))),
                    ])),
                ),
                ("b".into(), ValueType::Primitive(PrimitiveValue::U64(2))),
            ])
        );

        let back_to_struct: TestStruct = mvalue.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn test_option() {
        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: Option<u32>,
        }

        let test_struct = TestStruct { a: Some(1) };

        let mvalue: MValue = test_struct.clone().into(); // Ensure this method exists
                                                         // println!("{:?}", mvalue); // Debug output
        assert_eq!(
            mvalue,
            Value::from([("a".into(), ValueType::Primitive(PrimitiveValue::U64(1)))]),
        );

        let back_to_struct: TestStruct = mvalue.into();
        assert_eq!(back_to_struct, test_struct);

        let test_struct = TestStruct { a: None };

        let mvalue: MValue = test_struct.clone().into(); // Ensure this method exists
                                                         // println!("{:?}", mvalue); // Debug output
        assert_eq!(mvalue, Value::from([("a".into(), ValueType::Primitive(PrimitiveValue::Null))]));

        let back_to_struct: TestStruct = mvalue.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn test_m_value_nested() {
        #[derive(MValue, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: MValue,
        }

        let test_struct = TestStruct {
            a: MValue::from([("b".into(), ValueType::Primitive(PrimitiveValue::U64(1)))]),
        };

        let mvalue: MValue = test_struct.clone().into(); // Ensure this method exists
                                                         // println!("{:?}", mvalue); // Debug output
        assert_eq!(
            mvalue,
            Value::from([(
                "a".into(),
                ValueType::Nested(Value::from([(
                    "b".into(),
                    ValueType::Primitive(PrimitiveValue::U64(1))
                )]))
            )]),
        );

        let back_to_struct: TestStruct = mvalue.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn nested_object_optional_test() {
        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct NestedStruct {
            a: String,
            b: u32,
        }
        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: Option<NestedStruct>,
            pub b: u32,
        }

        let test_struct = TestStruct { a: Some(NestedStruct { a: "a".into(), b: 1 }), b: 2 };

        let mvalue: MValue = test_struct.clone().into(); // Ensure this method exists
                                                         // println!("{:?}", mvalue); // Debug output
        assert_eq!(
            mvalue,
            Value::from([
                (
                    "a".into(),
                    ValueType::Nested(Value::from([
                        ("a".into(), ValueType::Primitive(PrimitiveValue::String("a".into()))),
                        ("b".into(), ValueType::Primitive(PrimitiveValue::U64(1))),
                    ])),
                ),
                ("b".into(), ValueType::Primitive(PrimitiveValue::U64(2))),
            ])
        );

        let back_to_struct: TestStruct = mvalue.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn vec_test() {
        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: Vec<String>,
            pub b: Vec<u32>,
            pub c: Vec<i32>,
            pub d: Option<Vec<f32>>,
        }
    }

    #[test]
    fn generics_test() {
        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct<T: MValueCompatible = MValue> {
            pub a: Option<T>,
        }

        let test_struct = TestStruct {
            a: Some(MValue::from([("b".into(), ValueType::Primitive(PrimitiveValue::U64(1)))])),
        };

        let mvalue: MValue = test_struct.clone().into(); // Ensure this method exists

        assert_eq!(
            mvalue,
            Value::from([(
                "a".into(),
                ValueType::Nested(Value::from([(
                    "b".into(),
                    ValueType::Primitive(PrimitiveValue::U64(1))
                )]))
            )]),
        );

        let back_to_struct: TestStruct = mvalue.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn basic_json_test() {
        #[derive(JSONProperties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub name: String,
            pub value: u32,
        }

        let test_struct = TestStruct { name: "example".into(), value: 42 };

        let json: JSONProperties = test_struct.clone().into(); // Ensure this method exists
                                                               // println!("{:?}", json); // Debug output
        assert_eq!(
            json,
            JSONProperties::from([
                ("name".into(), JSONValue::Primitive(PrimitiveValue::String("example".into()))),
                ("value".into(), JSONValue::Primitive(PrimitiveValue::U64(42))),
            ])
        );

        let back_to_struct: TestStruct = json.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn json_unsigned_test() {
        #[derive(JSONProperties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: u8,
            pub b: u16,
            pub c: u32,
            pub d: u64,
            pub e: usize,
        }

        let test_struct = TestStruct { a: 1, b: 2, c: 3, d: 4, e: 5 };

        let json: JSONProperties = test_struct.clone().into(); // Ensure this method exists
                                                               // println!("{:?}", json); // Debug output
        assert_eq!(
            json,
            JSONProperties::from([
                ("a".into(), JSONValue::Primitive(PrimitiveValue::U64(1))),
                ("b".into(), JSONValue::Primitive(PrimitiveValue::U64(2))),
                ("c".into(), JSONValue::Primitive(PrimitiveValue::U64(3))),
                ("d".into(), JSONValue::Primitive(PrimitiveValue::U64(4))),
                ("e".into(), JSONValue::Primitive(PrimitiveValue::U64(5))),
            ])
        );

        let back_to_struct: TestStruct = json.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn json_signed_test() {
        #[derive(JSONProperties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: i8,
            pub b: i16,
            pub c: i32,
            pub d: i64,
            pub e: isize,
        }

        let test_struct = TestStruct { a: -1, b: 2, c: -3, d: 4, e: -5 };

        let json: JSONProperties = test_struct.clone().into(); // Ensure this method exists
                                                               // println!("{:?}", json); // Debug output
        assert_eq!(
            json,
            JSONProperties::from([
                ("a".into(), JSONValue::Primitive(PrimitiveValue::I64(-1))),
                ("b".into(), JSONValue::Primitive(PrimitiveValue::I64(2))),
                ("c".into(), JSONValue::Primitive(PrimitiveValue::I64(-3))),
                ("d".into(), JSONValue::Primitive(PrimitiveValue::I64(4))),
                ("e".into(), JSONValue::Primitive(PrimitiveValue::I64(-5))),
            ])
        );

        let back_to_struct: TestStruct = json.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn json_float_test() {
        #[derive(JSONProperties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: f32,
            pub b: f64,
        }

        let test_struct = TestStruct { a: 1.0, b: 2.0 };

        let json: JSONProperties = test_struct.clone().into(); // Ensure this method exists
                                                               // println!("{:?}", json); // Debug output
        assert_eq!(
            json,
            JSONProperties::from([
                ("a".into(), JSONValue::Primitive(PrimitiveValue::F32(1.0))),
                ("b".into(), JSONValue::Primitive(PrimitiveValue::F64(2.0))),
            ])
        );

        let back_to_struct: TestStruct = json.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn json_bool_test() {
        #[derive(JSONProperties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: bool,
        }

        let test_struct = TestStruct { a: true };

        let json: JSONProperties = test_struct.clone().into(); // Ensure this method exists
                                                               // println!("{:?}", json); // Debug output
        assert_eq!(
            json,
            JSONProperties::from([("a".into(), JSONValue::Primitive(PrimitiveValue::Bool(true)))]),
        );

        let back_to_struct: TestStruct = json.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn json_test_option() {
        #[derive(JSONProperties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: Option<u32>,
        }

        let test_struct = TestStruct { a: Some(1) };

        let json: JSONProperties = test_struct.clone().into(); // Ensure this method exists
                                                               // println!("{:?}", json); // Debug output
        assert_eq!(
            json,
            JSONProperties::from([("a".into(), JSONValue::Primitive(PrimitiveValue::U64(1)))]),
        );

        let back_to_struct: TestStruct = json.into();
        assert_eq!(back_to_struct, test_struct);

        let test_struct = TestStruct { a: None };

        let json: JSONProperties = test_struct.clone().into(); // Ensure this method exists
                                                               // println!("{:?}", json); // Debug output
        assert_eq!(
            json,
            JSONProperties::from([("a".into(), JSONValue::Primitive(PrimitiveValue::Null))])
        );

        let back_to_struct: TestStruct = json.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn json_test_m_value_nested() {
        #[derive(JSONProperties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: JSONProperties,
        }

        let test_struct = TestStruct {
            a: JSONProperties::from([("b".into(), JSONValue::Primitive(PrimitiveValue::U64(1)))]),
        };

        let json: JSONProperties = test_struct.clone().into(); // Ensure this method exists
                                                               // println!("{:?}", json); // Debug output
        assert_eq!(
            json,
            JSONProperties::from([(
                "a".into(),
                JSONValue::Object(JSONProperties::from([(
                    "b".into(),
                    JSONValue::Primitive(PrimitiveValue::U64(1))
                )]))
            )]),
        );

        let back_to_struct: TestStruct = json.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn json_nested_object_optional_test() {
        #[derive(JSONProperties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct NestedStruct {
            a: String,
            b: u32,
        }
        #[derive(JSONProperties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: Option<NestedStruct>,
            pub b: u32,
        }

        let test_struct = TestStruct { a: Some(NestedStruct { a: "a".into(), b: 1 }), b: 2 };

        let json: JSONProperties = test_struct.clone().into(); // Ensure this method exists
                                                               // println!("{:?}", json); // Debug output
        assert_eq!(
            json,
            JSONProperties::from([
                (
                    "a".into(),
                    JSONValue::Object(JSONProperties::from([
                        ("a".into(), JSONValue::Primitive(PrimitiveValue::String("a".into()))),
                        ("b".into(), JSONValue::Primitive(PrimitiveValue::U64(1))),
                    ])),
                ),
                ("b".into(), JSONValue::Primitive(PrimitiveValue::U64(2))),
            ])
        );

        let back_to_struct: TestStruct = json.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn json_vec_test() {
        #[derive(JSONProperties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            pub a: Vec<String>,
            pub b: Vec<u32>,
            pub c: Vec<i32>,
            pub d: Option<Vec<f32>>,
        }
    }

    #[test]
    fn json_generics_test() {
        #[derive(JSONProperties, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct<T: JSONPropertiesCompatible = JSONProperties> {
            pub a: Option<T>,
        }

        let test_struct = TestStruct {
            a: Some(JSONProperties::from([(
                "b".into(),
                JSONValue::Primitive(PrimitiveValue::U64(1)),
            )])),
        };

        let json: JSONProperties = test_struct.clone().into(); // Ensure this method exists

        assert_eq!(
            json,
            JSONProperties::from([(
                "a".into(),
                JSONValue::Object(JSONProperties::from([(
                    "b".into(),
                    JSONValue::Primitive(PrimitiveValue::U64(1))
                )]))
            )]),
        );

        let back_to_struct: TestStruct = json.into();
        assert_eq!(back_to_struct, test_struct);
    }

    #[test]
    fn test_value_prim() {
        #[derive(ValuePrimitive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct NestedStruct {
            a: String,
            b: u32,
            c: Option<usize>,
            d: bool,
            e: (),
        }

        #[derive(MValueDerive, Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
        pub struct TestStruct {
            a: Vec<NestedStruct>,
        }

        let test_struct = TestStruct {
            a: vec![NestedStruct { a: "a".into(), b: 1, c: Some(2), d: true, e: () }],
        };

        let json: MValue = test_struct.clone().into(); // Ensure this method exists
                                                       // println!("{:?}", json); // Debug output
        assert_eq!(
            json,
            MValue::from([(
                "a".into(),
                ValueType::Array(vec![ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from(
                    [
                        ("a".into(), PrimitiveValue::String("a".into())),
                        ("b".into(), PrimitiveValue::U64(1)),
                        ("c".into(), PrimitiveValue::U64(2)),
                        ("d".into(), PrimitiveValue::Bool(true)),
                        ("e".into(), PrimitiveValue::Null),
                    ]
                ))]),
            )])
        );

        let back_to_struct: TestStruct = json.into();
        assert_eq!(back_to_struct, test_struct);
    }
}
