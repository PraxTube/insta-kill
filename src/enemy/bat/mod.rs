mod movement;
mod spawn;

use bevy::prelude::*;

const SCORE: u32 = 100;
const MOVE_SPEED: f32 = 120.0;

pub use super::Enemy;

pub struct EnemyBatPlugin;

impl Plugin for EnemyBatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((spawn::EnemyBatSpawnPlugin, movement::EnemyBatMovementPlugin));
    }
}

#[derive(Component)]
pub struct EnemyBat;
