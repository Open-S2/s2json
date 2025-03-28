use s2json_core::*;

#[test]
fn json_value_from_str() {
    let test_str: JSONValue = "test".into();
    assert_eq!(test_str, JSONValue::Primitive(PrimitiveValue::String("test".into())));

    let test_str: String = "test".into();
    let test_str: JSONValue = test_str.into();
    assert_eq!(test_str, JSONValue::Primitive(PrimitiveValue::String("test".into())));
    let back_to_str: String = test_str.into();
    assert_eq!(back_to_str, "test");
    let no_str = JSONValue::Primitive(PrimitiveValue::F32(1.0));
    let back_to_str: String = no_str.into();
    assert_eq!(back_to_str, "");

    let test_str: String = "test".into();
    let test_str: JSONValue = (&test_str).into();
    assert_eq!(test_str, JSONValue::Primitive(PrimitiveValue::String("test".into())));
    let back_to_str: String = (&test_str).into();
    assert_eq!(back_to_str, "test");
    let no_str = JSONValue::Primitive(PrimitiveValue::F32(1.0));
    let back_to_str: String = (&no_str).into();
    assert_eq!(back_to_str, "");
}

#[test]
fn json_value_from_nums() {
    let test_num: u32 = 1_u32;
    let test_num: JSONValue = test_num.into();
    assert_eq!(test_num, JSONValue::Primitive(PrimitiveValue::U64(1)));
    let test_num: JSONValue = (&test_num).into();
    assert_eq!(test_num, JSONValue::Primitive(PrimitiveValue::U64(1)));
    let back_to_num: u32 = (test_num.clone()).into();
    assert_eq!(back_to_num, 1_u32);
    let back_to_num: u32 = (&test_num).into();
    assert_eq!(back_to_num, 1_u32);
    let no_num = JSONValue::Primitive(PrimitiveValue::String("test".into()));
    let back_to_num: u32 = (&no_num).into();
    assert_eq!(back_to_num, 0);
    let no_num = JSONValue::Primitive(PrimitiveValue::String("1".into()));
    let back_to_num: u32 = (no_num).into();
    assert_eq!(back_to_num, 1);
    let null_val = JSONValue::Primitive(PrimitiveValue::Null);
    let back_to_num: u32 = (&null_val).into();
    assert_eq!(back_to_num, 0);
    let null_val = JSONValue::Primitive(PrimitiveValue::Null);
    let back_to_num: u32 = null_val.into();
    assert_eq!(back_to_num, 0);

    let test_num: i32 = 1_i32;
    let test_num: JSONValue = test_num.into();
    assert_eq!(test_num, JSONValue::Primitive(PrimitiveValue::I64(1)));
    let test_num: JSONValue = (&test_num).into();
    assert_eq!(test_num, JSONValue::Primitive(PrimitiveValue::I64(1)));
    let back_to_num: i32 = (test_num.clone()).into();
    assert_eq!(back_to_num, 1_i32);
    let back_to_num: i32 = (&test_num).into();
    assert_eq!(back_to_num, 1_i32);
    let no_num = JSONValue::Primitive(PrimitiveValue::String("test".into()));
    let back_to_num: i32 = (&no_num).into();
    assert_eq!(back_to_num, 0);
    let no_num = JSONValue::Primitive(PrimitiveValue::String("test".into()));
    let back_to_num: i32 = (no_num).into();
    assert_eq!(back_to_num, 0);
    let null_val = JSONValue::Primitive(PrimitiveValue::Null);
    let back_to_num: i32 = (&null_val).into();
    assert_eq!(back_to_num, 0);
    let null_val = JSONValue::Primitive(PrimitiveValue::Null);
    let back_to_num: i32 = null_val.into();
    assert_eq!(back_to_num, 0);

    let test_num: f32 = 1.0_f32;
    let test_num: JSONValue = test_num.into();
    assert_eq!(test_num, JSONValue::Primitive(PrimitiveValue::F32(1.)));
    let test_num: JSONValue = (&test_num).into();
    assert_eq!(test_num, JSONValue::Primitive(PrimitiveValue::F32(1.)));
    let back_to_num: f32 = (test_num.clone()).into();
    assert_eq!(back_to_num, 1.0_f32);
    let back_to_num: f32 = (&test_num).into();
    assert_eq!(back_to_num, 1.0_f32);
    let no_num = JSONValue::Primitive(PrimitiveValue::String("test".into()));
    let back_to_num: f32 = (&no_num).into();
    assert_eq!(back_to_num, 0.);
    let no_num = JSONValue::Primitive(PrimitiveValue::String("1.2".into()));
    let back_to_num: f32 = (no_num).into();
    assert_eq!(back_to_num, 1.2);
    let null_val = JSONValue::Primitive(PrimitiveValue::Null);
    let back_to_num: f32 = (&null_val).into();
    assert_eq!(back_to_num, 0.);
    let null_val = JSONValue::Primitive(PrimitiveValue::Null);
    let back_to_num: f32 = null_val.into();
    assert_eq!(back_to_num, 0.);

    let test_num: f64 = 1.0_f64;
    let test_num: JSONValue = test_num.into();
    assert_eq!(test_num, JSONValue::Primitive(PrimitiveValue::F64(1.)));
    let test_num: JSONValue = (&test_num).into();
    assert_eq!(test_num, JSONValue::Primitive(PrimitiveValue::F64(1.)));
    let back_to_num: f64 = (test_num.clone()).into();
    assert_eq!(back_to_num, 1.0_f64);
    let back_to_num: f64 = (&test_num).into();
    assert_eq!(back_to_num, 1.0_f64);
    let no_num = JSONValue::Primitive(PrimitiveValue::String("test".into()));
    let back_to_num: f64 = (&no_num).into();
    assert_eq!(back_to_num, 0.);
    let no_num = JSONValue::Primitive(PrimitiveValue::String("1.2".into()));
    let back_to_num: f64 = (no_num).into();
    assert_eq!(back_to_num, 1.2);
    let null_val = JSONValue::Primitive(PrimitiveValue::Null);
    let back_to_num: f64 = (&null_val).into();
    assert_eq!(back_to_num, 0.);
    let null_val = JSONValue::Primitive(PrimitiveValue::Null);
    let back_to_num: f64 = null_val.into();
    assert_eq!(back_to_num, 0.);
}

