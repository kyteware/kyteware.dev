use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub enum GumballKind {
    PersonalProject = "personal_project",
    Event = "event",
    Experience = "experience",
    TidBit = "tidbit"
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Gumball {
    pub id: u32,
    pub kind: GumballKind
}

#[wasm_bindgen]
pub fn new_gumball(id: u32) -> Gumball {
    return Gumball { id, kind: GumballKind::PersonalProject }
}

#[wasm_bindgen]
extern "C" {
    pub fn getGumballs(); // -> Vec<Gumball>;
}