use bevy::{prelude::*, window::CursorGrabMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Paused,
    Running,
}

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Running)
            .add_system_set(SystemSet::on_enter(GameState::Running).with_system(lock_cursor))
            .add_system_set(SystemSet::on_enter(GameState::Paused).with_system(free_cursor))
            .add_system(toggle_game_state);
    }
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
        };

        state.set(new_state).unwrap();
    }
}
