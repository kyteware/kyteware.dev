use std::sync::LazyLock;

use bevy::{prelude::*, window::WindowResolution};
use avian3d::prelude::*;
use exploder::{loader_plugin, AvailableBall, MyButton, VisState};
use wasm_bindgen::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(loader_plugin)
        .insert_state(VisState::Loading)
        .add_systems(Update, remove_ball.run_if(button_checker))
        .run();
}


fn button_checker(query: Query<&Interaction, With<MyButton>>, mut already_pressed: Local<bool>) -> bool {
    let interaction = query.single().unwrap();

    match (*already_pressed, *interaction == Interaction::Pressed) {
        (false, true) => {
            *already_pressed = true;
            return true;
        }
        (true, false) => {
            *already_pressed = false;
        }
        _ => {}
    }

    return false;
}

fn remove_ball(mut commands: Commands, mut query: Query<(Entity, &mut Transform), With<AvailableBall>>) {
    let lowest = query.iter_mut().min_by_key(|t| (t.1.translation.y * 10000.) as i64);

    if let Some((lowest_entity, mut lowest_translation)) = lowest {
        lowest_translation.translation = Vec3::splat(100.);
        commands.entity(lowest_entity).remove::<AvailableBall>();
    }

    if test_extra(3.) == 4. {
        test_extra(5.);
    }
}

#[wasm_bindgen]
extern "C" {
    fn test_extra(value: f32) -> f32;
}