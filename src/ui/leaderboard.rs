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

use crate::{
    player::{kill_counter::KillCounter, score::PlayerScore, speed_timer::SpeedTimer},
    utils::format_time,
    GameAssets, GameState,
};

use super::{game_over::GameOverState, text_field::SubmittedTextInput};

const HOST: &str = "rancic.org";
const PORT: &str = "3434";
const LEADERBOARD_COUNT: usize = 7;

struct LeaderboardEntry {
    name: String,
    score: String,
    kills: String,
    time: String,
}

#[derive(Resource, Deref, DerefMut)]
struct LeaderboardData(Vec<LeaderboardEntry>);

#[derive(Component)]
struct Leaderboard;
#[derive(Component)]
struct FetchData(Task<String>);
#[derive(Component)]
struct PostData(Task<String>);

#[derive(Event)]
struct DataFetched(String);

fn spawn_text(commands: &mut Commands, font: Handle<Font>, text: &str, i: usize) -> Entity {
    let pos = [150.0, 550.0, 750.0, 950.0];
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };

    commands
        .spawn(TextBundle::from_sections([TextSection::new(
            text,
            text_style.clone(),
        )]))
        .insert(Style {
            left: Val::Px(pos[i]),
            position_type: PositionType::Absolute,
            ..default()
        })
        .id()
}

fn spawn_row(commands: &mut Commands, children: &[Entity]) -> Entity {
    commands
        .spawn(NodeBundle {
            style: Style { ..default() },
            ..default()
        })
        .push_children(children)
        .id()
}

fn spawn_header(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let name = spawn_text(commands, font.clone(), "NAME", 0);
    let score = spawn_text(commands, font.clone(), "SCORE", 1);
    let kill = spawn_text(commands, font.clone(), "KILLS", 2);
    let time = spawn_text(commands, font.clone(), "TIME", 3);

    spawn_row(commands, &[name, score, kill, time])
}

fn spawn_score_row(
    commands: &mut Commands,
    font: Handle<Font>,
    leaderboard_data: &Res<LeaderboardData>,
    i: usize,
) -> Entity {
    if i >= leaderboard_data.len() {
        return commands.spawn(NodeBundle::default()).id();
    }

    let name = spawn_text(commands, font.clone(), &leaderboard_data[i].name, 0);
    let score = spawn_text(commands, font.clone(), &leaderboard_data[i].score, 1);
    let kill = spawn_text(commands, font.clone(), &leaderboard_data[i].kills, 2);
    let time = spawn_text(commands, font.clone(), &leaderboard_data[i].time, 3);

    spawn_row(commands, &[name, score, kill, time])
}

fn spawn_leaderboard(
    mut commands: Commands,
    assets: Res<GameAssets>,
    leaderboard_data: Res<LeaderboardData>,
) {
    let header = spawn_header(&mut commands, assets.font.clone());

    let mut score_rows = Vec::new();
    for i in 0..LEADERBOARD_COUNT {
        score_rows.push(spawn_score_row(
            &mut commands,
            assets.font.clone(),
            &leaderboard_data,
            i,
        ));
    }

    let mut children = vec![header];
    children.extend(score_rows);

    commands
        .spawn((
            Leaderboard,
            NodeBundle {
                style: Style {
                    top: Val::Percent(25.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(7.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .push_children(&children);
}

fn despawn_leaderboard(mut commands: Commands, q_leaderboards: Query<Entity, With<Leaderboard>>) {
    for entity in &q_leaderboards {
        commands.entity(entity).despawn_recursive();
    }
}

fn send_get_request(mut commands: Commands) {
    let thread_pool = AsyncComputeTaskPool::get();
    // Spawn new task on the AsyncComputeTaskPool; the task will be
    // executed in the background, and the Task future returned by
    // spawn() can be used to poll for the result
    let task = thread_pool.spawn(async move {
        let request = format!(
            "GET / HTTP/1.1\r\n\
             Host: {}\r\n\
             Connection: close\r\n\
             \r\n",
            HOST
        );

        if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", HOST, PORT)) {
            if let Err(e) = stream.write_all(request.as_bytes()) {
                return format!("Failed to send request: {}", e);
            }

            let mut response = String::new();
            if let Err(e) = stream.read_to_string(&mut response) {
                return format!("Failed to read response: {}", e);
            }

            return response;
        }
        "Failed to connect to the server".to_string()
    });
    commands.spawn(FetchData(task));
}

fn handle_get_task(
    mut commands: Commands,
    mut tasks: Query<(Entity, &mut FetchData)>,
    mut ev_data_fetched: EventWriter<DataFetched>,
) {
    for (entity, mut task) in &mut tasks {
        if let Some(response) = block_on(future::poll_once(&mut task.0)) {
            ev_data_fetched.send(DataFetched(response));
            commands.entity(entity).remove::<FetchData>();
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn handle_post_task(mut commands: Commands, mut tasks: Query<(Entity, &mut PostData)>) {
    for (entity, mut task) in &mut tasks {
        if let Some(_response) = block_on(future::poll_once(&mut task.0)) {
            commands.entity(entity).remove::<PostData>();
            commands.entity(entity).despawn_recursive();
        }
    }
}

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
        result.push(LeaderboardEntry {
            name: values[0].to_string(),
            score: values[1].to_string(),
            kills: values[2].to_string(),
            time: format_time(values[3].parse().unwrap_or_default()),
        });
    }
    LeaderboardData(result)
}

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

            let request = format!(
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
            );

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

// fn spawn_restart_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
//     let text = "PRESS 'R' TO RESTART";
//     let text_style = TextStyle {
//         font,
//         font_size: 30.0,
//         color: Color::WHITE,
//     };
//     let text_bundle = TextBundle::from_sections([TextSection::new(text, text_style.clone())]);
//     commands
//         .spawn((GameOverScreen, text_bundle))
//         .insert(Style {
//             margin: UiRect {
//                 bottom: Val::Px(20.0),
//                 ..default()
//             },
//             ..default()
//         })
//         .id()
// }

pub struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DataFetched>()
            .add_systems(OnEnter(GameOverState::Leaderboard), (spawn_leaderboard,))
            .add_systems(OnExit(GameState::GameOver), (despawn_leaderboard,))
            .add_systems(
                OnEnter(GameOverState::Loading),
                (send_get_request,).run_if(in_state(GameState::GameOver)),
            )
            .add_systems(
                Update,
                (
                    handle_get_task,
                    handle_post_task,
                    trigger_loading,
                    trigger_leaderboard,
                    send_post_request,
                ),
            );
    }
}
