use std::time::Duration;

use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

use crate::in_game_time::InGameTime;

pub struct DaylightCyclePlugin;

impl Plugin for DaylightCyclePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AtmospherePlugin)
            .insert_resource(AtmosphereModel::new(Nishita {
                ..Default::default()
            }))
            .insert_resource(CycleTimer(Timer::new(
                Duration::from_millis(100),
                TimerMode::Repeating,
            )))
            .insert_resource(DaylightCycleSettings { speed: 5.0 })
            .add_system(Sun::cycle)
            .add_startup_system(Sun::spawn);
    }
}

#[derive(Component)]
struct Sun;

impl Sun {
    fn spawn(mut commands: Commands) {
        commands.spawn((
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    shadows_enabled: true,
                    shadow_projection: OrthographicProjection {
                        far: 6372.0e3,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            Sun,
        ));
    }

    fn cycle(
        mut atmosphere: AtmosphereMut<Nishita>,
        mut query: Query<(&mut Transform, &mut DirectionalLight), With<Sun>>,
        mut timer: ResMut<CycleTimer>,
        time: Res<InGameTime>,
        settings: Res<DaylightCycleSettings>,
    ) {
        timer.0.tick(time.delta());

        if settings.speed != 0.0 && timer.0.finished() {
            let delta = time.elapsed_seconds_wrapped() * settings.speed / 100.0;
            atmosphere.sun_position = Vec3::new(0.0, delta.sin(), delta.cos());

            if let Some((mut transform, mut light)) = query.get_single_mut().ok() {
                transform.rotation = Quat::from_rotation_x(-delta.sin().atan2(delta.cos()));
                light.illuminance = delta.sin().max(0.0).powf(2.0) * 100_000.0;
            }
        }
    }
}

#[derive(Resource)]
pub struct DaylightCycleSettings {
    pub speed: f32,
}

#[derive(Resource)]
pub struct CycleTimer(Timer);
