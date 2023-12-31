mod collision;
mod hit_effect;
mod movement;
mod spawn;

use bevy::prelude::*;

const MOVE_SPEED: f32 = 100.0;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            spawn::EnemySpawnPlugin,
            collision::EnemyCollisionPlugin,
            movement::EnemyMovementPlugin,
            hit_effect::EnemyHitEffectPlugin,
        ));
    }
}

#[derive(Component, Default)]
pub struct Enemy {
    pub stunned: bool,
    pub disabled: bool,
}