#[test]
fn json_value_from_bool() {
    let test_bool: bool = true;
    let test_bool: JSONValue = test_bool.into();
    assert_eq!(test_bool, JSONValue::Primitive(PrimitiveValue::Bool(true)));
    let test_bool: JSONValue = (&test_bool).into();
    assert_eq!(test_bool, JSONValue::Primitive(PrimitiveValue::Bool(true)));
    let back_to_bool: bool = (test_bool.clone()).into();
    assert!(back_to_bool);
    let back_to_bool: bool = (&test_bool).into();
    assert!(back_to_bool);
    let no_bool = JSONValue::Primitive(PrimitiveValue::String("test".into()));
    let back_to_bool: bool = (&no_bool).into();
    assert!(!back_to_bool);
    let no_bool = JSONValue::Primitive(PrimitiveValue::String("true".into()));
    let back_to_bool: bool = (no_bool).into();
    assert!(back_to_bool);
    let null_val = JSONValue::Primitive(PrimitiveValue::Null);
    let back_to_bool: bool = (&null_val).into();
    assert!(!back_to_bool);
    let null_val = JSONValue::Primitive(PrimitiveValue::Null);
    let back_to_bool: bool = null_val.into();
    assert!(!back_to_bool);
}

#[test]
fn json_value_from_nothing() {
    let test_nothing: JSONValue = ().into();
    assert_eq!(test_nothing, JSONValue::Primitive(PrimitiveValue::Null));
    let test_nothing: JSONValue = (&test_nothing).into();
    assert_eq!(test_nothing, JSONValue::Primitive(PrimitiveValue::Null));
    let _back_to_nothing: () = (test_nothing.clone()).into();
    let _back_to_nothing: () = (&test_nothing).into();
}

