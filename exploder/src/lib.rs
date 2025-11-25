mod loader;
mod running;
pub mod js_bindings;

use std::sync::LazyLock;

use bevy::prelude::*;

pub use loader::*;
pub use running::*;

#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum VisState {
    Loading,
    Waiting,
    Dropping
}

pub const DISTANCE_AWAY: f32 = 10.;
pub const BALL_RAD: f32 = 0.30;
pub static CAM_TRANSFORM: LazyLock<Transform> = LazyLock::new(|| Transform::from_xyz(20.7, 6.12, 7.4).looking_at(Vec3::new(0., 3., 0.), Vec3::Y) );

#[derive(Component)]
pub struct AvailableBall;

#[derive(Component)]
pub struct MyButton;
