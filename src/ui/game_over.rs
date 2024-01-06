use bevy::prelude::*;

use crate::{
    player::{kill_counter::KillCounter, score::PlayerScore, speed_timer::SpeedTimer},
    GameAssets, GameState,
};

#[derive(Component)]
struct GameOverScreen;

fn spawn_background(commands: &mut Commands, texture: Handle<Image>) {
    commands.spawn((
        GameOverScreen,
        ImageBundle {
            style: Style {
                height: Val::Vh(100.0),
                width: Val::Vw(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            image: UiImage {
                texture,
                ..default()
            },
            background_color: BackgroundColor(Color::rgba(0.2, 0.2, 0.2, 0.85)),
            z_index: ZIndex::Local(100),
            ..default()
        },
    ));
}

fn spawn_title(commands: &mut Commands, font: Handle<Font>) -> Entity {
    let text_style = TextStyle {
        font,
        font_size: 100.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(
        "GAME OVER".to_string(),
        text_style.clone(),
    )]);
    commands.spawn((GameOverScreen, text_bundle)).id()
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
        .spawn((GameOverScreen, text_bundle))
        .insert(Style {
            margin: UiRect {
                bottom: Val::Px(20.0),
                ..default()
            },
            ..default()
        })
        .id()
}

fn spawn_time(commands: &mut Commands, font: Handle<Font>, time: f32) -> Entity {
    let text = format!("TIME: {:.2} seconds", time);
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(text, text_style.clone())]);
    commands.spawn((GameOverScreen, text_bundle)).id()
}

fn spawn_kill_counter(commands: &mut Commands, font: Handle<Font>, kills: u32) -> Entity {
    let text = format!("KILLS: {}", kills);
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(text, text_style.clone())]);
    commands.spawn((GameOverScreen, text_bundle)).id()
}

fn spawn_player_score(commands: &mut Commands, font: Handle<Font>, score: u32) -> Entity {
    let text = format!("SCORE: {}", score);
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(text, text_style.clone())]);
    commands.spawn((GameOverScreen, text_bundle)).id()
}

fn spawn_text(commands: &mut Commands, font: Handle<Font>, time: f32, kills: u32, score: u32) {
    let title_text = spawn_title(commands, font.clone());
    let restart_text = spawn_restart_text(commands, font.clone());
    let time_text = spawn_time(commands, font.clone(), time);
    let kill_text = spawn_kill_counter(commands, font.clone(), kills);
    let score_text = spawn_player_score(commands, font, score);

    commands
        .spawn((
            GameOverScreen,
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
        .push_children(&[title_text, restart_text, score_text, kill_text, time_text]);
}

fn spawn_game_over_screen(
    mut commands: Commands,
    assets: Res<GameAssets>,
    speed_timer: Res<SpeedTimer>,
    kill_counter: Res<KillCounter>,
    player_score: Res<PlayerScore>,
) {
    spawn_background(&mut commands, assets.white_pixel.clone());
    spawn_text(
        &mut commands,
        assets.font.clone(),
        speed_timer.elapsed,
        kill_counter.kills(),
        player_score.score(),
    );
}

fn despawn_game_over_screens(
    mut commands: Commands,
    q_game_over_screen: Query<Entity, With<GameOverScreen>>,
) {
    for entity in &q_game_over_screen {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), (spawn_game_over_screen,))
            .add_systems(OnExit(GameState::Restart), despawn_game_over_screens);
    }
}
