use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::player_plugin::Player;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<InteractType>()
            .add_systems(Update, (handle_interaction, interaction_events));
    }
}

// marker
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

fn handle_interaction(
    query: Query<(Entity, &Transform), With<Player>>,
    interactable_query: Query<Entity, With<Interactable>>,
    rapier_context: Query<&RapierContext>, // REMEMBER ITS A COMPONENT, NOT A RESOURCE
    dialog_query: Query<Entity, With<DialogTrigger>>,
    door_query: Query<Entity, With<DoorTrigger>>,
    item_query: Query<Entity, With<ItemTrigger>>,
    mut event_writer: EventWriter<InteractType>,
) {
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
                true
            },
        );
    }
}

fn interaction_events(mut event_reader: EventReader<InteractType>) {
    for event in event_reader.read() {
        match event {
            InteractType::Dialog(_) => println!("Press E to talk"),
            InteractType::Door(_) => println!("Press E to open/close door"),
            InteractType::Item(_) => println!("Press E to pick up item"),
        }
    }
}
