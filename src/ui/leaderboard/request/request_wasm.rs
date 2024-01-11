use std::{
    io::{Read, Write},
    net::TcpStream,
};

use bevy::prelude::*;

use crate::player::{kill_counter::KillCounter, score::PlayerScore, speed_timer::SpeedTimer};

use super::{
    super::{super::text_field::SubmittedTextInput, DataFetched, HOST, PORT},
    post_request_string, GET_REQEUST,
};

#[derive(Event)]
struct DataPosted(String);

fn post_request(request: &str) -> String {
    if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", HOST, PORT)) {
        if let Err(e) = stream.write_all(request.as_bytes()) {
            return format!("Failed to send request: {}", e);
        }

        let mut response = String::new();
        if let Err(e) = stream.read_to_string(&mut response) {
            return format!("Failed to read response: {}", e);
        }

        println!("Response:\n{}", response);
        response
    } else {
        error!("Failed to connect to the server");
        "Failed to connect to the server".to_string()
    }
}

fn send_post_request(
    player_score: Res<PlayerScore>,
    kill_counter: Res<KillCounter>,
    speed_timer: Res<SpeedTimer>,
    mut ev_submitted_text_input: EventReader<SubmittedTextInput>,
    mut ev_data_posted: EventWriter<DataPosted>,
) {
    for ev in ev_submitted_text_input.read() {
        let name = ev.0.clone();
        let score = player_score.score();
        let kills = kill_counter.kills();
        let time = speed_timer.elapsed;

        let data_to_send = format!("{},{},{},{}", name, score, kills, time);
        let request = post_request_string(data_to_send);

        let response = post_request(&request);
        ev_data_posted.send(DataPosted(response));
    }
}

fn send_get_request(
    mut ev_data_posted: EventReader<DataPosted>,
    mut ev_data_fetched: EventWriter<DataFetched>,
) {
    if ev_data_posted.is_empty() {
        return;
    }
    ev_data_posted.clear();

    let response = match reqwest::blocking::get(GET_REQEUST) {
        Ok(r) => r.text().unwrap_or_default(),
        Err(err) => {
            error!("GET reqwest failed, {}", err);
            return;
        }
    };
    ev_data_fetched.send(DataFetched(response));
}

pub struct LeaderboardRequestWASMPlugin;

impl Plugin for LeaderboardRequestWASMPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DataPosted>()
            .add_systems(Update, (send_post_request, send_get_request));
    }
}
