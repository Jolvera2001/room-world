use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, setup_objects)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands
        .spawn(Camera3d {
            ..Default::default()
        })
        .insert(Transform::from_xyz(-3.0, 3.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y));
}

fn setup_objects(mut commands: Commands) {
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::capsule(
            Vec3::new(0.0, 0.0, 0.0).into(), // Start at feet
            Vec3::new(0.0, 1.8, 0.0).into(), // End at head (1.8 units tall)
            0.4,
        ))
        .insert(Player)
        .insert(Transform::from_xyz(0.0, 0.0, 0.0));
}

#[derive(Component)]
struct Player;

fn player_movement(
    mut query: Query<(&mut Velocity, &Transform), (With<RigidBody>, With<Player>)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
}
