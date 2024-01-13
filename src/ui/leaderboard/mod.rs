mod loading_screen;
mod request;
mod visual;

use bevy::prelude::*;

use crate::{player::input::PlayerInput, GameState};

use super::game_over::GameOverState;

const LEADERBOARD_COUNT: usize = 7;

#[derive(Default)]
struct LeaderboardEntry {
    name: String,
    score: String,
    kills: String,
    time: String,
}

#[derive(Resource, Deref, DerefMut)]
struct LeaderboardData(Vec<LeaderboardEntry>);

#[derive(Event)]
struct DataFetched(String);

impl LeaderboardData {
    pub fn find_index(&self, name: &str, score: &str) -> Option<usize> {
        self.0
            .iter()
            .position(|entry| entry.name == name && entry.score == score)
    }
}

fn restart(mut next_state: ResMut<NextState<GameState>>, player_input: Res<PlayerInput>) {
    if player_input.restart || player_input.escape {
        next_state.set(GameState::Restart);
    }
}

pub struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DataFetched>()
            .add_plugins((
                visual::LeaderboardVisualPlugin,
                request::LeaderboardRequestPlugin,
                loading_screen::LeaderboardLoadingScreenPlugin,
            ))
            .add_systems(
                Update,
                (restart,).run_if(
                    in_state(GameState::GameOver).and_then(in_state(GameOverState::Leaderboard)),
                ),
            );
    }
}
