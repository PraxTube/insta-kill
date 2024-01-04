use bevy::prelude::*;

use crate::{player::combo::ComboMeter, GameState};

#[derive(Component)]
struct ComboBarContainer;
#[derive(Component)]
struct ComboBar;

fn spawn_combo_bar(mut commands: Commands) {
    let combo_bar = commands
        .spawn((
            ComboBar,
            ImageBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            ComboBarContainer,
            NodeBundle {
                style: Style {
                    left: Val::Px(20.0),
                    top: Val::Percent(25.0),
                    height: Val::Percent(50.0),
                    width: Val::Percent(5.0),
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
        ))
        .push_children(&[combo_bar]);
}

fn despawn_combo_bar(mut commands: Commands, q_combo_bar: Query<Entity, With<ComboBarContainer>>) {
    for entity in &q_combo_bar {
        commands.entity(entity).despawn_recursive();
    }
}

fn update_combo_bar(
    combo_meter: Res<ComboMeter>,
    mut q_combo_bar: Query<&mut Style, With<ComboBar>>,
) {
    let mut style = match q_combo_bar.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    style.height = Val::Percent(combo_meter.progress());
}

pub struct ComboBarPlugin;

impl Plugin for ComboBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_combo_bar,).run_if(in_state(GameState::Gaming)),
        )
        .add_systems(OnEnter(GameState::Gaming), (spawn_combo_bar,))
        .add_systems(OnExit(GameState::Gaming), (despawn_combo_bar,));
    }
}
