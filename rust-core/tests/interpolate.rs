#[cfg(test)]
mod tests {
    use s2json_core::{Interpolate, *};

    #[test]
    fn interpolate_numbers() {
        // ()
        let a = ();
        let b = ();
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, ());
        // f64
        let a: f64 = 1.;
        let b: f64 = 2.;
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, 1.5);
        // f32
        let a: f32 = 1.;
        let b: f32 = 2.;
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, 1.5);
        // isize
        let a: isize = 1;
        let b: isize = 2;
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, 1);
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, 2);
        // i8
        let a: i8 = 1;
        let b: i8 = 2;
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, 1);
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, 2);
        // i16
        let a: i16 = 1;
        let b: i16 = 2;
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, 1);
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, 2);
        // i32
        let a: i32 = 1;
        let b: i32 = 2;
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, 1);
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, 2);
        // i64
        let a: i64 = 1;
        let b: i64 = 2;
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, 1);
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, 2);
        // usize
        let a: usize = 1;
        let b: usize = 2;
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, 1);
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, 2);
        // u8
        let a: u8 = 1;
        let b: u8 = 2;
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, 1);
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, 2);
        // u16
        let a: u16 = 1;
        let b: u16 = 2;
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, 1);
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, 2);
        // u32
        let a: u32 = 1;
        let b: u32 = 2;
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, 1);
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, 2);
        // u64
        let a: u64 = 1;
        let b: u64 = 2;
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, 1);
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, 2);
    }

    #[test]
    fn interpolate_option() {
        let a: Option<f64> = Some(1.);
        let b: Option<f64> = Some(2.);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, Some(1.5));

        let a: Option<f64> = Some(1.);
        let b: Option<f64> = None;
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, None);
    }

    #[test]
    fn interpolate_point() {
        let a = Point(1., 2.);
        let b = Point(3., 4.);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, Point(2., 3.));
    }

    #[test]
    fn interpolate_point3d() {
        let a = Point3D(1., 2., 3.);
        let b = Point3D(4., 5., 6.);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, Point3D(2.5, 3.5, 4.5));
    }

    #[test]
    fn interpolate_point_or_point3d() {
        let a = PointOrPoint3D(1., 2., Some(3.));
        let b = PointOrPoint3D(4., 5., Some(6.));
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, PointOrPoint3D(2.5, 3.5, Some(4.5)));

        let a = PointOrPoint3D(1., 2., Some(3.));
        let b = PointOrPoint3D(4., 5., None);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, PointOrPoint3D(2.5, 3.5, None));
    }

    #[test]
    fn interpolate_stpoint() {
        let a = STPoint { face: 1.into(), s: 1., t: 2., z: Some(3.), m: Some(4.) };
        let b = STPoint { face: 2.into(), s: 4., t: 5., z: Some(6.), m: Some(7.) };
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, STPoint { face: 1.into(), s: 2.5, t: 3.5, z: Some(4.5), m: Some(5.5) });
    }

    #[test]
    fn interpolate_vector_point() {
        let a = VectorPoint { x: 1., y: 2., z: Some(3.), m: Some(4.), t: Some(5.) };
        let b = VectorPoint { x: 4., y: 5., z: Some(6.), m: Some(7.), t: Some(8.) };
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, VectorPoint { x: 2.5, y: 3.5, z: Some(4.5), m: Some(5.5), t: Some(6.5) });
    }

    #[test]
    fn interpolate_primitive_value() {
        // u64
        let a = PrimitiveValue::U64(1);
        let b = PrimitiveValue::U64(2);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, PrimitiveValue::U64(2));
        // i64
        let a = PrimitiveValue::I64(1);
        let b = PrimitiveValue::I64(2);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, PrimitiveValue::I64(2));
        // f64
        let a = PrimitiveValue::F64(1.);
        let b = PrimitiveValue::F64(2.);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, PrimitiveValue::F64(1.5));
        // f32
        let a = PrimitiveValue::F32(1.);
        let b = PrimitiveValue::F32(2.);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, PrimitiveValue::F32(1.5));
        // string
        let a = PrimitiveValue::String("hello".into());
        let b = PrimitiveValue::String("world".into());
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, PrimitiveValue::String("hello".into()));
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, PrimitiveValue::String("world".into()));
        // bool
        let a = PrimitiveValue::Bool(true);
        let b = PrimitiveValue::Bool(false);
        let c = a.interpolate(&b, 0.4);
        assert_eq!(c, PrimitiveValue::Bool(true));
        let c = a.interpolate(&b, 0.6);
        assert_eq!(c, PrimitiveValue::Bool(false));
        // two types not the same
        let a = PrimitiveValue::String("hello".into());
        let b = PrimitiveValue::F64(2.);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, a);
    }

    #[test]
    fn test_value() {
        let a = Value::from([("a".into(), (1.0_f64).into()), ("b".into(), (2.0_f64).into())]);
        let b = Value::from([("a".into(), (3.0_f64).into()), ("b".into(), (4.0_f64).into())]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            Value::from([("a".into(), (2.0_f64).into()), ("b".into(), (3.0_f64).into())])
        );

        let a = Value::from([("a".into(), (1.0_f64).into()), ("b".into(), (2.0_f64).into())]);
        let b = Value::from([("a".into(), (3.0_f64).into())]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            Value::from([("a".into(), (2.0_f64).into()), ("b".into(), (2.0_f64).into())])
        );

        let a = Value::from([("a".into(), (1.0_f64).into())]);
        let b = Value::from([("a".into(), (3.0_f64).into()), ("b".into(), (4.0_f64).into())]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            Value::from([("a".into(), (2.0_f64).into()), ("b".into(), (4.0_f64).into())])
        );
    }

    #[test]
    fn test_json_props() {
        let a =
            JSONProperties::from([("a".into(), (1.0_f64).into()), ("b".into(), (2.0_f64).into())]);
        let b =
            JSONProperties::from([("a".into(), (3.0_f64).into()), ("b".into(), (4.0_f64).into())]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            JSONProperties::from([("a".into(), (2.0_f64).into()), ("b".into(), (3.0_f64).into())])
        );

        let a =
            JSONProperties::from([("a".into(), (1.0_f64).into()), ("b".into(), (2.0_f64).into())]);
        let b = JSONProperties::from([("a".into(), (3.0_f64).into())]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            JSONProperties::from([("a".into(), (2.0_f64).into()), ("b".into(), (2.0_f64).into())])
        );

        let a = JSONProperties::from([("a".into(), (1.0_f64).into())]);
        let b =
            JSONProperties::from([("a".into(), (3.0_f64).into()), ("b".into(), (4.0_f64).into())]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            JSONProperties::from([("a".into(), (2.0_f64).into()), ("b".into(), (4.0_f64).into())])
        );
    }

    #[test]
    fn test_value_type() {
        let a = Value::from([(
            "a".into(),
            ValueType::Array(vec![(1.0_f64).into(), (2.0_f64).into(), (3.0_f64).into()]),
        )]);
        let b = Value::from([(
            "a".into(),
            ValueType::Array(vec![(4.0_f64).into(), (5.0_f64).into(), (6.0_f64).into()]),
        )]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            Value::from([(
                "a".into(),
                ValueType::Array(vec![(2.5_f64).into(), (3.5_f64).into(), (4.5_f64).into()])
            )])
        );

        let a = Value::from([(
            "a".into(),
            ValueType::Nested(Value::from([("a".into(), (1.0_f64).into())])),
        )]);
        let b = Value::from([(
            "a".into(),
            ValueType::Nested(Value::from([("a".into(), (4.0_f64).into())])),
        )]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            Value::from([(
                "a".into(),
                ValueType::Nested(Value::from([("a".into(), (2.5_f64).into())]))
            )])
        );

        let a = Value::from([("a".into(), ValueType::Primitive((1.0_f64).into()))]);
        let b = Value::from([(
            "a".into(),
            ValueType::Array(vec![(4.0_f64).into(), (5.0_f64).into(), (6.0_f64).into()]),
        )]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, Value::from([("a".into(), ValueType::Primitive(PrimitiveValue::Null))]));
    }

    #[test]
    fn test_value_primitive() {
        let a =
            ValuePrimitive::from([("a".into(), (1.0_f64).into()), ("b".into(), (2.0_f64).into())]);
        let b =
            ValuePrimitive::from([("a".into(), (3.0_f64).into()), ("b".into(), (4.0_f64).into())]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            ValuePrimitive::from([("a".into(), (2.0_f64).into()), ("b".into(), (3.0_f64).into())])
        );

        let a =
            ValuePrimitive::from([("a".into(), (1.0_f64).into()), ("b".into(), (2.0_f64).into())]);
        let b = ValuePrimitive::from([("a".into(), (3.0_f64).into())]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            ValuePrimitive::from([("a".into(), (2.0_f64).into()), ("b".into(), (2.0_f64).into())])
        );

        let a = ValuePrimitive::from([("a".into(), (1.0_f64).into())]);
        let b =
            ValuePrimitive::from([("a".into(), (3.0_f64).into()), ("b".into(), (4.0_f64).into())]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            ValuePrimitive::from([("a".into(), (2.0_f64).into()), ("b".into(), (4.0_f64).into())])
        );
    }

    #[test]
    fn test_value_primitive_type() {
        // prim
        let a = ValuePrimitiveType::Primitive((1.0_f64).into());
        let b = ValuePrimitiveType::Primitive((2.0_f64).into());
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, ValuePrimitiveType::Primitive((1.5_f64).into()));
        // nested_prim
        let a = ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([(
            "a".into(),
            (1.0_f64).into(),
        )]));
        let b = ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([(
            "a".into(),
            (2.0_f64).into(),
        )]));
        let c = a.interpolate(&b, 0.5);
        assert_eq!(
            c,
            ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([(
                "a".into(),
                (1.5_f64).into(),
            )]))
        );
        // null case
        let a = ValuePrimitiveType::Primitive(PrimitiveValue::Null);
        let b = ValuePrimitiveType::NestedPrimitive(ValuePrimitive::from([(
            "a".into(),
            (2.0_f64).into(),
        )]));
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, ValuePrimitiveType::Primitive(PrimitiveValue::Null));
    }

    #[test]
    fn test_json_value() {
        // prim
        let a = JSONValue::Primitive((1.0_f64).into());
        let b = JSONValue::Primitive((2.0_f64).into());
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, JSONValue::Primitive((1.5_f64).into()));
        // nested_prim
        let a = JSONValue::Object(JSONProperties::from([("a".into(), (1.0_f64).into())]));
        let b = JSONValue::Object(JSONProperties::from([("a".into(), (2.0_f64).into())]));
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, JSONValue::Object(JSONProperties::from([("a".into(), (1.5_f64).into(),)])));
        // null case
        let a = JSONValue::Primitive(PrimitiveValue::Null);
        let b = JSONValue::Object(JSONProperties::from([("a".into(), (2.0_f64).into())]));
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, JSONValue::Primitive(PrimitiveValue::Null));

        // array
        let a = JSONValue::Array(vec![(1.0_f64).into(), (2.0_f64).into(), (3.0_f64).into()]);
        let b = JSONValue::Array(vec![(4.0_f64).into(), (5.0_f64).into(), (6.0_f64).into()]);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, JSONValue::Array(vec![(2.5_f64).into(), (3.5_f64).into(), (4.5_f64).into()]));
    }

    #[test]
    fn test_bbox() {
        let a = BBox::new(1., 2., 3., 4.);
        let b = BBox::new(5., 6., 7., 8.);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, BBox::new(3., 4., 5., 6.));

        let a: BBOX = BBox::new(1., 2., 3., 4.).into();
        let b: BBOX = BBox::new(5., 6., 7., 8.).into();
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, BBox::new(3., 4., 5., 6.).into());

        let a = BBox3D::new(1., 2., 3., 4., 5., 6.);
        let b = BBox3D::new(7., 8., 9., 10., 11., 12.);
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, BBox3D::new(4., 5., 6., 7., 8., 9.));

        let a: BBOX = BBox3D::new(1., 2., 3., 4., 5., 6.).into();
        let b: BBOX = BBox3D::new(7., 8., 9., 10., 11., 12.).into();
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, BBox3D::new(4., 5., 6., 7., 8., 9.).into());

        let a: BBOX = BBox::new(1., 2., 3., 4.).into();
        let b: BBOX = BBox3D::new(7., 8., 9., 10., 11., 12.).into();
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, a);

        let a: BBOX = BBox3D::new(1., 2., 3., 4., 5., 6.).into();
        let b: BBOX = BBox::new(7., 8., 9., 10.).into();
        let c = a.interpolate(&b, 0.5);
        assert_eq!(c, a);
    }
}
