use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{DroppingBall, FinishedBall, VisState, BALL_RAD, END_BALL_LOCATION, FAKE_GRAVITY, FLOOR_Y_BOTTOM};

pub fn dropping_plugin(app: &mut App) {
    app.add_systems(Update, (move_dropping_ball, finish_dropping_into_slot).chain().run_if(in_state(VisState::Dropping)));
}

// physics engine doesn't let us selectively disable collisions for one object, so we must 
fn move_dropping_ball(mut query: Query<(&mut DroppingBall, &mut Transform)>, time: Res<Time>) {
    for (mut dropping_ball, mut transform) in &mut query {
        dropping_ball.velocity += FAKE_GRAVITY * time.delta_secs();
        transform.translation.y += dropping_ball.velocity;
    }
}

fn finish_dropping_into_slot(mut commands: Commands, query: Query<(Entity, &mut Transform), With<DroppingBall>>, mut next_state: ResMut<NextState<VisState>>) {
    for (entity, mut transform) in query {
        if transform.translation.y + BALL_RAD < FLOOR_Y_BOTTOM {
            info!("done!");
            transform.translation = END_BALL_LOCATION;
            commands.entity(entity)
                .remove::<(DroppingBall, RigidBodyDisabled)>()
                .insert(FinishedBall);
            *next_state = NextState::Pending(VisState::Waiting);
        }
    }
}
