// use exec::memory::SerializableInstructionMemFields;
use wasm_bindgen::prelude::*;
use serde::{Serialize};

mod exec;

#[wasm_bindgen]
pub fn parse(inst: String) -> String {
    return inst + " Yeah!"
}

// #[wasm_bindgen]
// pub fn testMemField() -> exec::memory::SerializableInstructionMemFields {
//     SerializableInstructionMemFields {
//         register: "r0".to_string(),
//         value: 0,
//         specialRegister: "cpsr".to_string(),
//         specialValue: 0,
//         memAdr: 0,
//         memValue: 0,
//     }
// }

#[derive(Serialize)]
pub struct TestStruct {
    pub register: String,
    pub value: u32,
    pub specialRegister: String,
    pub specialValue: u32,
    pub memAdr: u32,
    pub memValue: u32,
}

#[wasm_bindgen]
pub fn testMemField() -> wasm_bindgen::JsValue {
    let testStruct = TestStruct {
        register: "r0".to_string(),
        value: 0,
        specialRegister: "cpsr".to_string(),
        specialValue: 0,
        memAdr: 0,
        memValue: 0,
    };

    wasm_bindgen::JsValue::from_serde(&testStruct).unwrap()
}

#[test]
fn add_test() {
    assert_eq!("hello Yeah!", parse("hello".to_string()));
}
