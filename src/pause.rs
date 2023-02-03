use bevy::{prelude::*, window::CursorGrabMode};
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::egui::{SidePanel, Window};

use crate::{chunk::ChunksConfig, learn_shaders::ColorGradient, noise_graph::NoiseGraph};
use crate::noise_graph::graph_manager::NoiseGraphManager;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    AssetsLoading,
    Paused,
    Running,
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Running).with_system(lock_cursor))
            .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(free_cursor))
            .add_system_set(SystemSet::on_update(GameState::Paused).with_system(draw_pause_menu))
            .add_system(toggle_game_state);
    }
}

fn draw_pause_menu(
    mut context: ResMut<EguiContext>,
    mut graph: ResMut<NoiseGraph>,
    mut color_gradient: ResMut<ColorGradient>,
    mut chunks_config: ResMut<ChunksConfig>,
    mut manager: ResMut<NoiseGraphManager>,
) {
    let ctx = context.ctx_mut();

    SidePanel::left("Side Panel").show(ctx, |ui| {
        ui.add(&mut *color_gradient);
        ui.separator();
        ui.add(&mut *chunks_config);
        ui.separator();
    });
    Window::new("noise graph")
        .title_bar(false)
        .fixed_rect(ctx.available_rect())
        .show(ctx, |ui| ui.add(&mut *manager));
}

fn lock_cursor(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_cursor_visibility(false);
    window.set_cursor_grab_mode(CursorGrabMode::Locked);
}

fn free_cursor(mut windows: ResMut<Windows>) {
    let window = windows.primary_mut();
    window.set_cursor_visibility(true);
    window.set_cursor_grab_mode(CursorGrabMode::None);
}

fn toggle_game_state(input: Res<Input<KeyCode>>, mut state: ResMut<State<GameState>>) {
    if input.just_pressed(KeyCode::P) {
        let new_state = match state.current() {
            GameState::Paused => GameState::Running,
            GameState::Running => GameState::Paused,
            _ => return,
        };

        state.set(new_state).unwrap();
    }
}
