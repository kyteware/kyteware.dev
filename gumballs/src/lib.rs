mod dropping;
mod loader;
mod waiting;
mod run;
pub mod js_bindings;

use std::sync::LazyLock;

use bevy::prelude::*;

pub use run::run;
pub use dropping::*;
pub use loader::*;
use serde::Deserialize;
pub use waiting::*;

pub const DISTANCE_AWAY: f32 = 10.;
pub const BALL_RAD: f32 = 0.29;
pub const FAKE_GRAVITY: f32 = -9.81 / 20.;
pub const FLOOR_Y_BOTTOM: f32 = 2.0;
pub const END_BALL_LOCATION: Vec3 = Vec3::new(0.573791, 1.4756, 0.47438);
pub static CAM_TRANSFORM: LazyLock<Transform> = LazyLock::new(|| Transform::from_xyz(20.7, 6.12, 7.4).looking_at(Vec3::new(0., 3., 0.), Vec3::Y) );

#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum VisState {
    Loading,
    Waiting,
    Dropping
}

#[derive(Debug, Component, Clone, Deserialize)]
pub struct Ball {
    pub id: u32,
    pub category: BallCategory
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum BallCategory {
    PersonalProject,
    Event,
    Experience,
    Tidbit
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

impl BallCategory {
    fn mesh() -> impl Into<Mesh> {
        return Sphere::new(BALL_RAD);
    }

    fn material(self) -> impl Into<StandardMaterial> {
        match self {
            BallCategory::PersonalProject => Color::linear_rgb(0.75, 0.25, 0.25),
            BallCategory::Event => Color::linear_rgb(0.25, 0.75, 0.25),
            BallCategory::Experience => Color::linear_rgb(0.25, 0.25, 0.75),
            BallCategory::Tidbit => Color::linear_rgb(0.5, 0.5, 0.5),
        }
    }
}
