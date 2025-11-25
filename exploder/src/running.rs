use bevy::prelude::*;

use crate::{js_bindings, AvailableBall, VisState};

pub fn running_plugin(app: &mut App) {
    app.add_systems(Update, remove_ball.run_if(button_checker.and(in_state(VisState::Waiting))));
}

fn button_checker(query: Query<&Interaction, With<Button>>, mut already_pressed: Local<bool>) -> bool {
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

    if js_bindings::test_extra(3.) == 4. {
        js_bindings::test_extra(5.);
    }
}