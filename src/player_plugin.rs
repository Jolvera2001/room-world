use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::gui_plugin::{GameState, GameStateMarker};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractType>()
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (
                player_controls, 
                camera_control, 
                camera_follow,
                handle_interaction,
            ));
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::capsule(
            Vec3::new(0.0, 0.0, 0.0).into(), // Start at feet
            Vec3::new(0.0, 1.8, 0.0).into(), // End at head (1.8 units tall)
            0.4,
        ),
        KinematicCharacterController {
            max_slope_climb_angle: 0.7,
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(0.3),
                min_width: CharacterLength::Absolute(0.2),
                include_dynamic_bodies: false,
            }),  // Add this
            offset: CharacterLength::Absolute(0.01),
            ..default()
        },
        KinematicCharacterControllerOutput::default(),
        PlayerPhysics::default(),
        Transform::from_xyz(0.0, 1.0, 0.0),
        Player,
    ));
    commands
        .spawn((CameraOrbit::default(), Transform::default()))
        .with_children(|parent| {
            parent
                .spawn(Camera3d { ..default() })
                .insert(Transform::from_xyz(0.0, 2.0, 10.0));
        });
}

#[derive(Component)]
struct CameraOrbit {
    pitch: f32,
    yaw: f32,
}

impl Default for CameraOrbit {
    fn default() -> Self {
        Self {
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

// markers
#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Interactable;

// triggers
#[derive(Component)]
pub struct DialogTrigger;

#[derive(Component)]
pub struct DoorTrigger;

#[derive(Component)]
pub struct ItemTrigger;

#[derive(Event)]
pub enum InteractType {
    Dialog(Entity),
    Door(Entity),
    Item(Entity),
}

#[derive(Component)]
pub struct PlayerPhysics {
    pub velocity: Vec3,
}

impl Default for PlayerPhysics {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
        }
    }
}

fn camera_control(
    mut mouse_motion: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &mut CameraOrbit)>,
    game_query: Query<&GameState, With<GameStateMarker>>,
    time: Res<Time>,
) {
    if let Ok(game_state) = game_query.get_single() {
        if game_state.paused { return; }
    }

    const ROTATION_SPEED: f32 = 0.3;
    const MAX_PITCH: f32 = std::f32::consts::FRAC_PI_2 - 0.1;

    if let Ok((mut transform, mut orbit)) = query.get_single_mut() {
        let mut rotation = Vec2::ZERO;
        for event in mouse_motion.read() {
            rotation += event.delta * ROTATION_SPEED * time.delta_secs();
        }

        // (left/right)
        orbit.yaw -= rotation.x;

        // (up/down)
        orbit.pitch = (orbit.pitch - rotation.y).clamp(-MAX_PITCH, MAX_PITCH);

        transform.rotation =
            Quat::from_axis_angle(Vec3::Y, orbit.yaw) * Quat::from_axis_angle(Vec3::X, orbit.pitch);
    }
}

fn camera_follow(
    player_query: Query<&Transform, (With<PlayerPhysics>, With<Player>, Without<CameraOrbit>)>,
    mut camera_query: Query<&mut Transform, With<CameraOrbit>>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) =
        (player_query.get_single(), camera_query.get_single_mut())
    {
        camera_transform.translation = player_transform.translation;
    }
}

fn player_controls(
    mut query: Query<(
        &mut KinematicCharacterController,
        &mut PlayerPhysics,
        &KinematicCharacterControllerOutput,
    ), With<Player>>,
    camera_orbit_query: Query<&Transform, With<CameraOrbit>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    game_query: Query<&GameState, With<GameStateMarker>>,
) {
    if let Ok(game_state) = game_query.get_single() {
        if game_state.paused || game_state.in_dialog { return; }
    }

    const WALK: f32 = 5.0;
    const RUN: f32 = 8.0;
    const FRICTION: f32 = 0.875;
    const JUMP_FORCE: f32 = 8.0;
    const GRAVITY: f32 = -9.81;
    const FALL_MULTIPLIER: f32 = 2.25;
    const JUMP_MULTIPLIER: f32 = 0.95;

    let Ok(orbit_transform) = camera_orbit_query.get_single() else {
        return;
    };
    if let Ok((mut controller, mut physics, output)) = query.get_single_mut() {
        // getting directions
        let mut direction = Vec3::ZERO;

        let forward = orbit_transform.forward();
        let right = orbit_transform.right();

        let forward = Vec3::new(forward.x, 0.0, forward.z).normalize();
        let right = Vec3::new(right.x, 0.0, right.z).normalize();

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
            let mut gravity_scale = if physics.velocity.y > 0.0 {
                JUMP_MULTIPLIER
            } else {
                FALL_MULTIPLIER
            };

            if output.is_sliding_down_slope {
                gravity_scale *= 0.475;
            }

            physics.velocity.y += GRAVITY * gravity_scale * time.delta_secs();
        }

        // Smooth movement towards desired velocity
        physics.velocity = physics.velocity.lerp(desired_velocity, 1.0 - FRICTION);

        if physics.velocity.length() < 0.01 {
            physics.velocity = Vec3::ZERO;
        }

        // Apply final movement through character controller
        controller.translation = Some(physics.velocity * time.delta_secs());
    }
}

fn handle_interaction(
    query: Query<(Entity, &Transform), With<Player>>,
    check_query: Query<(&KinematicCharacterControllerOutput, &GameState), (With<Player>, With<GameStateMarker>)>,
    interactable_query: Query<Entity, With<Interactable>>,
    rapier_context: Query<&RapierContext>, // REMEMBER ITS A COMPONENT, NOT A RESOURCE
    dialog_query: Query<Entity, With<DialogTrigger>>,
    door_query: Query<Entity, With<DoorTrigger>>,
    item_query: Query<Entity, With<ItemTrigger>>,
    input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<InteractType>,
) {
    if let Ok((output, state)) = check_query.get_single() {
        if !output.grounded || state.paused || state.in_dialog { return; }
    }

    let (player_entity, transform) = query.single();
    const INT_RADIUS: f32 = 4.5;

    if let Ok(context) = rapier_context.get_single() {
        context.intersections_with_shape(
            transform.translation,
            Quat::default(),
            &Collider::ball(INT_RADIUS),
            QueryFilter::default(),
            |entity| {
                // if the found entity isn't ourselves AND it is an entity marked as interactable
                if entity != player_entity && interactable_query.contains(entity) {
                    if input.just_pressed(KeyCode::KeyE) {
                        if dialog_query.contains(entity) {
                            event_writer.send(InteractType::Dialog(entity));
                        }
                        if door_query.contains(entity) {
                            event_writer.send(InteractType::Door(entity));
                        }
                        if item_query.contains(entity) {
                            event_writer.send(InteractType::Item(entity));
                        }
                    }
                }
                true
            },
        );
    }
}