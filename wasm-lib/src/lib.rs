use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn parse(inst: String) -> String {
    return inst + " Yeah!"
}

#[derive(Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u8,
    xyz: String,
}

#[wasm_bindgen]
pub fn get_person() -> JsValue {
    let person = Person {
        name: "John".to_string(),
        age: 32,
        xyz: "xyz".to_string(),
    };
    return JsValue::from_serde(&person).unwrap();
}

#[test]
fn add_test() {
    assert_eq!(1 + 1, add(1, 1));
}