mod dropping;
pub mod js_bindings;
mod loader;
mod machine_lights;
mod run;
mod waiting;
mod filling;

use std::sync::LazyLock;

use bevy::prelude::*;
use serde::Deserialize;

pub use dropping::*;
pub use loader::*;
pub use machine_lights::*;
pub use run::run;
pub use waiting::*;
pub use filling::*;

pub const BALL_RAD: f32 = 0.29;
pub const FAKE_GRAVITY: f32 = -9.81 / 20.;
pub const FLOOR_Y_BOTTOM: f32 = 2.0;
pub const BALL_STARTING_COORDS: Vec3 = Vec3::new(0., 5.45, 0.);
pub const HIDDEN_BALL_CHAMBER_COORDS: Vec3 = Vec3::new(0.573791, 1.4756, 0.47438);
pub const FINAL_BALL_LANDING_COORDS: Vec3 = Vec3::new(0.58, 1.4, 1.27);
pub const GUMBALL_EJECT_VELOCITY: Vec3 = Vec3::new(0., 4., 1.);

pub const BACKGROUND_COLOR: Color = Color::srgb_u8(36, 27, 60);
pub const FLOOR_COLOR: Color = Color::srgb_u8(200, 200, 200);

pub const SPOTLIGHT_POS: Vec3 = Vec3::new(1.5, 15., 1.5);
pub const SPOTLIGHT_INNER_ANGLE: f32 = f32::to_radians(10.);
pub const SPOTLIGHT_OUTER_ANGLE: f32 = f32::to_radians(18.);
pub const SPOTLIGHT_INTENSITY: f32 = 1_200_000.;

pub const MACHINE_LIGHT_POSITIONS: [Vec3; 2] = [Vec3::new(0.7, 5.5, 0.), Vec3::new(-0.7, 5.5, 0.)];
pub const MACHINE_LIGHT_INTENSITY: f32 = 100_000.;
pub const MACHINE_LIGHT_AVG_TIME_ON: f64 = 7.;
pub const MACHINE_LIGHT_AVG_TIME_OFF: f64 = 0.1;
pub const MACHINE_LIGHT_RANGE: f32 = 5.;

pub const AMBIENT_BRIGHTNESS: f32 = 100.;
pub const AMBIENT_COLOR: Color = Color::srgb_u8(36, 27, 60);

pub static CAM_TRANSFORM: LazyLock<Transform> = LazyLock::new(|| {
    Transform::from_xyz(20.7, 8.5, 9.4).looking_at(Vec3::new(0., 1.5, 0.), Vec3::Y)
});

#[derive(States, Debug, Hash, PartialEq, Eq, Clone)]
pub enum VisState {
    Loading,
    Filling,
    Waiting,
    Dropping,
}

#[derive(Debug, Component, Clone, Deserialize)]
pub struct Ball {
    pub id: u32,
    pub category: BallCategory,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum BallCategory {
    PersonalProject,
    Event,
    Experience,
    Tidbit,
}

#[derive(Component)]
pub struct AvailableBall;

#[derive(Component, Default)]
pub struct DroppingBall {
    pub velocity: f32,
}

#[derive(Component)]
pub struct FinishedBall;

impl BallCategory {
    fn mesh() -> impl Into<Mesh> {
        Sphere::new(BALL_RAD)
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
