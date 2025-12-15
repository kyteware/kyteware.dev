use std::sync::RwLock;

use bevy_channel_trigger::{ChannelSender, ChannelTriggerApp};
use wasm_bindgen::prelude::*;
use bevy::prelude::*;

use crate::Ball;

static GUMBALLS_AVAILABLE_EVENT_SENDER: RwLock<Option<ChannelSender<GumballsAvailable>>> = RwLock::new(None);
static GUMBALL_DROP_EVENT_SENDER: RwLock<Option<ChannelSender<GumballDrop>>> = RwLock::new(None);
static GUMBALL_DISCARD_EVENT_SENDER: RwLock<Option<ChannelSender<GumballDiscard>>> = RwLock::new(None);

#[wasm_bindgen]
extern "C" {
    /// send loading progress
    #[wasm_bindgen(js_name = loadingProgress)]
    pub fn loading_progress(progress: String);
    /// relays information about which ball fell
    #[wasm_bindgen(js_name = doneDropping)]
    pub fn done_dropping(id: u32);
}

#[derive(Event)]
pub struct GumballsAvailable(pub Vec<Ball>);

#[derive(Event)]
pub struct GumballDrop;

#[derive(Event)]
pub struct GumballDiscard;

pub fn js_binding_plugin(app: &mut App) {
    let mut gumballs_available_sender = GUMBALLS_AVAILABLE_EVENT_SENDER.write().unwrap();
    let mut gumball_drop_sender = GUMBALL_DROP_EVENT_SENDER.write().unwrap();
    let mut gumball_discard_sender = GUMBALL_DISCARD_EVENT_SENDER.write().unwrap();

    *gumballs_available_sender = Some(app.add_channel_trigger::<GumballsAvailable>());
    *gumball_drop_sender = Some(app.add_channel_trigger::<GumballDrop>());
    *gumball_discard_sender = Some(app.add_channel_trigger::<GumballDiscard>());
}

#[wasm_bindgen]
pub fn gumballs_available(raw_gumballs: JsValue) -> Result<(), JsValue> {
    let gumballs: Vec<Ball> = serde_wasm_bindgen::from_value(raw_gumballs)?;
    GUMBALLS_AVAILABLE_EVENT_SENDER.read()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(GumballsAvailable(gumballs));

    Ok(())
}

#[wasm_bindgen]
pub fn drop_gumball() {
    GUMBALL_DROP_EVENT_SENDER.read()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(GumballDrop);
}

#[wasm_bindgen]
pub fn discard_gumball() {
    GUMBALL_DISCARD_EVENT_SENDER.read()
        .unwrap()
        .as_ref()
        .unwrap()
        .send(GumballDiscard);
}
