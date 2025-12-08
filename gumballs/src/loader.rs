use std::collections::HashMap;

use bevy::prelude::*;
use avian3d::prelude::*;

use crate::{js_bindings, AvailableBall, BallCategory, MachineLight, MyButton, VisState, AMBIENT_BRIGHTNESS, AMBIENT_COLOR, BACKGROUND_COLOR, BALL_RAD, CAM_TRANSFORM, FLOOR_COLOR, MACHINE_LIGHT_INTENSITY, MACHINE_LIGHT_POSITIONS, MACHINE_LIGHT_RANGE, SPOTLIGHT_INNER_ANGLE, SPOTLIGHT_INTENSITY, SPOTLIGHT_OUTER_ANGLE, SPOTLIGHT_POS};

#[derive(Resource)]
struct LoadingData {
    assets_to_load: Vec<UntypedHandle>,
    balls_loaded: bool
}

#[derive(Resource)]
struct BallAssets {
    ball_mesh: Handle<Mesh>,
    ball_materials: HashMap<BallCategory, Handle<StandardMaterial>>
}

pub fn loader_plugin(app: &mut App) {
    app.insert_resource(LoadingData { assets_to_load: vec![], balls_loaded: false });
    app.add_systems(
        Startup, 
        (
            setup_camera, 
            setup_spotlight, 
            setup_scene, 
            setup_button,
            setup_ball_assets,
            setup_floor,
            setup_machine_lights
        )
    );
    app.add_observer(on_gumballs_available);
    app.add_systems(Update, (
        start_if_done,
    ).run_if(in_state(VisState::Loading)));
    app.add_systems(
        OnExit(VisState::Loading),
        (add_ball_physics, add_machine_physics)
    );
}

fn setup_camera(mut commands: Commands) {
    commands.insert_resource(ClearColor(BACKGROUND_COLOR));
    commands.insert_resource(AmbientLight {
        color: AMBIENT_COLOR,
        brightness: AMBIENT_BRIGHTNESS,
        affects_lightmapped_meshes: true
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
        Transform::from_translation(SPOTLIGHT_POS).looking_at(Vec3::ZERO, Vec3::Y)
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
            MachineLight::default()
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
        Mesh3d(
            mesh_assets.add(Cuboid::from_size(cuboid_size))
        ),
        MeshMaterial3d(material_assets.add(
            StandardMaterial {
                base_color: FLOOR_COLOR,
                reflectance: 0.5,
                ..default()
            }
        )),
        RigidBody::Static,
        Collider::cuboid(cuboid_size.x, cuboid_size.y, cuboid_size.z)
    ));
}

fn setup_scene(
    mut commands: Commands, 
    mut loading_data: ResMut<LoadingData>,
    asset_server: Res<AssetServer>
) {
    let scene_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("gumball_machine.glb"));
    loading_data.assets_to_load.push(scene_handle.clone().untyped());
    commands.spawn(
        SceneRoot(scene_handle)
    );
}

fn setup_ball_assets(
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>, 
    mut material_assets: ResMut<Assets<StandardMaterial>>
) {
    use BallCategory::*;
    commands.insert_resource(BallAssets {
        ball_mesh: mesh_assets.add(BallCategory::mesh()),
        ball_materials: [
            (PersonalProject, material_assets.add(PersonalProject.material())),
            (Event, material_assets.add(Event.material())),
            (Experience, material_assets.add(Experience.material())),
            (Tidbit, material_assets.add(Tidbit.material()))
        ].into()
    });
}

fn on_gumballs_available(
    available: On<js_bindings::GumballsAvailable>,
    mut commands: Commands, 
    mut loading_data: ResMut<LoadingData>,
    ball_assets: Res<BallAssets>,
) {
    for (i, ball) in available.0.iter().enumerate() {
        commands.spawn((
            ball.clone(),
            AvailableBall,
            Mesh3d(ball_assets.ball_mesh.clone()),
            MeshMaterial3d(ball_assets.ball_materials[&ball.category].clone()),
            Transform::from_translation(Vec3::new(0.5, 3. + (0.5* i as f32), 0.))
        ));
    }

    loading_data.balls_loaded = true;
}

fn setup_button(mut commands: Commands) {
    commands.spawn((
        Node {
            ..default()
        },
        children![(
            Button,
            MyButton,
            Node {
                ..default()
            },
            children![
                Text::new("hi")
            ]
        )]
    ));
}

fn start_if_done(mut next_state: ResMut<NextState<VisState>>, asset_server: Res<AssetServer>, loading_data: Res<LoadingData>) {
    if !loading_data.balls_loaded {
        return;
    }
    
    for to_load in &loading_data.assets_to_load {
        if !asset_server.is_loaded_with_dependencies(to_load) {
            return;
        }
    }

    // all assets loaded!
    *next_state = NextState::Pending(VisState::Waiting);
}

fn add_machine_physics(mut commands: Commands, query: Query<(Entity, &Mesh3d, &Name)>, mesh_assets: Res<Assets<Mesh>>) {
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
                CollisionMargin(0.02)
            ));
        }
    }
}

fn add_ball_physics(mut commands: Commands, query: Query<Entity, With<AvailableBall>>) {
    for entity in &query {
        commands.entity(entity).insert((
            RigidBody::Dynamic,
            Collider::sphere(BALL_RAD),
            Friction::new(0.05)
        ));
    }
}
