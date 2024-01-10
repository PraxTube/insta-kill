#[cfg(not(target_arch = "wasm32"))]
mod request_desktop;
#[cfg(target_arch = "wasm32")]
mod request_wasm;

use bevy::prelude::*;

use crate::{ui::leaderboard::HOST, utils::format_time};

use super::{
    super::{game_over::GameOverState, text_field::SubmittedTextInput},
    DataFetched, LeaderboardData, LeaderboardEntry,
};

fn post_request_string(data_to_send: String) -> String {
    format!(
        "POST / HTTP/1.1\r\n\
         Host: {}\r\n\
         Content-Length: {}\r\n\
         Content-Type: text/plain\r\n\
         Connection: close\r\n\
         \r\n\
         {}\r\n",
        HOST,
        data_to_send.len(),
        data_to_send
    )
}

fn get_request_string() -> String {
    format!(
        "GET / HTTP/1.1\r\n\
             Host: {}\r\n\
             Connection: close\r\n\
             \r\n",
        HOST
    )
}

fn trigger_loading(
    mut next_state: ResMut<NextState<GameOverState>>,
    mut ev_submitted_text_input: EventReader<SubmittedTextInput>,
) {
    for _ in ev_submitted_text_input.read() {
        next_state.set(GameOverState::Loading);
    }
}

fn trigger_leaderboard(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameOverState>>,
    mut ev_data_fetched: EventReader<DataFetched>,
) {
    for ev in ev_data_fetched.read() {
        commands.insert_resource(string_to_leaderboard(&ev.0));
        next_state.set(GameOverState::Leaderboard);
    }
}

fn string_to_leaderboard(s: &str) -> LeaderboardData {
    if s.is_empty() {
        return LeaderboardData(Vec::new());
    }

    let rows: Vec<&str> = s.split(';').collect();
    let mut result = Vec::new();
    for row in rows {
        let values: Vec<&str> = row.split(',').collect();
        let entry = if values.len() != 4 {
            error!(
                "The leaderboard entry doesn't have exaclty 4 entries, {:?}",
                values
            );
            LeaderboardEntry::default()
        } else {
            LeaderboardEntry {
                name: values[0].to_string(),
                score: values[1].to_string(),
                kills: values[2].to_string(),
                time: format_time(values[3].parse().unwrap_or_default()),
            }
        };
        result.push(entry);
    }
    LeaderboardData(result)
}

pub struct LeaderboardRequestPlugin;

impl Plugin for LeaderboardRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            #[cfg(not(target_arch = "wasm32"))]
            request_desktop::LeaderboardRequestDesktopPlugin,
            #[cfg(target_arch = "wasm32")]
            request_wasm::LeaderboardRequestWASMPlugin,
        ))
        .add_systems(Update, (trigger_loading, trigger_leaderboard));
    }
}
