#[cfg(test)]
mod tests {
    use core::cmp::Ordering;
    use s2json_core::*;
    use serde_json::json;

    #[test]
    fn json_value() {
        let json_default = JSONValue::default();
        assert_eq!(json_default, JSONValue::Primitive(PrimitiveValue::Null));

        let json_default2: JSONValue = Default::default();
        assert_eq!(json_default2, json_default);
    }

    #[test]
    fn mvalue_from_ref() {
        let mvalue = MValue::from(&MValue::default());
        assert_eq!(mvalue, MValue::default());
    }

    #[test]
    fn primitive_value() {
        let prim_value = PrimitiveValue::String("test".into());
        assert!(!prim_value.is_number());
        assert_eq!(prim_value, PrimitiveValue::String("test".into()));
        let prim_value = PrimitiveValue::U64(1);
        assert!(prim_value.is_number());
        assert_eq!(prim_value, PrimitiveValue::U64(1));
        let prim_value = PrimitiveValue::I64(1);
        assert!(prim_value.is_number());
        assert_eq!(prim_value, PrimitiveValue::I64(1));
        let prim_value = PrimitiveValue::F32(1.0);
        assert!(prim_value.is_number());
        assert_eq!(prim_value, PrimitiveValue::F32(1.0));
        let prim_value = PrimitiveValue::F64(1.0);
        assert!(prim_value.is_number());
        assert_eq!(prim_value, PrimitiveValue::F64(1.0));
        let prim_value = PrimitiveValue::Bool(true);
        assert!(!prim_value.is_number());
        assert_eq!(prim_value, PrimitiveValue::Bool(true));
        let prim_value = PrimitiveValue::Null;
        assert!(!prim_value.is_number());
        assert_eq!(prim_value, PrimitiveValue::Null);
    }

