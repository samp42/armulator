// use exec::memory::SerializableInstructionMemFields;
use wasm_bindgen::prelude::*;

use serde::{Serialize, Deserialize};

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

#[derive(Serialize, Deserialize)]
pub struct TestStruct {
    pub reg: String,
    pub val: u32,
    pub special_reg: String,
    pub special_val: u32,
    pub mem_adr: u32,
    pub mem_val: u32,
}

#[wasm_bindgen]
pub fn test_mem_field() -> wasm_bindgen::JsValue {
    let test_struct = TestStruct {
        reg: "r0".to_string(),
        val: 0,
        special_reg: "cpsr".to_string(),
        special_val: 0,
        mem_adr: 0,
        mem_val: 0,
    };

    wasm_bindgen::JsValue::from_serde(&test_struct).unwrap()
}

#[test]
fn add_test() {
    assert_eq!("hello Yeah!", parse("hello".to_string()));
}
