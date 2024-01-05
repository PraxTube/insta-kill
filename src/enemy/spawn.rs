use bevy::prelude::*;

use crate::{
    player::{combo::ComboMeter, kill_counter::KillCounter, Player},
    GameState,
};

use super::{Enemy, EnemyProjectile};

pub const SPAWN_OFFSET: f32 = 900.0;
const MAX_PLAYER_DISTANCE: f32 = 1200.0;

#[derive(Event)]
pub struct DespawnEnemy {
    pub pos: Vec2,
}

fn despawn_enemies(
    mut commands: Commands,
    mut combo_meter: ResMut<ComboMeter>,
    mut death_counter: ResMut<KillCounter>,
    q_enemies: Query<(Entity, &Transform, &Enemy)>,
    mut ev_despawn_enemy: EventWriter<DespawnEnemy>,
) {
    for (entity, transform, enemy) in &q_enemies {
        if enemy.disabled {
            combo_meter.increase(5.0);
            death_counter.increase();
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

fn despawn_all_enemies(mut commands: Commands, q_enemies: Query<Entity, With<Enemy>>) {
    for entity in &q_enemies {
        commands.entity(entity).despawn_recursive();
    }
}

fn despawn_all_projectiles(
    mut commands: Commands,
    q_projectiles: Query<Entity, With<EnemyProjectile>>,
) {
    for entity in &q_projectiles {
        commands.entity(entity).despawn_recursive();
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

fn redeploy_enemies(
    q_player: Query<&Transform, With<Player>>,
    mut q_enemies: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for mut enemy_transform in &mut q_enemies {
        if enemy_transform
            .translation
            .truncate()
            .distance_squared(player_transform.translation.truncate())
            >= MAX_PLAYER_DISTANCE.powi(2)
        {
            let dir = (player_transform.translation - enemy_transform.translation)
                .truncate()
                .normalize_or_zero();
            enemy_transform.translation =
                player_transform.translation + dir.extend(0.0) * SPAWN_OFFSET;
        }
    }
}

pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DespawnEnemy>()
            .add_systems(
                Update,
                (adjust_sprite_flip, redeploy_enemies).run_if(in_state(GameState::Gaming)),
            )
            .add_systems(Update, (despawn_enemies, despawn_projectiles))
            .add_systems(
                OnEnter(GameState::Restart),
                (despawn_all_enemies, despawn_all_projectiles),
            );
    }
}
