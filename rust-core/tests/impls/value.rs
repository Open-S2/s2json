extern crate alloc;

#[cfg(test)]
mod tests {
    use alloc::vec;
    use pbf::Protobuf;
    use s2json_core::*;

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
        // string to number
        let prim_value: PrimitiveValue = "1".into();
        assert_eq!(PrimitiveValue::String("1".into()), prim_value);
        assert_eq!(prim_value.to_u64(), Some(1));
        assert_eq!(prim_value.to_i64(), Some(1));
        assert_eq!(prim_value.to_f32(), Some(1.0));
        assert_eq!(prim_value.to_f64(), Some(1.0));
        // string to bool
        let prim_value: PrimitiveValue = "true".into();
        assert_eq!(PrimitiveValue::String("true".into()), prim_value);
        assert_eq!(prim_value.to_bool(), Some(true));
        let prim_value: PrimitiveValue = "false".into();
        assert_eq!(PrimitiveValue::String("false".into()), prim_value);
        assert_eq!(prim_value.to_bool(), Some(false));
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
        assert_eq!(prim_value.to_u64(), None);
        assert_eq!(prim_value.to_i64(), None);
        assert_eq!(prim_value.to_f32(), None);
        assert_eq!(prim_value.to_f64(), None);
        assert_eq!(prim_value.to_bool(), None);
        // Option
        let prim_value: PrimitiveValue = Some(true).into();
        assert_eq!(PrimitiveValue::Bool(true), prim_value);
        assert_eq!(prim_value.to_bool(), Some(true));
        let prim_value: PrimitiveValue = None::<bool>.into();
        assert_eq!(PrimitiveValue::Null, prim_value);
        assert!(prim_value.is_null());

        assert!(PrimitiveValue::Null != PrimitiveValue::I64(2));
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
    fn value_prim_type_numbers() {
        let prim_value: ValuePrimitiveType = "1".into();
        let unsigned: u32 = (&prim_value).into();
        assert_eq!(unsigned, 1);
        let prim_value: ValuePrimitiveType = "-1".into();
        let signed: i32 = (&prim_value).into();
        assert_eq!(signed, -1);
        let prim_value: ValuePrimitiveType = "1".into();
        let unsigned: u32 = (prim_value).into();
        assert_eq!(unsigned, 1);
        let prim_value: ValuePrimitiveType = "-1".into();
        let signed: i32 = (prim_value).into();
        assert_eq!(signed, -1);
        let prim_value: ValuePrimitiveType = "1.0".into();
        let float: f32 = (&prim_value).into();
        assert_eq!(float, 1.0);
        let prim_value: ValuePrimitiveType = "-1.0".into();
        let double: f64 = (&prim_value).into();
        assert_eq!(double, -1.0);
        let prim_value: ValuePrimitiveType = "1.0".into();
        let float: f32 = (prim_value).into();
        assert_eq!(float, 1.0);
        let prim_value: ValuePrimitiveType = "-1.0".into();
        let double: f64 = (prim_value).into();
        assert_eq!(double, -1.0);

        let prim_value: ValuePrimitiveType = (1_f32).into();
        let float: f32 = (&prim_value).into();
        assert_eq!(float, 1.0);
        let prim_value: ValuePrimitiveType = (-1_f32).into();
        let double: f32 = (&prim_value).into();
        assert_eq!(double, -1.0);
        let prim_value: ValuePrimitiveType = (1_f32).into();
        let float: f32 = (prim_value).into();
        assert_eq!(float, 1.0);
        let prim_value: ValuePrimitiveType = (-1_f32).into();
        let double: f32 = (prim_value).into();
        assert_eq!(double, -1.0);

        let prim_value: ValuePrimitiveType = (1_f64).into();
        let float: f64 = (&prim_value).into();
        assert_eq!(float, 1.0);
        let prim_value: ValuePrimitiveType = (-1_f64).into();
        let double: f64 = (&prim_value).into();
        assert_eq!(double, -1.0);
        let prim_value: ValuePrimitiveType = (1_f64).into();
        let float: f64 = (prim_value).into();
        assert_eq!(float, 1.0);
        let prim_value: ValuePrimitiveType = (-1_f64).into();
        let double: f64 = (prim_value).into();
        assert_eq!(double, -1.0);

        let prim_value: ValuePrimitiveType = ().into();
        let unsigned: u32 = (&prim_value).into();
        assert_eq!(unsigned, 0);
        let prim_value: ValuePrimitiveType = ().into();
        let signed: i32 = (&prim_value).into();
        assert_eq!(signed, 0);
        let prim_value: ValuePrimitiveType = ().into();
        let unsigned: u32 = (prim_value).into();
        assert_eq!(unsigned, 0);
        let prim_value: ValuePrimitiveType = ().into();
        let signed: i32 = (prim_value).into();
        assert_eq!(signed, 0);
        let prim_value: ValuePrimitiveType = ().into();
        let float: f32 = (&prim_value).into();
        assert_eq!(float, 0.0);
        let prim_value: ValuePrimitiveType = ().into();
        let double: f64 = (&prim_value).into();
        assert_eq!(double, 0.0);
        let prim_value: ValuePrimitiveType = ().into();
        let float: f32 = (prim_value).into();
        assert_eq!(float, 0.0);
        let prim_value: ValuePrimitiveType = ().into();
        let double: f64 = (prim_value).into();
        assert_eq!(double, 0.0);

        let prim_value: ValuePrimitiveType = true.into();
        let bool_val: bool = (prim_value).into();
        assert!(bool_val);
        let prim_value: ValuePrimitiveType = true.into();
        let bool_val: bool = (&prim_value).into();
        assert!(bool_val);

        let prim_value: ValuePrimitiveType = false.into();
        let bool_val: bool = (prim_value).into();
        assert!(!bool_val);
        let prim_value: ValuePrimitiveType = false.into();
        let bool_val: bool = (&prim_value).into();
        assert!(!bool_val);

        let prim_value: ValuePrimitiveType = ().into();
        let bool_val: bool = (&prim_value).into();
        assert!(!bool_val);
        let prim_value: ValuePrimitiveType = ().into();
        let bool_val: bool = (prim_value).into();
        assert!(!bool_val);
    }

