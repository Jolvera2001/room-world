use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_egui::EguiPlugin;

mod player_plugin;
mod gui_plugin;
mod debug_plugin;
mod mechanics;

use gui_plugin::GuiPlugin;
use debug_plugin::DebugPlugin;
use player_plugin::{DialogTrigger, Interactable, PlayerPlugin};
use mechanics::dialog::{DialogData, DialogEntity, DialogPlugin};

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
        .add_plugins(DialogPlugin)
        .add_plugins(DebugPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ground
    commands
        .spawn(Collider::cuboid(25.0, 0.1, 25.0))
        .insert(Transform::from_xyz(0.0, -2.0, 0.0))
        .insert(RigidBody::Fixed);

    commands.spawn((
        Collider::ball(1.0),
        Transform::from_xyz(5.0, 0.5, 0.0),
        Interactable,
        DialogTrigger,
        DialogEntity,
        DialogData {
            dialog_file: asset_server.load("dialog_test.ron"),
        }
    ));
}