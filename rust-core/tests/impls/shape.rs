use impls::shape::validate_types;
use s2json_core::*;
use std::panic::{self, AssertUnwindSafe};

fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

#[test]
fn encode_decode_shape() {
    let json_shape = r#"{
            "a": "i64",
            "b": ["string"],
            "c": {
                "d": "f64",
                "e": "bool",
                "f": "null",
                "g": {
                    "h": "i64",
                    "i": "string"
                }
            },
            "d": [{
                "j": "f64",
                "k": "bool"
            }]
        }"#;

    let shape = serde_json::from_str::<Shape>(json_shape).unwrap();
    assert_eq!(
        shape,
        Map::from([
            ("a".into(), ShapeType::Primitive(PrimitiveShape::I64)),
            (
                "b".into(),
                ShapeType::Array(vec![PrimitiveShapeType::Primitive(PrimitiveShape::String)])
            ),
            (
                "c".into(),
                ShapeType::Nested(Shape::from([
                    ("d".into(), ShapeType::Primitive(PrimitiveShape::F64)),
                    ("e".into(), ShapeType::Primitive(PrimitiveShape::Bool)),
                    ("f".into(), ShapeType::Primitive(PrimitiveShape::Null)),
                    (
                        "g".into(),
                        ShapeType::Nested(Shape::from([
                            ("h".into(), ShapeType::Primitive(PrimitiveShape::I64)),
                            ("i".into(), ShapeType::Primitive(PrimitiveShape::String)),
                        ]))
                    )
                ]))
            ),
            (
                "d".into(),
                ShapeType::Array(vec![PrimitiveShapeType::NestedPrimitive(ShapePrimitive::from(
                    [("j".into(), PrimitiveShape::F64), ("k".into(), PrimitiveShape::Bool),]
                ))])
            )
        ])
    );
}

#[test]
fn primitive_shape() {
    let int64 = PrimitiveShape::I64;
    let is_number = int64.is_number();
    assert!(is_number);
    assert!(int64.matching_shape(&PrimitiveShape::F32));
    assert!(!int64.matching_shape(&PrimitiveShape::String));
    // get_highest_order_number
    assert_eq!(
        PrimitiveShape::get_highest_order_number(&PrimitiveShape::F32, &PrimitiveShape::F64),
        PrimitiveShape::F64
    );
    assert_eq!(
        PrimitiveShape::get_highest_order_number(&PrimitiveShape::F64, &PrimitiveShape::F32),
        PrimitiveShape::F64
    );
    assert_eq!(
        PrimitiveShape::get_highest_order_number(&PrimitiveShape::F32, &PrimitiveShape::F32),
        PrimitiveShape::F32
    );
    assert_eq!(
        PrimitiveShape::get_highest_order_number(&PrimitiveShape::I64, &PrimitiveShape::F32),
        PrimitiveShape::F32
    );
    assert_eq!(
        PrimitiveShape::get_highest_order_number(&PrimitiveShape::U64, &PrimitiveShape::Null),
        PrimitiveShape::U64
    );
    assert_eq!(
        PrimitiveShape::get_highest_order_number(&PrimitiveShape::I64, &PrimitiveShape::I64),
        PrimitiveShape::I64
    );
    assert_eq!(
        PrimitiveShape::get_highest_order_number(&PrimitiveShape::F64, &PrimitiveShape::U64),
        PrimitiveShape::F64
    );

    // error if number doesn't exist
    let result = panic::catch_unwind(AssertUnwindSafe(|| PrimitiveShape::from(100)));
    assert!(result.is_err());
}

#[test]
fn encode_decode_value() {
    let example_shape_str = r#"{
            "a": "i64",
            "b": "u64",
            "c": "f64"
        }"#;
    let example_shape = serde_json::from_str::<Shape>(example_shape_str).unwrap();
    // back to shape
    let back_to_shape = serde_json::to_string(&example_shape).unwrap();
    assert_eq!(back_to_shape, remove_whitespace(example_shape_str));

    let example_value_str = r#"{
            "a": 3,
            "b": 1,
            "c": 2.2
        }"#;
    let example_value = serde_json::from_str::<Value>(example_value_str).unwrap();
    // back to value
    let back_to_value = serde_json::to_string(&example_value).unwrap();
    assert_eq!(back_to_value.trim(), remove_whitespace(example_value_str));
}

