use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_egui::EguiPlugin;

mod player_plugin;
mod gui_plugin;
mod mechanics;

use player_plugin::PlayerPlugin;
use gui_plugin::GuiPlugin;

fn main() {
    App::new()
        // outside plugins
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EguiPlugin)
        
        // personal plugins
        .add_plugins(GuiPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    // ground
    commands
        .spawn(Collider::cuboid(25.0, 0.1, 25.0))
        .insert(Transform::from_xyz(0.0, -2.0, 0.0))
        .insert(RigidBody::Fixed);
}