use bevy::prelude::*;

use crate::pause::GameState;

pub struct InGameTimePlugin;

impl Plugin for InGameTimePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(InGameTime::setup)
            .add_system(InGameTime::update)
            .add_system(InGameTime::toggle)
            .add_system_set(SystemSet::on_exit(GameState::Running).with_system(InGameTime::pause))
            .add_system_set(
                SystemSet::on_enter(GameState::Running).with_system(InGameTime::unpause),
            );
    }
}

#[derive(Resource, Deref)]
pub struct InGameTime(Time);

impl InGameTime {
    fn setup(mut commands: Commands, time: Res<Time>) {
        commands.insert_resource(InGameTime(time.clone()));
    }

    fn update(mut in_game_time: ResMut<InGameTime>) {
        in_game_time.0.update();
    }

    fn pause(mut in_game_time: ResMut<InGameTime>) {
        in_game_time.0.pause();
    }

    fn unpause(mut in_game_time: ResMut<InGameTime>) {
        in_game_time.0.unpause();
    }

    fn toggle(mut in_game_time: ResMut<InGameTime>, input: Res<Input<KeyCode>>) {
        if input.just_pressed(KeyCode::T) {
            let relative_speed = if in_game_time.relative_speed() == 0.0 {
                1.0
            } else {
                0.0
            };

            in_game_time.0.set_relative_speed(relative_speed);
        }
    }
}