#[test]
fn validate_types_none() {
    assert_eq!(validate_types(&[]), PrimitiveShapeType::Primitive(PrimitiveShape::Null));

    assert_eq!(
        validate_types(&[
            ValuePrimitiveType::Primitive(PrimitiveValue::I64(3)),
            ValuePrimitiveType::Primitive(PrimitiveValue::I64(22)),
        ]),
        PrimitiveShapeType::Primitive(PrimitiveShape::I64)
    );

    assert_eq!(
        validate_types(&[
            ValuePrimitiveType::Primitive(PrimitiveValue::I64(3)),
            ValuePrimitiveType::Primitive(PrimitiveValue::U64(22)),
        ]),
        PrimitiveShapeType::Primitive(PrimitiveShape::I64)
    );

    assert_eq!(
        validate_types(&[
            ValuePrimitiveType::Primitive(PrimitiveValue::I64(3)),
            ValuePrimitiveType::Primitive(PrimitiveValue::F64(-22.2)),
        ]),
        PrimitiveShapeType::Primitive(PrimitiveShape::F64)
    );

    assert_eq!(
        validate_types(&[
            ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([
                ("a".into(), PrimitiveValue::I64(3)),
                ("b".into(), PrimitiveValue::String("hello".into())),
            ])),
            ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([
                ("a".into(), PrimitiveValue::I64(22)),
                ("b".into(), PrimitiveValue::String("world".into())),
            ])),
        ]),
        PrimitiveShapeType::NestedPrimitive(ShapePrimitive::from([
            ("a".into(), PrimitiveShape::I64),
            ("b".into(), PrimitiveShape::String),
        ]))
    );

    let error_case_one = panic::catch_unwind(AssertUnwindSafe(|| {
        validate_types(&[
            ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([
                ("a".into(), PrimitiveValue::I64(3)),
                ("b".into(), PrimitiveValue::String("hello".into())),
            ])),
            ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([
                ("a".into(), PrimitiveValue::U64(5)),
                ("b".into(), PrimitiveValue::F32(2.2)),
            ])),
        ])
    }));
    assert!(error_case_one.is_err());

    let error_case_two = panic::catch_unwind(AssertUnwindSafe(|| {
        validate_types(&[
            ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([
                ("a".into(), PrimitiveValue::I64(3)),
                ("b".into(), PrimitiveValue::String("hello".into())),
            ])),
            ValuePrimitiveType::Primitive(PrimitiveValue::I64(3)),
        ])
    }));
    assert!(error_case_two.is_err());

    let error_case_three = panic::catch_unwind(AssertUnwindSafe(|| {
        validate_types(&[
            ValuePrimitiveType::Primitive(PrimitiveValue::I64(3)),
            ValuePrimitiveType::Primitive(PrimitiveValue::String("test".into())),
        ])
    }));
    assert!(error_case_three.is_err());
}

// ValuePrimitiveType -> PrimitiveShapeType
#[test]
fn test_value_primitive_type_to_shape_primitive_type() {
    let vpt: ValuePrimitiveType = ValuePrimitiveType::Primitive(PrimitiveValue::I64(1));
    let spt = PrimitiveShapeType::Primitive(PrimitiveShape::I64);
    let res = PrimitiveShapeType::from(&vpt);
    assert_eq!(res, spt);

    // NestedPrimitive
    let vpt: ValuePrimitiveType = ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([
        ("a".into(), PrimitiveValue::I64(1)),
        ("b".into(), PrimitiveValue::String("hello".into())),
    ]));
    let spt = PrimitiveShapeType::NestedPrimitive(ShapePrimitive::from([
        ("a".into(), PrimitiveShape::I64),
        ("b".into(), PrimitiveShape::String),
    ]));
    let res = PrimitiveShapeType::from(&vpt);
    assert_eq!(res, spt);
}

