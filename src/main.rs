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
        .spawn((
            RigidBody::KinematicPositionBased,
            Collider::capsule(
                Vec3::new(0.0, 0.0, 0.0).into(), // Start at feet
                Vec3::new(0.0, 1.8, 0.0).into(), // End at head (1.8 units tall)
                0.4,
            ),
            KinematicCharacterController::default(),
            KinematicCharacterControllerOutput::default(),
            PlayerPhysics {
                velocity: Vec3::ZERO,
                acceleration: Vec3::ZERO
            },
            Transform::from_xyz(0.0, 1.0, 0.0)));
}

#[derive(Component)]
struct PlayerPhysics {
    velocity: Vec3,
    acceleration: Vec3
}

fn player_movement(
    mut query: Query<(&mut KinematicCharacterController, &mut PlayerPhysics, &KinematicCharacterControllerOutput)>,
    camera_query: Query<&Transform, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const WALK: f32 = 5.0;
    const RUN: f32 = 8.0;
    const FRICTION: f32 = 0.85;
    const JUMP_FORCE: f32 = 8.0;
    const GRAVITY: f32 = -9.81;
    const FALL_MULTIPLIER: f32 = 2.25;
    const JUMP_MULTIPLIER: f32 = 0.95;

    let Ok(cam) = camera_query.get_single() else { return };
    if let Ok((mut controller, mut physics, output)) = query.get_single_mut() {
        // getting directions
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

        // getting target speed
        let speed = if input.pressed(KeyCode::ShiftLeft) {
            RUN
        } else {
            WALK
        };

        let mut desired_velocity = if direction != Vec3::ZERO {
            direction.normalize() * speed
        } else {
            Vec3::ZERO
        };

        desired_velocity.y = physics.velocity.y;

        if output.grounded && input.pressed(KeyCode::Space) {
            physics.velocity.y = JUMP_FORCE;
        }

        if !output.grounded {
            let gravity_scale = if physics.velocity.y > 0.0 {
                JUMP_MULTIPLIER
            } else {
                FALL_MULTIPLIER
            };

            physics.velocity.y += GRAVITY * gravity_scale * time.delta_secs();
        }

        // Smooth movement towards desired velocity
        physics.velocity = physics.velocity.lerp(desired_velocity, 1.0 - FRICTION);

        // Apply final movement through character controller
        controller.translation = Some(physics.velocity * time.delta_secs());

    }

}