#[test]
fn json_value_from_vec() {
    let test_vec: Vec<JSONValue> =
        vec![JSONValue::Primitive(PrimitiveValue::String("test".into()))];
    let test_vec: JSONValue = test_vec.into();
    assert_eq!(
        test_vec,
        JSONValue::Array(vec![JSONValue::Primitive(PrimitiveValue::String("test".into()))])
    );
    let test_vec: JSONValue = (&test_vec).into();
    assert_eq!(
        test_vec,
        JSONValue::Array(vec![JSONValue::Primitive(PrimitiveValue::String("test".into()))])
    );
    let back_to_vec: Vec<JSONValue> = (test_vec.clone()).into();
    assert_eq!(back_to_vec, vec![JSONValue::Primitive(PrimitiveValue::String("test".into()))]);
    let back_to_vec: Vec<JSONValue> = (&test_vec).into();
    assert_eq!(back_to_vec, vec![JSONValue::Primitive(PrimitiveValue::String("test".into()))]);
    let no_vec = JSONValue::Primitive(PrimitiveValue::String("test".into()));
    let back_to_vec: Vec<JSONValue> = (&no_vec).into();
    assert_eq!(back_to_vec, vec![]);
    let null_val = JSONValue::Primitive(PrimitiveValue::Null);
    let back_to_vec: Vec<JSONValue> = (&null_val).into();
    assert_eq!(back_to_vec, vec![]);

    let test_vec: Vec<u32> = vec![1, 2, 3];
    let test_vec: JSONValue = test_vec.into();
    assert_eq!(test_vec, JSONValue::Array(vec![1_u64.into(), 2_u64.into(), 3_u64.into()]));
    let test_vec: JSONValue = (&test_vec).into();
    assert_eq!(test_vec, JSONValue::Array(vec![1_u64.into(), 2_u64.into(), 3_u64.into()]));
    let back_to_vec: Vec<u32> = (test_vec.clone()).into();
    assert_eq!(back_to_vec, vec![1, 2, 3]);

    let test_val: JSONValue = false.into();
    let back_to_vec: Vec<u32> = test_val.into();
    assert_eq!(back_to_vec, vec![] as Vec<u32>);

    let test_val: JSONValue = false.into();
    let back_to_vec: Vec<u32> = (&test_val).into();
    assert_eq!(back_to_vec, vec![] as Vec<u32>);

    let test_vec: Vec<JSONValue> =
        vec![JSONValue::Primitive(PrimitiveValue::String("test".into()))];
    let test_vec_ref: JSONValue = (&test_vec).into(); // Explicitly testing From<&Vec<T>> for JSONValue
    assert_eq!(
        test_vec_ref,
        JSONValue::Array(vec![JSONValue::Primitive(PrimitiveValue::String("test".into()))])
    );
}

#[test]
fn json_value_from_and_to_properties() {
    let string_test = JSONValue::Primitive(PrimitiveValue::String("test".into()));
    let json_prop: JSONProperties = string_test.into();
    assert_eq!(json_prop, JSONProperties::from([]));

    let string_test = JSONValue::Primitive(PrimitiveValue::String("test".into()));
    let json_prop: JSONProperties = (&string_test).into();
    assert_eq!(json_prop, JSONProperties::from([]));

    let obj_test = JSONValue::Object(Map::from([(
        "test".into(),
        JSONValue::Primitive(PrimitiveValue::String("test".into())),
    )]));
    let json_prop: JSONProperties = obj_test.clone().into();
    let back_to_obj: JSONValue = json_prop.into();
    assert_eq!(back_to_obj, obj_test);

    let obj_test = JSONValue::Object(Map::from([(
        "test".into(),
        JSONValue::Primitive(PrimitiveValue::String("test".into())),
    )]));
    let json_prop: JSONProperties = (&obj_test).into();
    let back_to_obj: JSONValue = (&json_prop).into();
    assert_eq!(back_to_obj, obj_test);
}
