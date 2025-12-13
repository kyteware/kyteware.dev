use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{js_bindings, AvailableBall, Ball, DroppingBall, FinishedBall, VisState, BALL_RAD, FAKE_GRAVITY, FINAL_BALL_LANDING_COORDS, FLOOR_Y_BOTTOM, HIDDEN_BALL_CHAMBER_COORDS};

pub fn dropping_plugin(app: &mut App) {
    app.add_systems(
        Update, 
        (
            (move_dropping_ball, start_rolling_into_slot).chain(),
            finish_rolling_into_slot
        ).run_if(in_state(VisState::Dropping)));
    app.add_systems(OnEnter(VisState::Dropping), jolt_all_balls);
}

// physics engine doesn't let us selectively disable collisions for one object, so we must 
fn move_dropping_ball(mut query: Query<(&mut DroppingBall, &mut Transform)>, time: Res<Time>) {
    for (mut dropping_ball, mut transform) in &mut query {
        dropping_ball.velocity += FAKE_GRAVITY * time.delta_secs();
        transform.translation.y += dropping_ball.velocity;
    }
}

fn start_rolling_into_slot(mut commands: Commands, query: Query<(Entity, &mut Transform), With<DroppingBall>>) {
    for (entity, mut transform) in query {
        if transform.translation.y + BALL_RAD < FLOOR_Y_BOTTOM {
            info!("done!");
            transform.translation = HIDDEN_BALL_CHAMBER_COORDS;
            commands.entity(entity)
                .remove::<(DroppingBall, RigidBodyDisabled)>()
                .insert(FinishedBall);
            // *next_state = NextState::Pending(VisState::Waiting);
        }
    }
}

fn finish_rolling_into_slot(query: Query<(&Ball, &Transform), With<FinishedBall>>, mut next_state: ResMut<NextState<VisState>>) {
    for (ball, transform) in query {
        if transform.translation.distance_squared(FINAL_BALL_LANDING_COORDS) < (BALL_RAD / 3.).powi(2) {
            *next_state = NextState::Pending(VisState::Waiting);
            info!("done rolling!");
            js_bindings::doneDropping(ball.id);
        }
    }
}

fn jolt_all_balls(mut query: Query<&mut LinearVelocity, With<AvailableBall>>) {
    for mut vel in &mut query {
        vel.0.y += 0.1;
    }
}
