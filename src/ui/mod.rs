pub mod world_text;

mod game_over;
mod kill_counter;
mod leaderboard;
mod main_volume_bar;
mod score;
mod text_field;
mod vignette;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            world_text::WorldTextPlugin,
            kill_counter::KillCounterPlugin,
            score::ScoreUiPlugin,
            game_over::GameOverPlugin,
            leaderboard::LeaderboardPlugin,
            text_field::TextFieldPlugin,
            main_volume_bar::MainVolumeBarPlugin,
            vignette::VignettePlugin,
        ));
    }
}
