use wasm_bindgen::prelude::*;

mod interpreter;

#[wasm_bindgen]
pub fn parse(prog: String) -> String {
    return parse(prog);
}

#[wasm_bindgen]
pub fn run() -> bool {
    true
}

#[test]
fn add_test() {
    assert_eq!("hello Yeah!", parse("hello".to_string()));
}
