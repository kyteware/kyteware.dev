use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn test_extra(value: f32) -> f32;
}