use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{js_bindings, AvailableBall, DroppingBall, VisState};

pub fn waiting_plugin(app: &mut App) {
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

fn remove_ball(mut commands: Commands, query: Query<(Entity, &Transform), With<AvailableBall>>, mut next_state: ResMut<NextState<VisState>>) {
    let lowest = query.iter().min_by_key(|t| (t.1.translation.y * 10000.) as i64);

    if let Some((lowest_entity, _)) = lowest {
        commands.entity(lowest_entity)
            .remove::<(AvailableBall)>().insert((DroppingBall::default(), RigidBodyDisabled));
    }

    *next_state = NextState::Pending(VisState::Dropping);

    if js_bindings::test_extra(3.) == 4. {
        js_bindings::test_extra(5.);
    }
}
