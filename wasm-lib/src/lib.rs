// use exec::memory::SerializableInstructionMemFields;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(inst: String) -> String {
    return inst + " Yeah!"
}

#[test]
fn add_test() {
    assert_eq!("hello Yeah!", parse("hello".to_string()));
}
