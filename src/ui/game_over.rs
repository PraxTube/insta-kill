use bevy::prelude::*;

use crate::{player::score::PlayerScore, GameAssets, GameState};

use super::text_field::spawn_text_field;

#[derive(Component)]
struct GameOverScreen;
#[derive(Component)]
struct GameOverBackground;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameOverState {
    #[default]
    GameOver,
    Loading,
    Leaderboard,
}

fn spawn_background(commands: &mut Commands, texture: Handle<Image>) {
    commands.spawn((
        GameOverBackground,
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
    commands.spawn(text_bundle).id()
}

fn spawn_player_score(commands: &mut Commands, font: Handle<Font>, score: u32) -> Entity {
    let text = format!("SCORE: {}", score);
    let text_style = TextStyle {
        font,
        font_size: 30.0,
        color: Color::WHITE,
    };
    let text_bundle = TextBundle::from_sections([TextSection::new(text, text_style.clone())]);
    commands.spawn(text_bundle).id()
}

fn spawn_text(commands: &mut Commands, font: Handle<Font>, score: u32) {
    let title_text = spawn_title(commands, font.clone());
    let score_text = spawn_player_score(commands, font.clone(), score);
    let input_field = spawn_text_field(commands, font.clone());

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
        .push_children(&[title_text, score_text, input_field]);
}

fn spawn_game_over_screen(
    mut commands: Commands,
    assets: Res<GameAssets>,
    player_score: Res<PlayerScore>,
) {
    spawn_background(&mut commands, assets.white_pixel.clone());
    spawn_text(&mut commands, assets.font.clone(), player_score.score());
}

fn despawn_game_over_screens(
    mut commands: Commands,
    q_game_over_screen: Query<Entity, With<GameOverScreen>>,
) {
    for entity in &q_game_over_screen {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_game_over_background(
    mut commands: Commands,
    q_backgrounds: Query<Entity, With<GameOverBackground>>,
) {
    for entity in &q_backgrounds {
        commands.entity(entity).despawn_recursive();
    }
}

fn reset_game_over_state(mut next_state: ResMut<NextState<GameOverState>>) {
    next_state.set(GameOverState::GameOver);
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameOverState>()
            .add_systems(
                OnEnter(GameState::GameOver),
                (spawn_game_over_screen, reset_game_over_state),
            )
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_screens)
            .add_systems(OnExit(GameOverState::GameOver), despawn_game_over_screens)
            .add_systems(OnExit(GameState::GameOver), despawn_game_over_background);
    }
}
