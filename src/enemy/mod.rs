mod bat;
mod hit_effect;
mod spawn;

use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hit_effect::EnemyHitEffectPlugin,
            spawn::EnemySpawnPlugin,
            bat::EnemyBatPlugin,
        ));
    }
}

#[derive(Component, Default)]
pub struct Enemy {
    pub stunned: bool,
    pub disabled: bool,
}
