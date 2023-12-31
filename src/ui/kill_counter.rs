use bevy::prelude::*;

use crate::{player::kill_counter::KillCounter, GameAssets, GameState};

const SIZE: f32 = 32.0;

#[derive(Component)]
struct Counter;
#[derive(Component)]
struct CounterText;

fn spawn_counter(mut commands: Commands, assets: Res<GameAssets>) {
    let icon = commands
        .spawn(ImageBundle {
            style: Style {
                width: Val::Px(SIZE),
                height: Val::Px(SIZE),
                margin: UiRect {
                    right: Val::Px(10.0),
                    ..default()
                },
                ..default()
            },
            image: UiImage {
                texture: assets.death_counter_icon.clone(),
                ..default()
            },
            ..default()
        })
        .id();

    let text = commands
        .spawn((
            CounterText,
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
            Counter,
            NodeBundle {
                style: Style {
                    top: Val::Px(40.0),
                    left: Val::Px(40.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&[icon, text]);
}

fn despawn_counter(mut commands: Commands, q_counters: Query<Entity, With<Counter>>) {
    for entity in &q_counters {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_counter_text(
    death_counter: Res<KillCounter>,
    mut q_counter_text: Query<&mut Text, With<CounterText>>,
) {
    let mut text = match q_counter_text.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    text.sections[0].value = death_counter.kills().to_string();
}

pub struct KillCounterPlugin;

impl Plugin for KillCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_counter_text,).run_if(in_state(GameState::Gaming)),
        )
        .add_systems(OnEnter(GameState::Gaming), (spawn_counter,))
        .add_systems(OnExit(GameState::Gaming), (despawn_counter,));
    }
}
