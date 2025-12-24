use std::f32::consts::PI;

use bevy::prelude::*;

pub fn animation_plugins(app: &mut App) {
    app.add_systems(FixedUpdate, wiggle);
}

#[derive(Component)]
pub struct Wiggler {
    pos: Vec3,
    looking_at: Vec3,
    radius: f32,
    period: f32,
    axis: Vec3,
    arm: Vec3,
    current_angle: f32
}

impl Wiggler {
    pub fn new(pos: Vec3, looking_at: Vec3, radius: f32, period: f32) -> Self {
        let axis = (looking_at - pos).normalize();
        Wiggler {
            pos,
            looking_at,
            radius,
            period,
            axis,
            arm: axis.cross(Vec3::Y).normalize() * radius,
            current_angle: 0.
        }
    }
}

fn wiggle(query: Query<(&mut Wiggler, &mut Transform)>, step: Res<Time<Fixed>>) {
    for (mut wiggler, mut transform) in query {
        wiggler.current_angle = wiggler.current_angle
            + (2. * PI * step.timestep().as_secs_f32()) / wiggler.period;
        wiggler.current_angle %= 2. * PI;

        let rotation = Quat::from_axis_angle(wiggler.axis, wiggler.current_angle);
        let rotated_pos = wiggler.pos + rotation * wiggler.arm * wiggler.radius;

        *transform = Transform::from_translation(rotated_pos)
            .looking_at(wiggler.looking_at, Vec3::Y)
    }
}
