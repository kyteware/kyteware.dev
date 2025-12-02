use js_sys::Reflect;
use wasm_bindgen::prelude::*;

use crate::{Ball, BallCategory};

#[wasm_bindgen]
extern "C" {
    /// should be a list in the format ["CATEGORY": [ID, ...], ...]
    fn getGumballs() -> JsValue;
    /// polls if the gumball machine should dispense
    fn shouldDrop() -> bool;
    /// relays information about which ball fell
    pub fn dropped(id: u32);
}

pub const JS_POLL_INTERVAL: f32 = 1. / 10.;

pub fn try_get_gumballs() -> Option<Vec<Ball>> {
    let raw_gumballs = getGumballs();

    let mut gumballs = vec![];

    for (category, cat_str) in [
        (BallCategory::PersonalProject, "personal_projects"),
        (BallCategory::Experience, "experiences"),
        (BallCategory::Event, "events"),
        (BallCategory::Tidbit, "tidbits")
    ] {
        if let Ok(raw_inner_list) = Reflect::get(&raw_gumballs, &JsValue::from_str(cat_str)) {
            if !(raw_inner_list.is_array()) {
                return None; // TODO: throw error
            }
            let inner_list = js_sys::Array::from(&raw_inner_list);
            for raw_id in inner_list {
                if let Some(id) = raw_id.as_f64() {
                    gumballs.push(Ball { id: id.round() as u32, category });
                } else {
                    return None;
                }
            }
        } else {
            return None; // TODO: throw error
        }
    }

    Some(gumballs)
}

pub fn should_drop() -> bool {
    shouldDrop()
}
