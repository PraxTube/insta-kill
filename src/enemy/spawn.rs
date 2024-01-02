use bevy::prelude::*;

use crate::{player::Player, GameState};

use super::{Enemy, EnemyProjectile};

#[derive(Event)]
pub struct DespawnEnemy {
    pub pos: Vec2,
}

fn despawn_enemies(
    mut commands: Commands,
    q_enemies: Query<(Entity, &Transform, &Enemy)>,
    mut ev_despawn_enemy: EventWriter<DespawnEnemy>,
) {
    for (entity, transform, enemy) in &q_enemies {
        if enemy.disabled {
            ev_despawn_enemy.send(DespawnEnemy {
                pos: transform.translation.truncate(),
            });
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_projectiles(mut commands: Commands, q_projectiles: Query<(Entity, &EnemyProjectile)>) {
    for (entity, enemy_projectile) in &q_projectiles {
        if enemy_projectile.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn disable_enemies(mut q_enemies: Query<&mut Enemy>) {
    for mut enemy in &mut q_enemies {
        enemy.disabled = true;
    }
}

fn disabled_projectiles(mut q_projectiles: Query<&mut EnemyProjectile>) {
    for mut projectile in &mut q_projectiles {
        projectile.disabled = true;
    }
}

fn adjust_sprite_flip(
    q_player: Query<&Transform, With<Player>>,
    mut q_enemies: Query<(&Transform, &mut TextureAtlasSprite), With<Enemy>>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    for (transform, mut sprite) in &mut q_enemies {
        sprite.flip_x = player_pos.x < transform.translation.x;
    }
}

pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnEnemy>()
            .add_systems(
                Update,
                (adjust_sprite_flip,).run_if(in_state(GameState::Gaming)),
            )
            .add_systems(Update, (despawn_enemies, despawn_projectiles))
            .add_systems(
                OnEnter(GameState::Restart),
                (disable_enemies, disabled_projectiles),
            );
    }
}
