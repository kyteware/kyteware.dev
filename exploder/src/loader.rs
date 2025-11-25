use bevy::{prelude::*, window::WindowResolution};
use avian3d::prelude::*;

use crate::{AvailableBall, MyButton, VisState, BALL_RAD, CAM_TRANSFORM};

#[derive(Resource)]
struct LoadingData {
    assets_to_load: Vec<UntypedHandle>
}

pub fn loader_plugin(app: &mut App) {
    app.insert_resource(LoadingData { assets_to_load: vec![] });
    app.add_systems(
        Startup, 
        (
            setup_camera, 
            setup_lights, 
            setup_scene, 
            setup_button
        )
    );
    app.add_systems(
        Update,
        start_if_done.run_if(in_state(VisState::Loading))
    );
    app.add_systems(
        OnExit(VisState::Loading),
        (add_ball_physics, add_machine_physics)
    );
}

fn setup_camera(mut commands: Commands, mut query: Query<&mut Window, With<bevy::window::PrimaryWindow>>) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 30.0_f32.to_radians(),
            ..default()
        }),
        *CAM_TRANSFORM,
    ));

    query.single_mut().unwrap().resolution = WindowResolution::new(500, 1000);
}

fn setup_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(7., 5., 5.).looking_at(Vec3::splat(0.), Vec3::Y)
    ));
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(6.0, 8.0, 2.0),
    ));
}

fn setup_scene(
    mut commands: Commands, 
    mut loading_data: ResMut<LoadingData>,
    asset_server: Res<AssetServer>,
    mesh_assets: ResMut<Assets<Mesh>>, 
    material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let material_assets = material_assets.into_inner();
    let mesh_assets = mesh_assets.into_inner();

    for i in 0..3 {
        commands.spawn((
            Mesh3d(mesh_assets.add(Sphere::new(BALL_RAD))),
            MeshMaterial3d(material_assets.add(Color::srgb_u8(124, 144, 255))),
            // Transform::from_xyz(0.0, 3., 1.0).rotate_local_x(0.2)
            Transform::from_rotation(Quat::from_rotation_x(-1.1)).with_translation(Vec3::new(0.5, 3. + (0.5* i as f32), 0.)),
            AvailableBall
        ));

        commands.spawn((
            Mesh3d(mesh_assets.add(Sphere::new(BALL_RAD))),
            MeshMaterial3d(material_assets.add(Color::srgb_u8(255, 144, 100))),
            // Transform::from_xyz(0.0, 3., 1.0).rotate_local_x(0.2)
            Transform::from_rotation(Quat::from_rotation_x(-1.1)).with_translation(Vec3::new(-0.5, 3. + (0.41* i as f32), 0.)),
            AvailableBall
        ));
    }

    let scene_handle = asset_server.load(GltfAssetLabel::Scene(0).from_asset("gumball_machine.glb"));
    loading_data.assets_to_load.push(scene_handle.clone().untyped());
    commands.spawn(
        SceneRoot(scene_handle)
    );
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
    for to_load in &loading_data.assets_to_load {
        if !asset_server.is_loaded_with_dependencies(to_load) {
            return;
        }
    }

    // all assets loaded!
    *next_state = NextState::Pending(VisState::Waiting);
}

fn add_machine_physics(mut commands: Commands, query: Query<(Entity, &Mesh3d, &Name)>, mesh_assets: Res<Assets<Mesh>>) {
    dbg!("starting query");
    for (entity, mesh, name) in &query {
        dbg!("running query");
        if name.as_str().starts_with("inner-box-mesh") {
            dbg!("found it");
            let Some(mesh) = mesh_assets.get(&mesh.0) else {
                return;
            };
            dbg!("was loaded");
            commands.entity(entity).insert((
                RigidBody::Static,
                Collider::trimesh_from_mesh(mesh).unwrap(),
                Friction::new(0.05),
                CollisionMargin(0.1)
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
