use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player_plugin::Player;

fn detect_nearby(
    context: Query<&RapierContext>,
    query: Query<(Entity, &Transform), With<Player>>,
) {

}