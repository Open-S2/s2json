// This code exists strictly to ensure all exports are working as intended
// Re-add this fodler to the workspace Cargo.toml to check

use s2json::{MValue, MValueCompatible};

#[derive(MValueCompatible, Debug, Default, Clone)]
struct TestStruct {
    foo: String,
    bar: u32,
}

fn main() {
    let test = TestStruct { foo: "Hello".to_string(), bar: 42 };

    let mvalue: MValue = test.into();
    println!("{:?}", mvalue);
}
