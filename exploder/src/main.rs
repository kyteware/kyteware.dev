use bevy::prelude::*;
use avian3d::prelude::*;
use exploder::{dropping_plugin, loader_plugin, waiting_plugin, VisState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(dropping_plugin)
        .add_plugins(loader_plugin)
        .add_plugins(waiting_plugin)
        .insert_state(VisState::Loading)
        .run();
}

