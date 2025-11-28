mod dropping;
mod loader;
mod waiting;
pub mod js_bindings;

use std::sync::LazyLock;

use bevy::prelude::*;

pub use dropping::*;
pub use loader::*;
pub use waiting::*;

pub const DISTANCE_AWAY: f32 = 10.;
pub const BALL_RAD: f32 = 0.30;
pub const FAKE_GRAVITY: f32 = -9.81 / 20.;
pub const FLOOR_Y_BOTTOM: f32 = 2.0;
pub const END_BALL_LOCATION: Vec3 = Vec3::new(0.573791, 1.5128, 1.00652);
pub static CAM_TRANSFORM: LazyLock<Transform> = LazyLock::new(|| Transform::from_xyz(20.7, 6.12, 7.4).looking_at(Vec3::new(0., 3., 0.), Vec3::Y) );

#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum VisState {
    Loading,
    Waiting,
    Dropping
}

#[derive(Debug, Component)]
pub struct Ball {
    pub id: u32,
    pub category: BallCategory
}

#[derive(Component)]
pub struct AvailableBall;

#[derive(Component, Default)]
pub struct DroppingBall {
    pub velocity: f32
}

#[derive(Component)]
pub struct FinishedBall;

#[derive(Component)]
pub struct MyButton;

#[derive(Clone, Copy, Debug)]
pub enum BallCategory {
    PersonalProject,
    Event,
    Experience,
    Tidbit
}