#[test]
fn test_default_from_shape() {
    let val = Value::from([
        ("a".into(), (&PrimitiveValue::I64(1)).into()),
        ("b".into(), (&PrimitiveValue::String("hello".into())).into()),
        ("c".into(), (&PrimitiveValue::F64(3.0)).into()),
        ("d".into(), (&PrimitiveValue::Null).into()),
        ("e".into(), (&PrimitiveValue::Bool(true)).into()),
        ("f".into(), (&PrimitiveValue::F32(-2.)).into()),
        ("g".into(), (&PrimitiveValue::U64(2)).into()),
        ("h".into(), ValueType::Array(vec![])),
        ("i".into(), ValueType::Nested(Value::default())),
    ]);
    let shape = Shape::from(&val);
    let default_val = Value::default_from_shape(&shape);
    assert_eq!(
        default_val,
        Value::from([
            ("a".into(), (&PrimitiveValue::I64(0)).into()),
            ("b".into(), (&PrimitiveValue::String("".into())).into()),
            ("c".into(), (&PrimitiveValue::F64(0.)).into()),
            ("d".into(), (&PrimitiveValue::Null).into()),
            ("e".into(), (&PrimitiveValue::Bool(false)).into()),
            ("f".into(), (&PrimitiveValue::F32(0.)).into()),
            ("g".into(), (&PrimitiveValue::U64(0)).into()),
            ("h".into(), ValueType::Array(vec![])),
            ("i".into(), ValueType::Nested(Value::default())),
        ])
    );
}

#[test]
fn test_prim_shape_to_usize() {
    let shape = PrimitiveShape::String;
    let sizing: usize = (&shape).into();
    assert_eq!(sizing, 0);
    let back_to_prim: PrimitiveShape = sizing.into();
    assert_eq!(back_to_prim, shape);

    let shape = PrimitiveShape::U64;
    let sizing: usize = (&shape).into();
    assert_eq!(sizing, 1);
    let back_to_prim: PrimitiveShape = sizing.into();
    assert_eq!(back_to_prim, shape);

    let shape = PrimitiveShape::I64;
    let sizing: usize = (&shape).into();
    assert_eq!(sizing, 2);
    let back_to_prim: PrimitiveShape = sizing.into();
    assert_eq!(back_to_prim, shape);

    let shape = PrimitiveShape::F32;
    let sizing: usize = (&shape).into();
    assert_eq!(sizing, 3);
    let back_to_prim: PrimitiveShape = sizing.into();
    assert_eq!(back_to_prim, shape);

    let shape = PrimitiveShape::F64;
    let sizing: usize = (&shape).into();
    assert_eq!(sizing, 4);
    let back_to_prim: PrimitiveShape = sizing.into();
    assert_eq!(back_to_prim, shape);

    let shape = PrimitiveShape::Bool;
    let sizing: usize = (&shape).into();
    assert_eq!(sizing, 5);
    let back_to_prim: PrimitiveShape = sizing.into();
    assert_eq!(back_to_prim, shape);

    let shape = PrimitiveShape::Null;
    let sizing: usize = (&shape).into();
    assert_eq!(sizing, 6);
    let back_to_prim: PrimitiveShape = sizing.into();
    assert_eq!(back_to_prim, shape);
}

