mod bgm;
mod sound;

pub use sound::PlaySound;

use bevy::prelude::*;
use bevy_kira_audio::prelude::AudioPlugin;

use crate::player::input::PlayerInput;

const MAIN_VOLUME_DELTA: f64 = 0.05;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AudioPlugin, bgm::BgmPlugin, sound::GameSoundPlugin))
            .add_systems(Update, (update_main_volume,));
    }
}

#[derive(Resource)]
pub struct GameAudio {
    pub main_volume: f64,
}

impl Default for GameAudio {
    fn default() -> Self {
        Self { main_volume: 0.5 }
    }
}

impl GameAudio {
    pub fn update(&mut self, x: f64) {
        self.main_volume = (self.main_volume + x).clamp(0.0, 1.0);
    }
}

fn update_main_volume(player_input: Res<PlayerInput>, mut game_audio: ResMut<GameAudio>) {
    if player_input.scroll == 0.0 {
        return;
    }

    game_audio.update(-player_input.scroll as f64 * MAIN_VOLUME_DELTA);
}
