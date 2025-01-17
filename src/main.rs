use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_scene)
        .add_systems(Startup, setup_objects)
        .add_systems(Update, player_movement)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // ground
    commands
        .spawn(Collider::cuboid(25.0, 0.1, 25.0))
        .insert(Transform::from_xyz(0.0, -2.0, 0.0))
        .insert(RigidBody::Fixed);

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
        .insert(Transform::from_xyz(0.0, 1.0, 0.0))
        .insert(Velocity::default())
        .insert(GravityScale(1.0))
        .insert(Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        })
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z);
}

#[derive(Component)]
struct Player;

fn player_movement(
    mut query: Query<(&mut Velocity, &Transform), (With<RigidBody>, With<Player>)>,
    camera_query: Query<&Transform, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    const WALK: f32 = 3.25;
    const RUN: f32 = 5.0;
    const JUMP_VEL: f32 = 3.25;

    let Ok(cam) = camera_query.get_single() else { return };

    if let Ok((mut vel, trans)) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        let forward = Vec3::new(cam.forward().x, 0.0, cam.forward().z).normalize();
        let right = Vec3::new(cam.right().x, 0.0, cam.right().z).normalize();

        if input.pressed(KeyCode::KeyW) {
            direction += forward;
        }
        if input.pressed(KeyCode::KeyS) {
            direction -= forward;
        }
        if input.pressed(KeyCode::KeyD) {
            direction += right;
        }
        if input.pressed(KeyCode::KeyA) {
            direction -= right;
        }
        if input.pressed(KeyCode::Space) {
            vel.linvel.y = JUMP_VEL;
        }

        let speed = if input.pressed(KeyCode::ShiftLeft) {
            RUN
        } else {
            WALK
        };

        if direction != Vec3::ZERO {
            direction = direction.normalize();
            let target_velocity = direction * speed;

            vel.linvel.x = target_velocity.x;
            vel.linvel.z = target_velocity.z;
        } else {
            // what we do when we stop
            vel.linvel.x *= 0.95;
            vel.linvel.z *= 0.95;

        }

    }
}
