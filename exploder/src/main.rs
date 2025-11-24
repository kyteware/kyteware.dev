use std::sync::LazyLock;

use bevy::{prelude::*, window::WindowResolution};
use avian3d::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

const DISTANCE_AWAY: f32 = 10.;
const BALL_RAD: f32 = 0.30;
static CAM_TRANSFORM: LazyLock<Transform> = LazyLock::new(|| Transform::from_xyz(20.7, 6.12, 7.4).looking_at(Vec3::new(0., 3., 0.), Vec3::Y) );

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsPlugins::default())
        .init_resource::<Game>()
        .add_systems(Startup, (setup_camera, setup_lights, setup_scene, setup_button))
        .add_systems(Update, (solidify_box, remove_ball.run_if(button_checker))) // during update because we wait for gltf to finish loading
        .run();
}

#[derive(Resource, Default)]
struct Game {

}

#[derive(Component)]
struct AvailableBall;

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
            RigidBody::Dynamic,
            Collider::sphere(BALL_RAD),
                Friction::new(0.05),
            AvailableBall
        ));

        commands.spawn((
            Mesh3d(mesh_assets.add(Sphere::new(BALL_RAD))),
            MeshMaterial3d(material_assets.add(Color::srgb_u8(255, 144, 100))),
            // Transform::from_xyz(0.0, 3., 1.0).rotate_local_x(0.2)
            Transform::from_rotation(Quat::from_rotation_x(-1.1)).with_translation(Vec3::new(-0.5, 3. + (0.41* i as f32), 0.)),
            RigidBody::Dynamic,
            Collider::sphere(BALL_RAD),
                Friction::new(0.05),
            AvailableBall
        ));
    }

    commands.spawn(
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("gumball_machine.glb")))
    );
}

fn solidify_box(mut commands: Commands, query: Query<(Entity, &Mesh3d, &Name)>, mut done: Local<bool>, mesh_assets: Res<Assets<Mesh>>) {
    if *done {
        return;
    }
    for (entity, mesh, name) in &query {
        if name.as_str().starts_with("inner-box-mesh") {
            let Some(mesh) = mesh_assets.get(&mesh.0) else {
                return;
            };
            commands.entity(entity).insert((
                RigidBody::Static,
                Collider::trimesh_from_mesh(mesh).unwrap(),
                Friction::new(0.05),
                CollisionMargin(0.1)
            ));
            *done = true;
        }
    }
}

#[derive(Component)]
struct MyButton;

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

fn button_checker(query: Query<&Interaction, With<MyButton>>, mut already_pressed: Local<bool>) -> bool {
    let interaction = query.single().unwrap();

    match (*already_pressed, *interaction == Interaction::Pressed) {
        (false, true) => {
            *already_pressed = true;
            return true;
        }
        (true, false) => {
            *already_pressed = false;
        }
        _ => {}
    }

    return false;
}

fn remove_ball(mut commands: Commands, mut query: Query<(Entity, &mut Transform), With<AvailableBall>>) {
    let lowest = query.iter_mut().min_by_key(|t| (t.1.translation.y * 10000.) as i64);

    if let Some((lowest_entity, mut lowest_translation)) = lowest {
        lowest_translation.translation = Vec3::splat(DISTANCE_AWAY / 2.);
        commands.entity(lowest_entity).remove::<AvailableBall>();
    }
}

