mod movement;
mod shooting;
mod spawn;

use std::time::Duration;

use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{GameAssets, GameState};

const SCORE: u32 = 200;
const MOVE_SPEED: f32 = 80.0;
const SHOOT_RANGE: f32 = 500.0;
/// How long should the archer idle after shooting before moving again
const MOVING_COOLDOWN: f32 = 2.0;

pub use super::Enemy;

pub struct EnemyArcherPlugin;

impl Plugin for EnemyArcherPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::EnemyArcherSpawnPlugin,
            movement::EnemyArcherMovementPlugin,
            shooting::EnemyArcherShootingPlugin,
        ))
        .add_systems(Update, (tick_cooldowns,))
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
    Stunned,
}

#[derive(Component)]
pub struct EnemyArcher {
    state: ArcherState,
    moving_cooldown: Timer,
}

impl Default for EnemyArcher {
    fn default() -> Self {
        let mut moving_cooldown = Timer::from_seconds(MOVING_COOLDOWN, TimerMode::Once);
        moving_cooldown.set_elapsed(Duration::from_secs_f32(MOVING_COOLDOWN));
        Self {
            state: ArcherState::default(),
            moving_cooldown,
        }
    }
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
            ArcherState::Stunned => assets.enemy_archer_animations[3].clone(),
        };

        animator.play(animation).repeat();
    }
}

fn tick_cooldowns(time: Res<Time>, mut q_archers: Query<&mut EnemyArcher>) {
    for mut archer in &mut q_archers {
        archer.moving_cooldown.tick(time.delta());
    }
}
