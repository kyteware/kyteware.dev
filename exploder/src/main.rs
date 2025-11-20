use bevy::{ecs::query::QueryFilter, prelude::*};
use avian3d::prelude::*;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

const DISTANCE_AWAY: f32 = 10.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(PhysicsPlugins::default())
        .init_resource::<Game>()
        .add_systems(Startup, (setup_camera, setup_lights, setup_scene))
        .add_systems(Update, (checkout_cone,)) // during update because we wait for gltf to finish loading
        .run();
}

#[derive(Resource, Default)]
struct Game {

}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(DISTANCE_AWAY, DISTANCE_AWAY, DISTANCE_AWAY)
        .looking_at(Vec3::splat(0.), Vec3::Y)
    ));
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

// #[derive(Resource)]
// struct GumballScene(Handle<Gltf>);

// fn setup_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
//     dbg!("loadinggg");
//     let gumball_gltf = asset_server.load("gumball_machine.glb");
    // commands.insert_resource(GumballScene(gumball_gltf));
// }

fn setup_scene(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mesh_assets: ResMut<Assets<Mesh>>, 
    material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let material_assets = material_assets.into_inner();
    let mesh_assets = mesh_assets.into_inner();

    // commands.spawn((
    //     Mesh3d(mesh_assets.add(Circle::new(4.0))),
    //     MeshMaterial3d(material_assets.add(Color::WHITE)),
    //     Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    //     RigidBody::Static,
    //     Collider::cuboid(100., 100., 0.1)
    // ));
    for i in 0..200 {
        commands.spawn((
            Mesh3d(mesh_assets.add(Sphere::new(0.4))),
            MeshMaterial3d(material_assets.add(Color::srgb_u8(124, 144, 255))),
            // Transform::from_xyz(0.0, 3., 1.0).rotate_local_x(0.2)
            Transform::from_rotation(Quat::from_rotation_x(-1.1)).with_translation(Vec3::new(2., 8. + (2*i) as f32, 0.)),
            RigidBody::Dynamic,
            Collider::sphere(0.4)
        ));
    }

    commands.spawn(
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("gumball_machine.glb")))
    );
}

fn checkout_cone(mut commands: Commands, query: Query<(Entity, &Mesh3d, &Name)>, mut done: Local<bool>, mesh_assets: Res<Assets<Mesh>>) {
    if *done {
        return;
    }
    for (entity, mesh, name) in &query {
        if name.as_str() == "Cone" {
            let Some(mesh) = mesh_assets.get(&mesh.0) else {
                return;
            };
            commands.entity(entity).insert((
                RigidBody::Static,
                Collider::trimesh_from_mesh(mesh).unwrap(),
                CollisionMargin(0.1)
            ));
            *done = true;
        }
    }
}
