// #[cfg(test)]
// mod tests {
//     extern crate alloc;

//     use std::str;

//     use alloc::string::String;
//     // use alloc::vec::Vec;

//     use s2json::{MValueDeserialize, PrimitiveValue, ValueType};
//     use s2json_derive::Deserialize as S2Deserialize;
//     use serde::{Deserialize as SerdeDeserialize, Serialize as SerdeSerialize};

//     #[test]
//     fn basic_test() {
//         #[derive(
//             S2Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq, Default,
//         )]
//         pub struct TestStruct {
//             pub name: String,
//             pub value: u32,
//         }

//         let test_struct = TestStruct { name: "example".to_string(), value: 42 };

//         let mvalue = test_struct.to_mvalue(); // Ensure this method exists
//         println!("{:?}", mvalue); // Debug output
//         assert_eq!(
//             mvalue,
//             Value::from([
//                 ("name".into(), ValueType::Primitive(PrimitiveValue::String("example".into()))),
//                 ("value".into(), ValueType::Primitive(PrimitiveValue::U64(42))),
//             ])
//         );
//     }

//     #[test]
//     fn unsigned_test() {
//         #[derive(
//             S2Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq, Default,
//         )]
//         pub struct TestStruct {
//             pub a: u8,
//             pub b: u16,
//             pub c: u32,
//             pub d: u64,
//         }

//         let test_struct = TestStruct { a: 1, b: 2, c: 3, d: 4 };

//         let mvalue = test_struct.to_mvalue(); // Ensure this method exists
//         println!("{:?}", mvalue); // Debug output
//         assert_eq!(
//             mvalue,
//             Value::from([
//                 ("a".into(), ValueType::Primitive(PrimitiveValue::U64(1))),
//                 ("b".into(), ValueType::Primitive(PrimitiveValue::U64(2))),
//                 ("c".into(), ValueType::Primitive(PrimitiveValue::U64(3))),
//                 ("d".into(), ValueType::Primitive(PrimitiveValue::U64(4))),
//             ])
//         );
//     }

//     #[test]
//     fn signed_test() {
//         #[derive(
//             S2Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq, Default,
//         )]
//         pub struct TestStruct {
//             pub a: i8,
//             pub b: i16,
//             pub c: i32,
//             pub d: i64,
//         }

//         let test_struct = TestStruct { a: 1, b: 2, c: 3, d: 4 };

//         let mvalue = test_struct.to_mvalue(); // Ensure this method exists
//         println!("{:?}", mvalue); // Debug output
//         assert_eq!(
//             mvalue,
//             Value::from([
//                 ("a".into(), ValueType::Primitive(PrimitiveValue::I64(1))),
//                 ("b".into(), ValueType::Primitive(PrimitiveValue::I64(2))),
//                 ("c".into(), ValueType::Primitive(PrimitiveValue::I64(3))),
//                 ("d".into(), ValueType::Primitive(PrimitiveValue::I64(4))),
//             ])
//         );
//     }

//     #[test]
//     fn float_test() {
//         #[derive(
//             S2Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq, Default,
//         )]
//         pub struct TestStruct {
//             pub a: f32,
//             pub b: f64,
//         }

//         let test_struct = TestStruct { a: 1.0, b: 2.0 };

//         let mvalue = test_struct.to_mvalue(); // Ensure this method exists
//         println!("{:?}", mvalue); // Debug output
//         assert_eq!(
//             mvalue,
//             Value::from([
//                 ("a".into(), ValueType::Primitive(PrimitiveValue::F64(1.0))),
//                 ("b".into(), ValueType::Primitive(PrimitiveValue::F64(2.0))),
//             ])
//         );
//     }

//     #[test]
//     fn bool_test() {
//         #[derive(
//             S2Deserialize, SerdeSerialize, SerdeDeserialize, Debug, Clone, PartialEq, Default,
//         )]
//         pub struct TestStruct {
//             pub a: bool,
//         }

//         let test_struct = TestStruct { a: true };

//         let mvalue = test_struct.to_mvalue(); // Ensure this method exists
//         println!("{:?}", mvalue); // Debug output
//         assert_eq!(
//             mvalue,
//             Value::from([("a".into(), ValueType::Primitive(PrimitiveValue::Bool(true)))]),
//         );
//     }

//     #[test]
//     fn nested_object_test() {
//         #[derive(S2Deserialize, SerdeSerialize, SerdeDeserialize, Debug)]
//         pub struct NestedStruct {
//             a: String,
//             b: u32,
//         }
//         #[derive(S2Deserialize, SerdeSerialize, SerdeDeserialize, Debug)]
//         pub struct TestStruct {
//             pub a: NestedStruct,
//             pub b: u32,
//         }

//         let test_struct = TestStruct { a: NestedStruct { a: "a".into(), b: 1 }, b: 2 };

//         let mvalue = test_struct.to_mvalue(); // Ensure this method exists
//         println!("{:?}", mvalue); // Debug output
//         assert_eq!(
//             mvalue,
//             Value::from([
//                 (
//                     "a".into(),
//                     ValueType::Nested(Value::from([
//                         ("a".into(), ValueType::Primitive(PrimitiveValue::String("a".into()))),
//                         ("b".into(), ValueType::Primitive(PrimitiveValue::U64(1))),
//                     ])),
//                 ),
//                 ("b".into(), ValueType::Primitive(PrimitiveValue::U64(2))),
//             ])
//         );
//     }

//     #[test]
//     fn test_option() {
//         #[derive(S2Deserialize, SerdeSerialize, SerdeDeserialize, Debug)]
//         pub struct TestStruct {
//             pub a: Option<u32>,
//         }

//         let test_struct = TestStruct { a: Some(1) };

//         let mvalue = test_struct.to_mvalue(); // Ensure this method exists
//         println!("{:?}", mvalue); // Debug output
//         assert_eq!(
//             mvalue,
//             Value::from([("a".into(), ValueType::Primitive(PrimitiveValue::U64(1)))]),
//         );

//         let test_struct = TestStruct { a: None };

//         let mvalue = test_struct.to_mvalue(); // Ensure this method exists
//         println!("{:?}", mvalue); // Debug output
//         assert_eq!(
//             mvalue,
//             Value::from([("a".into(), ValueType::Primitive(PrimitiveValue::Null))]),
//         );
//     }
// }
