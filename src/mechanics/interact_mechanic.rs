use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player_plugin::Player;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_interaction);
    }
}

// marker
#[derive(Component)]
pub struct Interactable;

#[derive(Event)]
pub enum InteractType {
    Dialog(Entity),
    Door(Entity),
    Item(Entity),
}

fn handle_interaction(
    query: Query<(Entity, &Transform), With<Player>>,
    interactable_query: Query<Entity, With<Interactable>>,
    rapier_context: Query<&RapierContext>, // REMEMBER ITS A COMPONENT, NOT A RESOURCE
    mut event_writer: EventWriter<InteractType>,
) {
    let (player_entity, transform) = query.single();
    const INT_RADIUS: f32 = 4.5;

    if let Ok(context) = rapier_context.get_single() {
        context.intersections_with_shape(
            transform.translation, 
            Quat::default(), 
            &Collider::ball(INT_RADIUS), 
            QueryFilter::default(), |
            entity| {
                // if the found entity isn't ourselves AND it is an entity marked as interactable
                if entity != player_entity && interactable_query.contains(entity) {

                    println!("Found nearby entity: {:?}", entity);
                }
                true
        });
    }
}