    #[test]
    fn primitive_string_serialize() {
        let prim_value = PrimitiveValue::String("test".into());
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "\"test\"");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::String("test".into()));
    }

    #[test]
    fn primitive_u64_serialize() {
        let prim_value = PrimitiveValue::U64(1);
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "1");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::U64(1));
    }

    #[test]
    fn primitive_i64_serialize() {
        let prim_value = PrimitiveValue::I64(-1);
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "-1");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::I64(-1));
    }

    #[test]
    fn primitive_f32_serialize() {
        let prim_value = PrimitiveValue::F32(1.0);
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "1.0");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::F32(1.0));
    }

    #[test]
    fn primitive_f64_serialize() {
        let prim_value = PrimitiveValue::F64(-135435345435345345.0);
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "-1.3543534543534534e17");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::F32(-1.3543534e17));
    }

    #[test]
    fn primitive_bool_serialize() {
        let prim_value = PrimitiveValue::Bool(true);
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "true");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::Bool(true));
    }

    #[test]
    fn primitive_null_serialize() {
        let prim_value = PrimitiveValue::Null;
        let serialized = serde_json::to_string(&prim_value).unwrap();
        assert_eq!(serialized, "null");
        let deserialize = serde_json::from_str::<PrimitiveValue>(&serialized).unwrap();
        assert_eq!(deserialize, PrimitiveValue::Null);
    }

    #[test]
    fn value_default() {
        let default = ValueType::default();
        assert_eq!(default, ValueType::Primitive(PrimitiveValue::Null));

        let default_instance: ValueType = Default::default();
        assert_eq!(default, default_instance);
    }

    #[test]
    fn value_serialize() {
        let value = Value::from([
            ("type".into(), ValueType::Primitive(PrimitiveValue::String("Point".into()))),
            ("coordinates".into(), ValueType::Primitive(PrimitiveValue::F32(1.0))),
        ]);
        let serialized = serde_json::to_string(&value).unwrap();
        assert_eq!(serialized, "{\"coordinates\":1.0,\"type\":\"Point\"}");
        let deserialize = serde_json::from_str::<Value>(&serialized).unwrap();
        assert_eq!(deserialize, value);

        let value_str = r#"
        {
            "class": "ocean",
            "offset": 22,
            "info": {
                "name": "Pacific Ocean",
                "value": 22.2
            }
        }
        "#;

        let deserialize: MValue = serde_json::from_str::<Value>(value_str).unwrap();
        assert_eq!(
            deserialize,
            Value::from([
                ("class".into(), ValueType::Primitive(PrimitiveValue::String("ocean".into()))),
                ("offset".into(), ValueType::Primitive(PrimitiveValue::U64(22))),
                (
                    "info".into(),
                    ValueType::Nested(Value::from([
                        (
                            "name".into(),
                            ValueType::Primitive(PrimitiveValue::String("Pacific Ocean".into()))
                        ),
                        ("value".into(), ValueType::Primitive(PrimitiveValue::F32(22.2))),
                    ]))
                ),
            ])
        );
        let deserialize_to: MValue = deserialize.clone();
        assert_eq!(deserialize_to, deserialize);
        // from
        let desrialize_from: MValue = MValue::from(deserialize_to);
        assert_eq!(desrialize_from, deserialize);
    }

    #[test]
    fn from_json_properties_to_json_properties() {
        let json_properties = JSONProperties::from([
            ("type".into(), JSONValue::Primitive(PrimitiveValue::String("Point".into()))),
            ("coordinates".into(), JSONValue::Primitive(PrimitiveValue::F32(1.0))),
        ]);
        let json_properties_to: JSONProperties = (&json_properties).into();
        assert_eq!(json_properties_to, json_properties);
    }

    #[test]
    fn test_prim_value_cmp() {
        // null
        assert_eq!(PrimitiveValue::Null.partial_cmp(&PrimitiveValue::Null), Some(Ordering::Equal));
        // bool
        assert_eq!(
            PrimitiveValue::Bool(true).partial_cmp(&PrimitiveValue::Bool(true)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            PrimitiveValue::Bool(true).partial_cmp(&PrimitiveValue::Bool(false)),
            Some(Ordering::Greater)
        );
        // string
        assert_eq!(
            PrimitiveValue::String("a".into()).partial_cmp(&PrimitiveValue::String("a".into())),
            Some(Ordering::Equal)
        );
        assert_eq!(
            PrimitiveValue::String("a".into()).partial_cmp(&PrimitiveValue::String("c".into())),
            Some(Ordering::Less)
        );
        // f64
        assert_eq!(
            PrimitiveValue::F64(1.0).partial_cmp(&PrimitiveValue::F64(1.0)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            PrimitiveValue::F64(1.0).partial_cmp(&PrimitiveValue::F64(2.0)),
            Some(Ordering::Less)
        );
        // f32
        assert_eq!(
            PrimitiveValue::F32(1.0).partial_cmp(&PrimitiveValue::F32(1.0)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            PrimitiveValue::F32(1.0).partial_cmp(&PrimitiveValue::F32(2.0)),
            Some(Ordering::Less)
        );
        // u64
        assert_eq!(
            PrimitiveValue::U64(1).partial_cmp(&PrimitiveValue::U64(1)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            PrimitiveValue::U64(1).partial_cmp(&PrimitiveValue::U64(2)),
            Some(Ordering::Less)
        );
        // i64
        assert_eq!(
            PrimitiveValue::I64(1).partial_cmp(&PrimitiveValue::I64(1)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            PrimitiveValue::I64(1).partial_cmp(&PrimitiveValue::I64(2)),
            Some(Ordering::Less)
        );

        // different types
        assert_eq!(
            PrimitiveValue::String("a".into()).partial_cmp(&PrimitiveValue::U64(1)),
            Some(Ordering::Greater)
        );
        // different types 2
        assert_eq!(
            PrimitiveValue::Bool(false).partial_cmp(&PrimitiveValue::Null),
            Some(Ordering::Greater)
        );
        // different types f32 and f64
        assert_eq!(
            PrimitiveValue::F32(1.0).partial_cmp(&PrimitiveValue::F64(1.0)),
            Some(Ordering::Less)
        );
        // different types u64, i64
        assert_eq!(
            PrimitiveValue::U64(1).partial_cmp(&PrimitiveValue::I64(1)),
            Some(Ordering::Greater)
        );
    }

    #[test]
    fn prim_value_numbers_into() {
        let prim_value = PrimitiveValue::F64(1.1);
        let float: f32 = (&prim_value).into();
        assert_eq!(float, 1.1);
        let prim_value = PrimitiveValue::F64(1.1);
        let double: f64 = (&prim_value).into();
        assert_eq!(double, 1.1);

        // i32
        let prim_value = PrimitiveValue::I64(1);
        let i32: i32 = (&prim_value).into();
        assert_eq!(i32, 1);
        // i64
        let prim_value = PrimitiveValue::I64(1);
        let i64: i64 = (&prim_value).into();
        assert_eq!(i64, 1);
        // isize
        let prim_value = PrimitiveValue::I64(1);
        let isize: isize = (&prim_value).into();
        assert_eq!(isize, 1);

        // u32
        let prim_value = PrimitiveValue::U64(1);
        let u32: u32 = (&prim_value).into();
        assert_eq!(u32, 1);
        // u64
        let prim_value = PrimitiveValue::U64(1);
        let u64: u64 = (&prim_value).into();
        assert_eq!(u64, 1);
        // usize
        let prim_value = PrimitiveValue::U64(1);
        let usize: usize = (&prim_value).into();
        assert_eq!(usize, 1);
    }

    #[test]
    fn value_from_object() {
        let j = json!({
            "a": 1,
            "b": true,
            "c": "str",
            "d": null,
        });

        let v = Value::from(&j);

        assert!(matches!(
            v.get("a").unwrap(),
            ValueType::Primitive(PrimitiveValue::F64(x)) if *x == 1.0
        ));
        assert!(matches!(v.get("b").unwrap(), ValueType::Primitive(PrimitiveValue::Bool(true))));
        assert!(matches!(
            v.get("c").unwrap(),
            ValueType::Primitive(PrimitiveValue::String(s)) if s == "str"
        ));
        assert!(matches!(v.get("d").unwrap(), ValueType::Primitive(PrimitiveValue::Null)));
    }

    #[test]
    fn value_from_non_object_is_empty() {
        let j = json!(123);
        let v = Value::from(&j);
        assert!(v.is_empty());
    }

    #[test]
    fn valuetype_primitives() {
        assert!(matches!(
            ValueType::from(&json!(null)),
            ValueType::Primitive(PrimitiveValue::Null)
        ));

        assert!(matches!(
            ValueType::from(&json!(false)),
            ValueType::Primitive(PrimitiveValue::Bool(false))
        ));

        assert!(matches!(
            ValueType::from(&json!(2.5)),
            ValueType::Primitive(PrimitiveValue::F64(x)) if x == 2.5
        ));

        assert!(matches!(
            ValueType::from(&json!("abc")),
            ValueType::Primitive(PrimitiveValue::String(s)) if s == "abc"
        ));
    }

    #[test]
    fn valuetype_array() {
        let j = json!([1, 2, 3]);
        let v = ValueType::from(&j);

        match v {
            ValueType::Array(arr) => {
                assert_eq!(arr.len(), 3);
                assert!(matches!(
                    arr[0],
                    ValuePrimitiveType::Primitive(PrimitiveValue::F64(x)) if x == 1.0
                ));
            }
            _ => panic!("expected array"),
        }
    }

    #[test]
    fn valuetype_nested_object() {
        let j = json!({ "x": 1 });
        let v = ValueType::from(&j);

        match v {
            ValueType::Nested(map) => {
                assert!(matches!(
                    map.get("x").unwrap(),
                    ValueType::Primitive(PrimitiveValue::F64(x)) if *x == 1.0
                ));
            }
            _ => panic!("expected nested"),
        }
    }

    #[test]
    fn valueprimitive_only_accepts_primitives() {
        assert!(matches!(
            ValuePrimitiveType::from(&json!(true)),
            ValuePrimitiveType::Primitive(PrimitiveValue::Bool(true))
        ));

        assert!(matches!(
            ValuePrimitiveType::from(&json!("s")),
            ValuePrimitiveType::Primitive(PrimitiveValue::String(s)) if s == "s"
        ));

        // arrays and objects collapse to Null
        assert!(matches!(
            ValuePrimitiveType::from(&json!([1, 2])),
            ValuePrimitiveType::Primitive(PrimitiveValue::Null)
        ));
        assert!(matches!(
            ValuePrimitiveType::from(&json!({ "a": 1 })),
            ValuePrimitiveType::Primitive(PrimitiveValue::Null)
        ));
    }

    #[test]
    fn value_from_map() {
        let j = json!({ "k": 42 });
        let map = j.as_object().unwrap();
        let v = Value::from(map);

        assert!(matches!(
            v.get("k").unwrap(),
            ValueType::Primitive(PrimitiveValue::F64(x)) if *x == 42.0
        ));
    }
}
