use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{world::camera::YSort, GameAssets, GameState};

use super::spawn::DespawnEnemy;

#[derive(Component)]
struct HitEffect;

fn spawn_hit_effects(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_despawn_enemy: EventReader<DespawnEnemy>,
) {
    for ev in ev_despawn_enemy.read() {
        let mut animator = AnimationPlayer2D::default();
        animator.play(assets.enemy_hit_animations[0].clone());

        commands.spawn((
            HitEffect,
            animator,
            YSort(-1.0),
            SpriteSheetBundle {
                texture_atlas: assets.enemy_hit.clone(),
                transform: Transform::from_translation(ev.pos.extend(0.0)),
                ..default()
            },
        ));
    }
}

fn despawn_hit_effects(
    mut commands: Commands,
    q_hit_effects: Query<(Entity, &AnimationPlayer2D), With<HitEffect>>,
) {
    for (entity, animator) in &q_hit_effects {
        if animator.is_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct EnemyHitEffectPlugin;

impl Plugin for EnemyHitEffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_hit_effects, despawn_hit_effects).run_if(in_state(GameState::Gaming)),
        );
    }
}
