use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{js_bindings, AvailableBall, DroppingBall, VisState};

pub fn waiting_plugin(app: &mut App) {
    app.add_systems(Update, remove_ball.run_if(check_should_drop.and(in_state(VisState::Waiting))));
}

fn check_should_drop(time: Res<Time>, mut last_checked: Local<f64>) -> bool {
    if time.elapsed_secs_f64() > *last_checked + (js_bindings::JS_POLL_INTERVAL as f64) {
        *last_checked = time.elapsed_secs_f64();
        js_bindings::should_drop()
    } else {
        false
    }
}

fn remove_ball(mut commands: Commands, query: Query<(Entity, &Transform), With<AvailableBall>>, mut next_state: ResMut<NextState<VisState>>) {
    let lowest = query.iter().min_by_key(|t| (t.1.translation.y * 10000.) as i64);

    if let Some((lowest_entity, _)) = lowest {
        commands.entity(lowest_entity)
            .remove::<AvailableBall>().insert((DroppingBall::default(), RigidBodyDisabled));
    }

    *next_state = NextState::Pending(VisState::Dropping);
}
