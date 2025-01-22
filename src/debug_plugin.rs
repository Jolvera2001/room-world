use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_rapier3d::prelude::*;

use crate::{
    gui_plugin::{GameState, GameStateMarker},
    player_plugin::Player,
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_debug);
    }
}

fn player_debug(
    mut contexts: EguiContexts,
    rapier_context: Query<&KinematicCharacterControllerOutput, With<Player>>,
    game_query: Query<&GameState, With<GameStateMarker>>,
) {
    if let Ok(state) = game_query.get_single() {
        if let Ok(player_context) = rapier_context.get_single() {
            egui::Window::new("Debug")
                .anchor(egui::Align2::RIGHT_TOP, [0.0, 0.0])
                .resizable(false)
                .collapsible(false)
                .show(contexts.ctx_mut(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Game State");
                        ui.add_space(1.25);
                        ui.label(state.paused.to_string());
                        ui.label(state.in_dialog.to_string());
                        ui.label(if let Some(dialog) = &state.current_dialog {
                            format!("Current Dialog {}", dialog.lines.len())
                        } else {
                            "No Active Dialog".to_string()
                        });
                        ui.label(state.current_dialog_line.to_string());

                        ui.add_space(5.0);

                        ui.heading("Player context");
                        ui.add_space(1.25);
                        ui.label(player_context.grounded.to_string());
                    })
                });
        }
    }
}
