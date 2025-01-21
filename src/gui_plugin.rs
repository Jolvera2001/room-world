use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::app::AppExit;
use bevy_egui::{egui::{self, Color32, RichText}, EguiContexts};

use crate::mechanics::dialog::{DialogAsset, DialogEvent};

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvent>()
        .add_systems(Startup, setup_gui_plugin)
        .add_systems(Update, (
            handle_pause, 
            handle_game_events,
            render_pause_menu,
            render_dialog_box,
        ));
    }
}

#[derive(Event)]
enum GameEvent {
    TogglePause,
}

#[derive(Component)]
pub struct GameState {
    pub paused: bool,
    pub in_dialog: bool,
    pub current_dialog_line: usize,
    pub current_dialog: Option<DialogAsset>
}

#[derive(Component)]
pub struct GameStateMarker;

fn setup_gui_plugin(mut commands: Commands) {
    commands.spawn((
        GameState { 
            paused: false,
            in_dialog: false,
            current_dialog_line: 0,
            current_dialog: None,
        },
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
                },
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

fn render_dialog_box(
    mut contexts: EguiContexts,
    mut dialog_event: EventReader<DialogEvent>,
    mut game_state: Query<&mut GameState>,
    input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if let Ok(mut state) = game_state.get_single_mut() {
        for event in dialog_event.read() {
            if let DialogEvent::DialogData(dialog_asset) = event {
                state.in_dialog = true;
                state.current_dialog_line = 0;
                state.current_dialog = Some(dialog_asset.clone());
            }
        }

        if state.in_dialog {
            if let Some(dialog) = &state.current_dialog {
                egui::Window::new("Dialog")
                    .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -30.0])
                    .resizable(false)
                    .collapsible(false)
                    .fixed_size([600.0, 150.0])
                    .show(contexts.ctx_mut(), |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label("Character Name");
                            ui.add_space(10.0);
                            ui.label(RichText::new(&dialog.lines[state.current_dialog_line])
                                .size(16.0)
                                .family(egui::FontFamily::Proportional)
                            );
        
                            ui.add_space(10.0);
                            ui.label(RichText::new("[Space or Mouse1] Continue")
                                    .size(12.0)
                                    .color(Color32::LIGHT_GRAY));
                        });
                    });
    
                if input.just_pressed(KeyCode::Space) || mouse_input.just_pressed(MouseButton::Left) {
                    if state.current_dialog_line < dialog.lines.len() - 1 {
                        state.current_dialog_line += 1;
                    } else {
                        state.in_dialog = false;
                        state.current_dialog = None;
                        state.current_dialog_line = 0;
                    }
                }
            }
        }
    }
}