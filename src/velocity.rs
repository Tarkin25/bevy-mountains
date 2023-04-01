use bevy::prelude::*;
use bevy_egui::EguiContext;
use bevy_inspector_egui::egui;

use crate::{camera_controller::CameraController, pause::GameState};

pub struct VelocityPlugin;

impl Plugin for VelocityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(Velocity::update)
            .add_system(Velocity::add_to_player)
            .add_system_set(
                SystemSet::on_update(GameState::Running).with_system(Velocity::display),
            );
    }
}

#[derive(Component)]
pub struct Velocity(f32);

#[derive(Component)]
struct PreviousTranslation(Vec3);

impl Velocity {
    fn add_to_player(
        mut commands: Commands,
        query: Query<(Entity, &Transform), (Added<CameraController>, Without<Velocity>)>,
    ) {
        for (entity, transform) in &query {
            commands
                .entity(entity)
                .insert((Velocity(0.0), PreviousTranslation(transform.translation)));
        }
    }

    fn update(
        mut query: Query<(&mut Velocity, &mut PreviousTranslation, &Transform)>,
        time: Res<Time>,
    ) {
        for (mut velocity, mut previous_translation, transform) in &mut query {
            let distance = transform.translation - previous_translation.0;
            velocity.0 = distance.length() / time.delta_seconds();
            previous_translation.0 = transform.translation;
        }
    }

    fn display(mut context: ResMut<EguiContext>, query: Query<&Velocity>) {
        let velocity = query.single();

        egui::Window::new("velocity")
            .title_bar(false)
            .show(context.ctx_mut(), |ui| {
                ui.label(format!("Velocity: {:.0} m/s", velocity.0));
            });
    }
}
