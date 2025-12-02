use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use wasm_bindgen::prelude::*;

use crate::{dropping_plugin, js_bindings::js_binding_plugin, loader_plugin, waiting_plugin, VisState};

#[wasm_bindgen]
pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#gumball-canvas".into()),
                ..default()
            }),
            ..default()
        }))
        // .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(dropping_plugin)
        .add_plugins(loader_plugin)
        .add_plugins(waiting_plugin)
        .add_plugins(js_binding_plugin)
        .insert_state(VisState::Loading)
        .run();
}