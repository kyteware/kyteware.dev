use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    AvailableBall, DroppingBall, FinishedBall, GUMBALL_EJECT_VELOCITY, VisState, js_bindings,
};

pub fn waiting_plugin(app: &mut App) {
    app.add_observer(drop_ball);
    app.add_observer(eject_finished_balls);
}

fn drop_ball(
    _: On<js_bindings::GumballDrop>,
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<AvailableBall>>,
    mut next_state: ResMut<NextState<VisState>>,
) {
    let lowest = query
        .iter()
        .min_by_key(|t| (t.1.translation.y * 10000.) as i64);

    if let Some((lowest_entity, _)) = lowest {
        commands
            .entity(lowest_entity)
            .remove::<AvailableBall>()
            .insert((DroppingBall::default(), RigidBodyDisabled));
    }

    *next_state = NextState::Pending(VisState::Dropping);
}

fn eject_finished_balls(
    _: On<js_bindings::GumballDiscard>,
    mut commands: Commands,
    query: Query<(Entity, &mut LinearVelocity), With<FinishedBall>>,
) {
    for (entity, mut vel) in query {
        vel.0 = GUMBALL_EJECT_VELOCITY;
        commands.entity(entity).remove::<FinishedBall>();
    }
}
