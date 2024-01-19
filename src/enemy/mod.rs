mod archer;
mod bat;
mod collision;
mod hit_effect;
mod spawn;

use bevy::prelude::*;

const REFLECTION_PROJECTILE_SCORE_ADDITION: u32 = 100;
const DASH_SCORE_MULTIPLIYER: f32 = 0.35;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hit_effect::EnemyHitEffectPlugin,
            spawn::EnemySpawnPlugin,
            bat::EnemyBatPlugin,
            archer::EnemyArcherPlugin,
            collision::EnemyCollisionPlugin,
        ));
    }
}

#[derive(Component, Default, Clone)]
pub struct Enemy {
    pub stunned: bool,
    pub disabled: bool,
    pub score: u32,
}

#[derive(Component, Default)]
pub struct EnemyProjectile {
    pub disabled: bool,
}
