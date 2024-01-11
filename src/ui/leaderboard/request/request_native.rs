use std::{
    io::{Read, Write},
    net::TcpStream,
};

use futures_lite::future;

use bevy::{
    prelude::*,
    tasks::{block_on, AsyncComputeTaskPool, Task},
    utils::futures::now_or_never,
};

use crate::player::{kill_counter::KillCounter, score::PlayerScore, speed_timer::SpeedTimer};

use super::{
    super::{super::text_field::SubmittedTextInput, DataFetched, HOST, PORT},
    post_request_string, GET_REQEUST,
};

#[derive(Component)]
struct FetchData(Task<Result<String, reqwest::Error>>);
#[derive(Component)]
struct PostData(Task<String>);

#[derive(Event)]
struct DataPosted(String);

fn send_post_request(
    mut commands: Commands,
    player_score: Res<PlayerScore>,
    kill_counter: Res<KillCounter>,
    speed_timer: Res<SpeedTimer>,
    mut ev_submitted_text_input: EventReader<SubmittedTextInput>,
) {
    let thread_pool = AsyncComputeTaskPool::get();

    for ev in ev_submitted_text_input.read() {
        let name = ev.0.clone();
        let score = player_score.score();
        let kills = kill_counter.kills();
        let time = speed_timer.elapsed;

        let task = thread_pool.spawn(async move {
            let data_to_send = format!("{},{},{},{}", name, score, kills, time);
            let request = post_request_string(data_to_send);

            if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", HOST, PORT)) {
                if let Err(e) = stream.write_all(request.as_bytes()) {
                    return format!("Failed to send request: {}", e);
                }

                let mut response = String::new();
                if let Err(e) = stream.read_to_string(&mut response) {
                    return format!("Failed to read response: {}", e);
                }

                println!("Response:\n{}", response);
                return response;
            }
            "Failed to connect to the server".to_string()
        });
        commands.spawn(PostData(task));
    }
}

fn send_get_request(mut commands: Commands, mut ev_data_posted: EventReader<DataPosted>) {
    if ev_data_posted.is_empty() {
        return;
    }
    ev_data_posted.clear();

    let thread_pool = AsyncComputeTaskPool::get();
    // Spawn new task on the AsyncComputeTaskPool; the task will be
    // executed in the background, and the Task future returned by
    // spawn() can be used to poll for the result
    let task = thread_pool.spawn(async move { reqwest::blocking::get(GET_REQEUST)?.text() });
    commands.spawn(FetchData(task));
}

fn handle_post_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut PostData)>,
    mut ev_data_posted: EventWriter<DataPosted>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(response) = block_on(future::poll_once(&mut task.0)) {
            ev_data_posted.send(DataPosted(response));
            commands.entity(entity).remove::<PostData>();
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn handle_get_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut FetchData)>,
    mut ev_data_fetched: EventWriter<DataFetched>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(reqwest_response) = block_on(future::poll_once(&mut task.0)) {
            match reqwest_response {
                Ok(response) => {
                    ev_data_fetched.send(DataFetched(response));
                    commands.entity(entity).remove::<FetchData>();
                    commands.entity(entity).despawn_recursive();
                }
                Err(err) => {
                    error!("GET reqwest failed, {}", err);
                }
            }
        }
    }
}

#[allow(dead_code)]
fn drop_all_tasks(
    mut commands: Commands,
    mut get_tasks: Query<(Entity, &mut FetchData), Without<PostData>>,
    mut post_tasks: Query<(Entity, &mut PostData), Without<FetchData>>,
) {
    for (entity, mut task) in &mut get_tasks {
        now_or_never(&mut task.0);
        commands.entity(entity).remove::<FetchData>();
        commands.entity(entity).despawn_recursive();
    }
    for (entity, mut task) in &mut post_tasks {
        now_or_never(&mut task.0);
        commands.entity(entity).remove::<PostData>();
        commands.entity(entity).despawn_recursive();
    }
}

pub struct LeaderboardRequestNativePlugin;

impl Plugin for LeaderboardRequestNativePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                send_post_request,
                send_get_request,
                handle_post_task,
                handle_get_task,
            ),
        )
        .add_event::<DataPosted>();
    }
}
