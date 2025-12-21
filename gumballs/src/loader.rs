use std::{collections::HashMap, time::Duration};

use avian3d::prelude::*;
use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    AMBIENT_BRIGHTNESS, AMBIENT_COLOR, AvailableBall, BACKGROUND_COLOR, BALL_RAD, BallCategory,
    CAM_TRANSFORM, FLOOR_COLOR, MACHINE_LIGHT_INTENSITY, MACHINE_LIGHT_POSITIONS,
    MACHINE_LIGHT_RANGE, MachineLight, SPOTLIGHT_INNER_ANGLE, SPOTLIGHT_INTENSITY,
    SPOTLIGHT_OUTER_ANGLE, SPOTLIGHT_POS, VisState, js_bindings,
};

#[derive(Resource)]
struct LoadingData {
    assets_to_load: Vec<UntypedHandle>,
    facts_loaded: bool,
}

#[derive(Resource)]
pub struct BallAssets {
    pub ball_mesh: Handle<Mesh>,
    pub ball_materials: HashMap<BallCategory, Handle<StandardMaterial>>,
}

pub fn loader_plugin(app: &mut App) {
    app.insert_resource(LoadingData {
        assets_to_load: vec![],
        facts_loaded: false,
    });
    app.add_systems(
        Startup,
        (
            setup_camera,
            setup_spotlight,
            setup_scene,
            setup_ball_assets,
            setup_floor,
            setup_machine_lights,
        ),
    );
    app.add_observer(on_gumball_info_available);
    app.add_systems(
        Update,
        (
            start_if_done,
            report_progress.run_if(on_timer(Duration::from_millis(100))),
        )
            .run_if(in_state(VisState::Loading)),
    );
    app.add_systems(
        OnExit(VisState::Loading),
        (
            add_machine_physics,
            js_bindings::done_loading,
        ),
    );
}

fn setup_camera(mut commands: Commands) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    commands.insert_resource(AmbientLight {
        color: AMBIENT_COLOR,
        brightness: AMBIENT_BRIGHTNESS,
        affects_lightmapped_meshes: true,
    });

    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 30.0_f32.to_radians(),
            ..default()
        }),
        *CAM_TRANSFORM,
    ));
}

fn setup_spotlight(mut commands: Commands) {
    commands.spawn((
        SpotLight {
            intensity: SPOTLIGHT_INTENSITY,
            radius: 0.0,
            shadows_enabled: true,
            outer_angle: SPOTLIGHT_OUTER_ANGLE,
            inner_angle: SPOTLIGHT_INNER_ANGLE,
            ..default()
        },
        Transform::from_translation(SPOTLIGHT_POS).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn setup_machine_lights(mut commands: Commands) {
    for pos in MACHINE_LIGHT_POSITIONS {
        commands.spawn((
            PointLight {
                intensity: MACHINE_LIGHT_INTENSITY,
                shadows_enabled: true,
                radius: 0.,
                range: MACHINE_LIGHT_RANGE,
                ..default()
            },
            Transform::from_translation(pos),
            MachineLight::default(),
        ));
    }
}

fn setup_floor(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let cuboid_size = Vec3::new(1000., 0.05, 1000.);

    commands.spawn((
        Mesh3d(mesh_assets.add(Cuboid::from_size(cuboid_size))),
        MeshMaterial3d(material_assets.add(StandardMaterial {
            base_color: FLOOR_COLOR,
            reflectance: 0.5,
            ..default()
        })),
        RigidBody::Static,
        Collider::cuboid(cuboid_size.x, cuboid_size.y, cuboid_size.z),
    ));
}

fn setup_scene(
    mut commands: Commands,
    mut loading_data: ResMut<LoadingData>,
    asset_server: Res<AssetServer>,
) {
    let scene_handle =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("gumball_machine.glb"));
    loading_data
        .assets_to_load
        .push(scene_handle.clone().untyped());
    commands.spawn(SceneRoot(scene_handle));
}

fn setup_ball_assets(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    use BallCategory::*;

    let ball_mesh = mesh_assets.add(BallCategory::mesh());
    let ball_materials = [
        (
            PersonalProject,
            material_assets.add(PersonalProject.material()),
        ),
        (Event, material_assets.add(Event.material())),
        (Experience, material_assets.add(Experience.material())),
        (Tidbit, material_assets.add(Tidbit.material())),
    ]
    .into();

    commands.insert_resource(BallAssets {
        ball_mesh,
        ball_materials,
    });
}

fn on_gumball_info_available(
    available: On<js_bindings::GumballsAvailable>,
    mut commands: Commands,
    mut loading_data: ResMut<LoadingData>
) {
    for ball in available.0.iter() {
        commands.spawn(ball.clone());
    }

    loading_data.facts_loaded = true;
}

fn start_if_done(
    mut next_state: ResMut<NextState<VisState>>,
    asset_server: Res<AssetServer>,
    loading_data: Res<LoadingData>,
) {
    if !loading_data.facts_loaded {
        return;
    }

    for to_load in &loading_data.assets_to_load {
        if !asset_server.is_loaded_with_dependencies(to_load) {
            return;
        }
    }

    // all assets loaded!
    *next_state = NextState::Pending(VisState::Filling);
}

fn add_machine_physics(
    mut commands: Commands,
    query: Query<(Entity, &Mesh3d, &Name)>,
    mesh_assets: Res<Assets<Mesh>>,
) {
    for (entity, mesh, name) in &query {
        if name.as_str().starts_with("inner") {
            dbg!("found it");
            let Some(mesh) = mesh_assets.get(&mesh.0) else {
                return;
            };
            dbg!("was loaded");
            commands.entity(entity).insert((
                RigidBody::Static,
                Collider::trimesh_from_mesh(mesh).unwrap(),
                Friction::new(0.05),
                CollisionMargin(0.02),
            ));
        }
    }
}

fn report_progress(loading_data: Res<LoadingData>, asset_server: Res<AssetServer>) {
    let num_loaded = loading_data
        .assets_to_load
        .iter()
        .filter(|x| asset_server.is_loaded(*x))
        .count();
    let total_assets = loading_data.assets_to_load.len();
    let balls_loaded = loading_data.facts_loaded;

    let progress_str =
        format!("WASM loading\nAssets loaded: {num_loaded}/{total_assets}\nBalls loaded: {balls_loaded}\n");

    js_bindings::loading_progress(progress_str);
}
