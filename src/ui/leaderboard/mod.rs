mod request;
mod visual;

use bevy::prelude::*;

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

pub struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DataFetched>().add_plugins((
            visual::LeaderboardVisualPlugin,
            request::LeaderboardRequestPlugin,
        ));
    }
}