    #[test]
    fn value_type_numbers() {
        let prim_value: ValueType = "1".into();
        let unsigned: u32 = (&prim_value).into();
        assert_eq!(unsigned, 1);
        let prim_value: ValueType = "-1".into();
        let signed: i32 = (&prim_value).into();
        assert_eq!(signed, -1);
        let prim_value: ValueType = "1".into();
        let unsigned: u32 = (prim_value).into();
        assert_eq!(unsigned, 1);
        let prim_value: ValueType = "-1".into();
        let signed: i32 = (prim_value).into();
        assert_eq!(signed, -1);
        let prim_value: ValueType = "1.0".into();
        let float: f32 = (&prim_value).into();
        assert_eq!(float, 1.0);
        let prim_value: ValueType = "-1.0".into();
        let double: f64 = (&prim_value).into();
        assert_eq!(double, -1.0);
        let prim_value: ValueType = "1.0".into();
        let float: f32 = (prim_value).into();
        assert_eq!(float, 1.0);
        let prim_value: ValueType = "-1.0".into();
        let double: f64 = (prim_value).into();
        assert_eq!(double, -1.0);

        let prim_value: ValueType = ().into();
        let unsigned: u32 = (&prim_value).into();
        assert_eq!(unsigned, 0);
        let prim_value: ValueType = ().into();
        let signed: i32 = (&prim_value).into();
        assert_eq!(signed, 0);
        let prim_value: ValueType = ().into();
        let unsigned: u32 = (prim_value).into();
        assert_eq!(unsigned, 0);
        let prim_value: ValueType = ().into();
        let signed: i32 = (prim_value).into();
        assert_eq!(signed, 0);
        let prim_value: ValueType = ().into();
        let float: f32 = (&prim_value).into();
        assert_eq!(float, 0.0);
        let prim_value: ValueType = ().into();
        let double: f64 = (&prim_value).into();
        assert_eq!(double, 0.0);
        let prim_value: ValueType = ().into();
        let float: f32 = (prim_value).into();
        assert_eq!(float, 0.0);
        let prim_value: ValueType = ().into();
        let double: f64 = (prim_value).into();
        assert_eq!(double, 0.0);

        let prim_value: ValueType = ().into();
        let bool_val: bool = (&prim_value).into();
        assert!(!bool_val);
        let prim_value: ValueType = ().into();
        let bool_val: bool = (prim_value).into();
        assert!(!bool_val);

        let prim_value: ValueType = true.into();
        let bool_val: bool = (prim_value).into();
        assert!(bool_val);
        let prim_value: ValueType = true.into();
        let bool_val: bool = (&prim_value).into();
        assert!(bool_val);

        let prim_value: ValueType = false.into();
        let bool_val: bool = (prim_value).into();
        assert!(!bool_val);
        let prim_value: ValueType = false.into();
        let bool_val: bool = (&prim_value).into();
        assert!(!bool_val);
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
                let get_f = |key: &str| {
                    mvalue
                        .get(key)
                        .and_then(|v| v.to_prim())
                        .and_then(|v| v.to_f64())
                        .unwrap_or(0.0)
                };
                Rgba::new(get_f("r"), get_f("g"), get_f("b"), get_f("a"))
            }
        }
        impl From<&MValue> for Rgba {
            fn from(mvalue: &MValue) -> Self {
                let get_f = |key: &str| {
                    mvalue
                        .get(key)
                        .and_then(|v| v.to_prim())
                        .and_then(|v| v.to_f64())
                        .unwrap_or(0.0)
                };
                Rgba::new(get_f("r"), get_f("g"), get_f("b"), get_f("a"))
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
        let back_to_vec: Vec<u64> = value.into();
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

    #[test]
    fn test_pbf_prim() {
        let mut pb = Protobuf::new();
        let string_value = PrimitiveValue::String("test".to_string());
        pb.write_message(1, &string_value);
        let uint_value = PrimitiveValue::U64(1);
        pb.write_message(2, &uint_value);
        let sint_value = PrimitiveValue::I64(-1);
        pb.write_message(3, &sint_value);
        let float_value = PrimitiveValue::F32(-1.1);
        pb.write_message(4, &float_value);
        let double_value = PrimitiveValue::F64(1.1);
        pb.write_message(5, &double_value);
        let bool_value = PrimitiveValue::Bool(true);
        pb.write_message(6, &bool_value);
        let null_value = PrimitiveValue::Null;
        pb.write_message(7, &null_value);

        let bytes = pb.take();

        let mut pb_read = Protobuf::from(bytes);

        pb_read.read_field();
        let mut read_string = PrimitiveValue::Null;
        pb_read.read_message(&mut read_string);
        assert_eq!(read_string, string_value);

        pb_read.read_field();
        let mut read_uint = PrimitiveValue::Null;
        pb_read.read_message(&mut read_uint);
        assert_eq!(read_uint, uint_value);

        pb_read.read_field();
        let mut read_sint = PrimitiveValue::Null;
        pb_read.read_message(&mut read_sint);
        assert_eq!(read_sint, sint_value);

        pb_read.read_field();
        let mut read_float = PrimitiveValue::Null;
        pb_read.read_message(&mut read_float);
        assert_eq!(read_float, float_value);

        pb_read.read_field();
        let mut read_double = PrimitiveValue::Null;
        pb_read.read_message(&mut read_double);
        assert_eq!(read_double, double_value);

        pb_read.read_field();
        let mut read_bool = PrimitiveValue::Null;
        pb_read.read_message(&mut read_bool);
        assert_eq!(read_bool, bool_value);

        pb_read.read_field();
        let mut read_null = PrimitiveValue::Null;
        pb_read.read_message(&mut read_null);
        assert_eq!(read_null, null_value);
    }
}
