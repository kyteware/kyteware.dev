use std::sync::LazyLock;

use bevy::{prelude::*, window::WindowResolution};
use avian3d::prelude::*;
use exploder::{js_bindings::test_extra, loader_plugin, running_plugin, AvailableBall, MyButton, VisState};
use wasm_bindgen::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(loader_plugin)
        .add_plugins(running_plugin)
        .insert_state(VisState::Loading)
        .run();
}

