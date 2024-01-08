use std::{
    io::{Read, Write},
    net::TcpStream,
};

use futures_lite::future;

use bevy::{
    prelude::*,
    tasks::{block_on, AsyncComputeTaskPool, Task},
};

use crate::{GameAssets, GameState};

use super::game_over::GameOverState;

const HOST: &str = "rancic.org";
const PORT: &str = "3434";

#[derive(Component)]
struct Leaderboard;

#[derive(Component)]
struct FetchData(Task<String>);

fn spawn_header(commands: &mut Commands, font: Handle<Font>) -> Entity {
    fn spawn_text(commands: &mut Commands, font: Handle<Font>, text: &str) -> Entity {
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
            .id()
    }

    let name = spawn_text(commands, font.clone(), "NAME");
    let score = spawn_text(commands, font.clone(), "SCORE");
    let kill = spawn_text(commands, font.clone(), "KILLS");
    let time = spawn_text(commands, font.clone(), "TIME");

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(50.0),
                margin: UiRect {
                    bottom: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .push_children(&[name, score, kill, time])
        .id()
}

fn spawn_leaderboard(mut commands: Commands, assets: Res<GameAssets>) {
    let header = spawn_header(&mut commands, assets.font.clone());

    commands
        .spawn((
            Leaderboard,
            NodeBundle {
                style: Style {
                    top: Val::Percent(25.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(3.0),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .push_children(&[header]);
}

fn fetch_leaderboard_data(mut commands: Commands) {
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
        let error_str = String::from("ERROR");

        if let Ok(mut stream) = TcpStream::connect(format!("{}:{}", HOST, PORT)) {
            if let Err(e) = stream.write_all(request.as_bytes()) {
                error!("Failed to send request: {}", e);
                return error_str;
            }

            let mut response = String::new();
            if let Err(e) = stream.read_to_string(&mut response) {
                error!("Failed to read response: {}", e);
                return error_str;
            }

            return response;
        } else {
            error!("Failed to connect to the server");
        }
        error_str
    });
    commands.spawn(FetchData(task));
}

fn handle_leaderboard_task(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut FetchData)>,
) {
    for (entity, mut task) in &mut transform_tasks {
        if let Some(response) = block_on(future::poll_once(&mut task.0)) {
            info!("{}", response);
            commands.entity(entity).remove::<FetchData>();
        }
    }
}

pub struct LeaderboardPlugin;

impl Plugin for LeaderboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameOverState::Loading),
            (fetch_leaderboard_data,).run_if(in_state(GameState::GameOver)),
        )
        .add_systems(
            OnEnter(GameOverState::Leaderboard),
            (spawn_leaderboard,).run_if(in_state(GameState::GameOver)),
        )
        .add_systems(Update, handle_leaderboard_task);
    }
}
