use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::{GameAssets, GameState};

use super::GameAudio;

const BGM_VOLUME: f64 = 0.5;

#[derive(Component)]
struct Bgm {
    handle: Handle<AudioInstance>,
}

#[derive(Component, Deref, DerefMut)]
struct UnmuteTimer(Timer);

impl Default for UnmuteTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Once))
    }
}

fn play_bgm(
    mut commands: Commands,
    assets: Res<GameAssets>,
    audio: Res<Audio>,
    game_audio: Res<GameAudio>,
) {
    let volume = game_audio.main_volume * BGM_VOLUME;
    let handle = audio
        .play(assets.bgm.clone())
        .with_volume(volume)
        .looped()
        .handle();
    commands.spawn(Bgm { handle });
}

fn mute_bgms(
    mut commands: Commands,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    q_bgms: Query<&Bgm>,
) {
    for bgm in &q_bgms {
        if let Some(instance) = audio_instances.get_mut(bgm.handle.clone()) {
            commands.spawn(UnmuteTimer::default());
            instance.set_volume(
                0.0,
                AudioTween::new(Duration::from_secs_f32(0.5), AudioEasing::Linear),
            );
        }
    }
}

fn unmute_bgms(
    mut commands: Commands,
    time: Res<Time>,
    game_audio: Res<GameAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut q_unmute_timers: Query<(Entity, &mut UnmuteTimer)>,
    q_bgms: Query<&Bgm>,
) {
    let mut unmute = false;
    for (entity, mut unmute_timer) in &mut q_unmute_timers {
        unmute_timer.tick(time.delta());
        if unmute_timer.just_finished() {
            unmute = true;
            commands.entity(entity).despawn_recursive();
        }
    }

    if !unmute {
        return;
    }

    for bgm in &q_bgms {
        if let Some(instance) = audio_instances.get_mut(bgm.handle.clone()) {
            instance.set_volume(
                game_audio.main_volume * BGM_VOLUME,
                AudioTween::new(Duration::from_secs_f32(5.0), AudioEasing::InPowi(2)),
            );
        }
    }
}

pub struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), play_bgm);
    }
}
