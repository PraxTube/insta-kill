use bevy::prelude::*;
use bevy_mod_reqwest::*;

use crate::player::{kill_counter::KillCounter, score::PlayerScore, speed_timer::SpeedTimer};
use crate::utils::format_time;

use super::{
    super::{game_over::GameOverState, text_field::SubmittedTextInput},
    DataFetched, LeaderboardData, LeaderboardEntry,
};

const GET_URL: &str = "https://rancic.org/games/insta-kill/leaderboard.csv";
const POST_URL: &str = "https://rancic.org:3434/leaderboard";

#[derive(Component)]
struct PostRequest;
#[derive(Component)]
struct GetRequest;

#[derive(Event)]
pub struct DataPosted;

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

    let rows: Vec<&str> = s.trim_end_matches('\n').split('\n').collect();
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

fn send_post_request(
    mut commands: Commands,
    reqwest: Res<ReqwestClient>,
    player_score: Res<PlayerScore>,
    kill_counter: Res<KillCounter>,
    speed_timer: Res<SpeedTimer>,
    mut ev_submitted_text_input: EventReader<SubmittedTextInput>,
) {
    for ev in ev_submitted_text_input.read() {
        let url = format!(
            "{}/{}/{}/{}/{}",
            POST_URL,
            ev.0,
            player_score.score(),
            kill_counter.kills(),
            speed_timer.elapsed
        );

        let req = reqwest.0.post(url).build().unwrap();
        let req = ReqwestRequest::new(req);
        commands.spawn((req, PostRequest));
    }
}

fn send_get_request(
    mut commands: Commands,
    reqwest: Res<ReqwestClient>,
    mut ev_data_posted: EventReader<DataPosted>,
) {
    if ev_data_posted.is_empty() {
        return;
    }
    ev_data_posted.clear();

    let req = reqwest.0.post(GET_URL).build().unwrap();
    let req = ReqwestRequest::new(req);
    commands.spawn((req, GetRequest));
}

fn handle_post_responses(
    mut commands: Commands,
    results: Query<(Entity, &ReqwestBytesResult), With<PostRequest>>,
    mut ev_data_posted: EventWriter<DataPosted>,
) {
    for (entity, res) in &results {
        if let Err(err) = &res.0 {
            error!("{}", err);
        }

        commands.entity(entity).despawn_recursive();
        ev_data_posted.send(DataPosted);
    }
}

fn handle_get_responses(
    mut commands: Commands,
    results: Query<(Entity, &ReqwestBytesResult), With<GetRequest>>,
    mut ev_data_fetched: EventWriter<DataFetched>,
) {
    for (entity, res) in &results {
        let data = match &res.0 {
            Ok(bytes) => String::from_utf8(bytes.to_vec()).unwrap_or_default(),
            Err(err) => {
                error!("{}", err);
                String::new()
            }
        };

        commands.entity(entity).despawn_recursive();
        ev_data_fetched.send(DataFetched(data));
    }
}

pub struct LeaderboardRequestPlugin;

impl Plugin for LeaderboardRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                send_post_request,
                send_get_request,
                handle_post_responses,
                handle_get_responses,
            ),
        )
        .add_event::<DataPosted>()
        .add_systems(Update, (trigger_loading, trigger_leaderboard));
    }
}
