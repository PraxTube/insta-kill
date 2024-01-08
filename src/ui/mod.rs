pub mod world_text;

mod game_over;
mod kill_counter;
mod leaderboard;
mod score;

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
        ));
    }
}
