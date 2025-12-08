use bevy::prelude::*;
use rand::Rng;

use crate::{VisState, MACHINE_LIGHT_AVG_TIME_OFF, MACHINE_LIGHT_AVG_TIME_ON, MACHINE_LIGHT_INTENSITY};

pub fn machine_lights_plugin(app: &mut App) {
    app.add_systems(Update, process_machine_light.run_if(not(in_state(VisState::Loading))));
}

#[derive(Component, Default)]
pub struct MachineLight {
    on: bool,
    until: f64
}

fn process_machine_light(mut query: Query<(&mut MachineLight, &mut PointLight)>, time: Res<Time>) {
    let now = time.elapsed_secs_f64();
    for (mut machine_light, mut point_light) in &mut query {
        if now < machine_light.until {
            continue;
        }

        machine_light.on = !machine_light.on;
        machine_light.until = now + gen_wait_time(machine_light.on);

        match machine_light.on {
            true => point_light.intensity = MACHINE_LIGHT_INTENSITY,
            false => point_light.intensity = 0.
        }
    }
}

fn gen_wait_time(on: bool) -> f64 {
    let mut rng = rand::rng();
    match on {
        true => rng.random_range(0.0..MACHINE_LIGHT_AVG_TIME_ON * 2.),
        false => rng.random_range(0.0..MACHINE_LIGHT_AVG_TIME_OFF * 2.)
    }
}