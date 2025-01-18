use bevy::prelude::*;
use bevy_egui;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvent>()
        .add_systems(Update, (handle_pause, handle_game_events));
    }
}

#[derive(Event)]
enum GameEvent {
    TogglePause
}

#[derive(Component)]
pub struct GameState {
    pub paused: bool
}

fn handle_pause(
    mut event_writer: EventWriter<GameEvent>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        event_writer.send(GameEvent::TogglePause);
    }
}

fn handle_game_events(
    mut event_reader: EventReader<GameEvent>,
    mut state_query: Query<&mut GameState>,
) {
    for event in event_reader.read() {
        if let Ok(mut game_state) = state_query.get_single_mut() {
            match event {
                GameEvent::TogglePause => game_state.paused = !game_state.paused
            }
        }
    }
}