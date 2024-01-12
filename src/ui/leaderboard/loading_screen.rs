use bevy::prelude::*;
use bevy_mod_reqwest::ReqwestInflight;

use crate::{player::input::PlayerInput, ui::game_over::GameOverState, GameAssets, GameState};

const LOADING_MESSAGE: &str = "UPLOADING SCORE ";
const LOADING_TICKER_TIME: f32 = 0.15;

#[derive(Component)]
struct Loading;
#[derive(Component)]
struct LoadingText;

fn spawn_loading_text(mut commands: Commands, assets: Res<GameAssets>) {
    let text_style = TextStyle {
        font: assets.font.clone(),
        font_size: 45.0,
        color: Color::WHITE,
    };

    let text_bundle =
        TextBundle::from_sections([TextSection::new(LOADING_MESSAGE, text_style.clone())]);
    let loading_text = commands.spawn((text_bundle, LoadingText)).id();

    let text_bundle = TextBundle::from_sections([TextSection::new(
        "ESC TO CANCEL AND RESTART",
        text_style.clone(),
    )]);
    let restart_text = commands.spawn(text_bundle).id();

    commands
        .spawn((
            Loading,
            NodeBundle {
                style: Style {
                    top: Val::Percent(35.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Vh(20.0),
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                z_index: ZIndex::Local(101),
                ..default()
            },
        ))
        .push_children(&[loading_text, restart_text]);
}

fn despawn_loading_text(mut commands: Commands, q_loadings: Query<Entity, With<Loading>>) {
    for entity in &q_loadings {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_requests(mut commands: Commands, q_requests: Query<Entity, With<ReqwestInflight>>) {
    for entity in &q_requests {
        commands.entity(entity).despawn_recursive();
    }
}

fn animate_loading_texts(
    mut q_texts: Query<&mut Text, With<LoadingText>>,
    mut ticks: Local<f32>,
    mut forward: Local<bool>,
    time: Res<Time>,
) {
    for mut text in &mut q_texts {
        *ticks += time.delta_seconds();
        if *ticks < LOADING_TICKER_TIME {
            continue;
        }
        *ticks = 0.0;

        let content = &text.sections[0].value;
        let new_text = if content.ends_with('/') {
            "-"
        } else if content.ends_with('\\') {
            "|"
        } else {
            *forward = !*forward;
            if *forward {
                "/"
            } else {
                "\\"
            }
        };

        text.sections[0].value = LOADING_MESSAGE.to_string() + new_text;
    }
}

fn restart(mut next_state: ResMut<NextState<GameState>>, player_input: Res<PlayerInput>) {
    if player_input.escape {
        next_state.set(GameState::Restart);
    }
}

pub struct LeaderboardLoadingScreenPlugin;

impl Plugin for LeaderboardLoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_loading_texts,).run_if(in_state(GameState::GameOver)),
        )
        .add_systems(
            Update,
            (restart,)
                .run_if(in_state(GameState::GameOver).and_then(in_state(GameOverState::Loading))),
        )
        .add_systems(OnEnter(GameOverState::Loading), spawn_loading_text)
        .add_systems(
            OnExit(GameOverState::Loading),
            (despawn_loading_text, despawn_requests),
        );
    }
}
