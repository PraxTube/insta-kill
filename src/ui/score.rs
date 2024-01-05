use bevy::prelude::*;

use crate::{player::score::PlayerScore, GameAssets, GameState};

const SIZE: f32 = 32.0;

#[derive(Component)]
struct Score;
#[derive(Component)]
struct ScoreText;

fn spawn_score(mut commands: Commands, assets: Res<GameAssets>) {
    let icon = commands
        .spawn(ImageBundle {
            style: Style {
                width: Val::Px(SIZE),
                height: Val::Px(2.0 * SIZE),
                margin: UiRect {
                    right: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            image: UiImage {
                texture: assets.score_icon.clone(),
                ..default()
            },
            ..default()
        })
        .id();

    let text = commands
        .spawn((
            ScoreText,
            TextBundle {
                text: Text::from_section(
                    "",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: SIZE,
                        color: Color::WHITE,
                    },
                ),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Score,
            NodeBundle {
                style: Style {
                    top: Val::Px(120.0),
                    left: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&[icon, text]);
}

fn despawn_score(mut commands: Commands, q_counters: Query<Entity, With<Score>>) {
    for entity in &q_counters {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_score_text(
    player_score: Res<PlayerScore>,
    mut q_counter_text: Query<&mut Text, With<ScoreText>>,
) {
    let mut text = match q_counter_text.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    text.sections[0].value = player_score.score().to_string();
}

pub struct ScoreUiPlugin;

impl Plugin for ScoreUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_score_text,).run_if(in_state(GameState::Gaming)),
        )
        .add_systems(OnEnter(GameState::Gaming), (spawn_score,))
        .add_systems(OnExit(GameState::Gaming), (despawn_score,));
    }
}