#[test]
fn test_merge_values_to_shape() {
    let val = Value::from([
        ("a".into(), (&PrimitiveValue::I64(1)).into()),
        ("b".into(), (&PrimitiveValue::String("hello".into())).into()),
        ("c".into(), (&PrimitiveValue::F64(3.0)).into()),
        ("d".into(), (&PrimitiveValue::Null).into()),
        ("e".into(), (&PrimitiveValue::Bool(true)).into()),
        ("f".into(), (&PrimitiveValue::F32(-2.)).into()),
        ("g".into(), (&PrimitiveValue::U64(2)).into()),
        (
            "h".into(),
            ValueType::Array(vec![
                (PrimitiveValue::U64(2)).into(),
                (PrimitiveValue::U64(3)).into(),
            ]),
        ),
        (
            "i".into(),
            ValueType::Nested(Value::from([
                ("j".into(), (&PrimitiveValue::I64(-5)).into()),
                ("k".into(), (&PrimitiveValue::I64(-7)).into()),
            ])),
        ),
        (
            "l".into(),
            ValueType::Array(vec![ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([
                ("a".into(), PrimitiveValue::Bool(true)),
                ("b".into(), PrimitiveValue::Bool(false)),
            ]))]),
        ),
    ]);
    let val2 = Value::from([
        ("a".into(), (&PrimitiveValue::I64(5)).into()),
        ("b".into(), (&PrimitiveValue::String("world".into())).into()),
        ("c".into(), (&PrimitiveValue::F64(-3.0)).into()),
        ("d".into(), (&PrimitiveValue::Null).into()),
        ("e".into(), (&PrimitiveValue::Bool(false)).into()),
        ("f".into(), (&PrimitiveValue::F32(2.)).into()),
        ("g".into(), (&PrimitiveValue::U64(4)).into()),
        (
            "h".into(),
            ValueType::Array(vec![
                (PrimitiveValue::U64(22)).into(),
                (PrimitiveValue::U64(55)).into(),
            ]),
        ),
        (
            "i".into(),
            ValueType::Nested(Value::from([
                ("j".into(), (&PrimitiveValue::I64(5)).into()),
                ("k".into(), (&PrimitiveValue::I64(7)).into()),
            ])),
        ),
        (
            "l".into(),
            ValueType::Array(vec![ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([
                ("a".into(), PrimitiveValue::Bool(false)),
                ("b".into(), PrimitiveValue::Null),
                ("c".into(), PrimitiveValue::U64(2)),
            ]))]),
        ),
    ]);

    let values: Vec<Value> = vec![val, val2];
    let merge_shape = Shape::from(values.as_slice());

    assert_eq!(
        merge_shape,
        Shape::from([
            ("a".into(), ShapeType::Primitive(PrimitiveShape::I64)),
            ("b".into(), ShapeType::Primitive(PrimitiveShape::String)),
            ("c".into(), ShapeType::Primitive(PrimitiveShape::F64)),
            ("d".into(), ShapeType::Primitive(PrimitiveShape::Null)),
            ("e".into(), ShapeType::Primitive(PrimitiveShape::Bool)),
            ("f".into(), ShapeType::Primitive(PrimitiveShape::F32)),
            ("g".into(), ShapeType::Primitive(PrimitiveShape::U64)),
            (
                "h".into(),
                ShapeType::Array(vec![PrimitiveShapeType::Primitive(PrimitiveShape::U64)])
            ),
            (
                "i".into(),
                ShapeType::Nested(Shape::from([
                    ("j".into(), ShapeType::Primitive(PrimitiveShape::I64)),
                    ("k".into(), ShapeType::Primitive(PrimitiveShape::I64))
                ]))
            ),
            (
                "l".into(),
                ShapeType::Array(vec![PrimitiveShapeType::NestedPrimitive(ShapePrimitive::from(
                    [
                        ("a".into(), PrimitiveShape::Bool),
                        ("b".into(), PrimitiveShape::Bool),
                        ("c".into(), PrimitiveShape::U64)
                    ]
                ))])
            )
        ])
    )
}

#[test]
fn test_shape_type_default() {
    let default = ShapeType::default();
    assert_eq!(default, ShapeType::Primitive(PrimitiveShape::Null));
}

#[test]
fn test_value_type_to_prim_value() {
    let val_type = ValueType::Primitive(PrimitiveValue::Bool(true));
    let prim_value = PrimitiveValue::from(&val_type);

    assert_eq!(prim_value, PrimitiveValue::Bool(true));

    let val_type = ValueType::Nested(Map::default());
    let prim_value = PrimitiveValue::from(&val_type);

    assert_eq!(prim_value, PrimitiveValue::Null);
}
