use bevy::prelude::*;

use crate::{
    player::score::PlayerScore, ui::text_field::SubmittedTextInput, GameAssets, GameState,
};

use super::{super::game_over::GameOverState, LeaderboardData, LEADERBOARD_COUNT};

const FONT_SIZE: f32 = 25.0;

#[derive(Component)]
struct Leaderboard;
#[derive(Resource)]
struct UserName(String);

fn spawn_text(commands: &mut Commands, font: Handle<Font>, text: &str, i: usize) -> Entity {
    let pos = [-560.0, -450.0, 0.0, 200.0, 400.0];
    let text_style = TextStyle {
        font,
        font_size: FONT_SIZE,
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
    let rank = spawn_text(commands, font.clone(), "", 0);
    let name = spawn_text(commands, font.clone(), "NAME", 1);
    let score = spawn_text(commands, font.clone(), "SCORE", 2);
    let kill = spawn_text(commands, font.clone(), "KILLS", 3);
    let time = spawn_text(commands, font.clone(), "TIME", 4);

    spawn_row(commands, &[rank, name, score, kill, time])
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

    let rank = spawn_text(commands, font.clone(), &format!("{:03}", i + 1), 0);
    let name = spawn_text(commands, font.clone(), &leaderboard_data[i].name, 1);
    let score = spawn_text(commands, font.clone(), &leaderboard_data[i].score, 2);
    let kill = spawn_text(commands, font.clone(), &leaderboard_data[i].kills, 3);
    let time = spawn_text(commands, font.clone(), &leaderboard_data[i].time, 4);

    spawn_row(commands, &[rank, name, score, kill, time])
}

fn spawn_restart_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text = "ESP OR R TO RESTART";
    let text_style = TextStyle {
        font,
        font_size: FONT_SIZE,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(text, text_style.clone())]);
    commands.spawn(text_bundle).id()
}

fn spawn_last_row(
    commands: &mut Commands,
    font: Handle<Font>,
    leaderboard_data: &Res<LeaderboardData>,
    username: &str,
    score: u32,
) -> Entity {
    let i = match leaderboard_data.find_index(username, &score.to_string()) {
        Some(r) => r,
        None => return commands.spawn(NodeBundle::default()).id(),
    };

    let rank = spawn_text(commands, font.clone(), &format!("{:03}", i + 1), 0);
    let name = spawn_text(commands, font.clone(), &leaderboard_data[i].name, 1);
    let score = spawn_text(commands, font.clone(), &leaderboard_data[i].score, 2);
    let kill = spawn_text(commands, font.clone(), &leaderboard_data[i].kills, 3);
    let time = spawn_text(commands, font.clone(), &leaderboard_data[i].time, 4);

    spawn_row(commands, &[rank, name, score, kill, time])
}

fn spawn_buffer(commands: &mut Commands, buffer: f32) -> Entity {
    let text = "";
    let text_style = TextStyle {
        font_size: FONT_SIZE,
        ..default()
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(text, text_style.clone())]);
    commands
        .spawn(text_bundle)
        .insert(Style {
            margin: UiRect {
                top: Val::Px(buffer / 2.0),
                bottom: Val::Px(buffer / 2.0),
                ..default()
            },
            ..default()
        })
        .id()
}

fn spawn_leaderboard(
    mut commands: Commands,
    assets: Res<GameAssets>,
    leaderboard_data: Res<LeaderboardData>,
    username: Res<UserName>,
    player_score: Res<PlayerScore>,
) {
    let restart_text = spawn_restart_text(&mut commands, assets.font.clone());
    let restart_buffer = spawn_buffer(&mut commands, 6.0);
    let header = spawn_header(&mut commands, assets.font.clone());

    let last_row_buffer = spawn_buffer(&mut commands, 6.0);
    let last_row = spawn_last_row(
        &mut commands,
        assets.font.clone(),
        &leaderboard_data,
        &username.0,
        player_score.score(),
    );

    let mut score_rows = Vec::new();
    for i in 0..LEADERBOARD_COUNT {
        score_rows.push(spawn_score_row(
            &mut commands,
            assets.font.clone(),
            &leaderboard_data,
            i,
        ));
    }

    let mut children = vec![restart_text, restart_buffer, header];
    children.extend(score_rows);
    children.push(last_row_buffer);
    children.push(last_row);

    commands
        .spawn((
            Leaderboard,
            NodeBundle {
                style: Style {
                    top: Val::Percent(10.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(7.0),
                    align_items: AlignItems::Center,
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

fn insert_username(
    mut commands: Commands,
    mut ev_submitted_text_input: EventReader<SubmittedTextInput>,
) {
    for ev in ev_submitted_text_input.read() {
        commands.insert_resource(UserName(ev.0.clone()));
    }
}

pub struct LeaderboardVisualPlugin;

impl Plugin for LeaderboardVisualPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameOverState::Leaderboard),
            (spawn_leaderboard,)
                .run_if(in_state(GameState::GameOver).and_then(resource_exists::<UserName>())),
        )
        .add_systems(OnExit(GameState::GameOver), (despawn_leaderboard,))
        .add_systems(Update, insert_username);
    }
}
