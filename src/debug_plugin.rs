use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_rapier3d::prelude::*;

use crate::{
    gui_plugin::{GameState, GameStateMarker},
    player_plugin::{Player, PlayerPhysics},
};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, player_debug);
    }
}

fn player_debug(
    mut contexts: EguiContexts,
    rapier_context: Query<(&KinematicCharacterControllerOutput, &Transform, &PlayerPhysics), With<Player>>,
    game_query: Query<&GameState, With<GameStateMarker>>,
) {
    if let Ok(state) = game_query.get_single() {
        if let Ok((player_context, transform, physics)) = rapier_context.get_single() {
            egui::Window::new("Debug")
                .anchor(egui::Align2::RIGHT_TOP, [0.0, 0.0])
                .resizable(false)
                .collapsible(false)
                .show(contexts.ctx_mut(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Game State");
                        ui.add_space(1.25);
                        ui.label(format!("Paused: {}", state.paused.to_string()));
                        ui.label(format!("In Dialog state: {}", state.in_dialog.to_string()));
                        ui.label(if let Some(dialog) = &state.current_dialog {
                            format!("Current Dialog lines: {}", dialog.lines.len())
                        } else {
                            "No Active Dialog".to_string()
                        });
                        ui.label(format!("Current Line: {}", (state.current_dialog_line + 1).to_string()));

                        ui.add_space(5.0);

                        ui.heading("Player context");
                        ui.add_space(1.25);
                        ui.label(format!("Position Y: {:.6}", transform.translation.y));
                        ui.label(format!("Velocity: {:.6}", physics.velocity));
                        ui.label(format!("Grounded: {}", player_context.grounded.to_string()));
                        ui.label(format!("Sliding: {}", player_context.is_sliding_down_slope.to_string()));
                        ui.label(format!("Desired Translation: {:.6}", player_context.desired_translation));
                        ui.label(format!("Effective Translation: {:.6}", player_context.effective_translation));
                        if !player_context.collisions.is_empty() {
                            ui.heading("Collisions");
                            for collision in &player_context.collisions {
                                if let Some(details) = &collision.hit.details {
                                    ui.label(format!("Hit normal: {:.6}", details.normal1 ));
                                }
                                ui.label(format!("Time of impact: {:.6}", collision.hit.time_of_impact));
                                ui.label(format!("Translation applied: {:.6}", collision.translation_applied));
                                ui.label(format!("Translation remaining: {:.6}", collision.translation_remaining));
                            }
                        }
                    })
                });
        }
    }
}
