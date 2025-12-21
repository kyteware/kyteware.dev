use std::{f32, time::Duration};

use avian3d::prelude::*;
use bevy::{prelude::*, time::common_conditions::on_timer};
use rand::{rng, Rng};

use crate::{js_bindings, AvailableBall, Ball, BallAssets, VisState, BALL_RAD, BALL_STARTING_COORDS};

const FILL_SIDE_VEL: f32 = 7.;
const FILL_DOWN_VEL_MAX: f32 = 15.;

pub fn filling_plugin(app: &mut App) {
    app.add_systems(Update, insert_ball.run_if(in_state(VisState::Filling).and(on_timer(Duration::from_millis(150)))));
    app.add_systems(OnExit(VisState::Filling), js_bindings::done_filling);
}

fn insert_ball(mut commands: Commands, query: Query<(Entity, &Ball), Without<AvailableBall>>, mut next_state: ResMut<NextState<VisState>>, ball_assets: Res<BallAssets>) {
    let Some((entity, ball)) = query.iter().nth(0) else {
        *next_state = NextState::Pending(VisState::Waiting);
        return;
    };

    let vel_angle: f32 = rng().random_range(0.0..(f32::consts::TAU));
    let vel = Vec3::new(
        FILL_SIDE_VEL * vel_angle.cos(),
        FILL_DOWN_VEL_MAX * rng().random_range(0.5..1.),
        FILL_SIDE_VEL * vel_angle.sin()
    );
    commands.entity(entity).insert((
            AvailableBall,
            Mesh3d(ball_assets.ball_mesh.clone()),
            MeshMaterial3d(ball_assets.ball_materials[&ball.category].clone()),
            Transform::from_translation(BALL_STARTING_COORDS),
            RigidBody::Dynamic,
            Collider::sphere(BALL_RAD),
            Friction::new(0.05),
            LinearVelocity(vel)
    ));
}