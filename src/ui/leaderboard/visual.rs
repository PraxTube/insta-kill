use bevy::prelude::*;

use crate::{GameAssets, GameState};

use super::{super::game_over::GameOverState, LeaderboardData, LEADERBOARD_COUNT};

#[derive(Component)]
struct Leaderboard;

fn spawn_text(commands: &mut Commands, font: Handle<Font>, text: &str, i: usize) -> Entity {
    let pos = [-500.0, 0.0, 200.0, 400.0];
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

fn spawn_restart_text(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text = "PRESS 'R' TO RESTART";
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(text, text_style.clone())]);
    commands
        .spawn(text_bundle)
        .insert(Style {
            margin: UiRect {
                bottom: Val::Px(20.0),
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
) {
    let restart_text = spawn_restart_text(&mut commands, assets.font.clone());
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

    let mut children = vec![restart_text, header];
    children.extend(score_rows);

    commands
        .spawn((
            Leaderboard,
            NodeBundle {
                style: Style {
                    top: Val::Percent(15.0),
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

pub struct LeaderboardVisualPlugin;

impl Plugin for LeaderboardVisualPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameOverState::Leaderboard),
            (spawn_leaderboard,).run_if(in_state(GameState::GameOver)),
        )
        .add_systems(OnExit(GameState::GameOver), (despawn_leaderboard,));
    }
}
