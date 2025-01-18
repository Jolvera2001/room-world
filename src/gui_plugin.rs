use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_egui::{egui, EguiContexts};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvent>()
        .add_systems(Startup, setup_gui_plugin)
        .add_systems(Update, (
            handle_pause, 
            handle_game_events,
            render_pause_menu
        ));
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

#[derive(Component)]
pub struct GameStateMarker;

fn setup_gui_plugin(mut commands: Commands) {
    commands.spawn((
        GameState { paused: false },
        GameStateMarker,
    ));
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
                GameEvent::TogglePause => {
                    game_state.paused = !game_state.paused;
                    println!("Game paused: {}", game_state.paused);
                }     
            }
        }
    }
}

fn render_pause_menu(
    mut contexts: EguiContexts,
    mut event_writer: EventWriter<GameEvent>,
    mut exit: EventWriter<AppExit>,
    query: Query<&GameState>,
) {
    if let Ok(state) = query.get_single() {
        if state.paused {
            egui::Window::new("Pause Menu")
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(contexts.ctx_mut(), |ui| {
                ui.label("Paused");
                if ui.button("Resume").clicked() {
                    event_writer.send(GameEvent::TogglePause);
                }
                if ui.button("Quit").clicked() {
                    exit.send(AppExit::Success);
                }
            });
        }
    }
}