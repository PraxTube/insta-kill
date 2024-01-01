mod movement;
mod shooting;
mod spawn;

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{GameAssets, GameState};

const MOVE_SPEED: f32 = 80.0;
const SHOOT_RANGE: f32 = 500.0;

pub use super::Enemy;

pub struct EnemyArcherPlugin;

impl Plugin for EnemyArcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::EnemyArcherSpawnPlugin,
            movement::EnemyArcherMovementPlugin,
            shooting::EnemyArcherShootingPlugin,
        ))
        .add_systems(
            PostUpdate,
            (update_animations,).run_if(in_state(GameState::Gaming)),
        );
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
enum ArcherState {
    #[default]
    Idling,
    Moving,
    Shooting,
}

#[derive(Component, Default)]
pub struct EnemyArcher {
    state: ArcherState,
}

fn update_animations(
    assets: Res<GameAssets>,
    mut q_archers: Query<(&EnemyArcher, &mut AnimationPlayer2D)>,
) {
    for (archer, mut animator) in &mut q_archers {
        let animation = match archer.state {
            ArcherState::Idling => assets.enemy_archer_animations[0].clone(),
            ArcherState::Moving => assets.enemy_archer_animations[1].clone(),
            ArcherState::Shooting => assets.enemy_archer_animations[2].clone(),
        };

        animator.play(animation).repeat();
    }
}
