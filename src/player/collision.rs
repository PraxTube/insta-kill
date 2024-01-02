use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    enemy::{Enemy, EnemyProjectile},
    GameState,
};

use super::{hook::PlayerHook, state::PlayerState, Player};

fn enemy_collisions(
    mut q_player: Query<&mut Player>,
    q_enemies: Query<&Enemy>,
    q_colliders: Query<&Parent, (With<Collider>, Without<Enemy>, Without<Player>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let mut player = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let enemy_parent = if &player.collider_entity == source {
            match q_colliders.get(*target) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else if &player.collider_entity == target {
            match q_colliders.get(*source) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else {
            continue;
        };

        let enemy = match q_enemies.get(enemy_parent.get()) {
            Ok(r) => r,
            Err(_) => continue,
        };

        if enemy.disabled {
            continue;
        }

        player.disabled = true;
    }
}

fn enemy_projectile_collisions(
    mut q_player: Query<&mut Player>,
    q_enemy_projectiles: Query<&EnemyProjectile>,
    q_colliders: Query<&Parent, (With<Collider>, Without<EnemyProjectile>, Without<Player>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let mut player = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let enemy_parent = if &player.collider_entity == source {
            match q_colliders.get(*target) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else if &player.collider_entity == target {
            match q_colliders.get(*source) {
                Ok(parent) => parent,
                Err(_) => continue,
            }
        } else {
            continue;
        };

        let projectile = match q_enemy_projectiles.get(enemy_parent.get()) {
            Ok(r) => r,
            Err(_) => continue,
        };

        if projectile.disabled {
            continue;
        }

        player.disabled = true;
    }
}

fn hook_enemy_collision(
    mut q_player: Query<&mut Player>,
    q_hooks: Query<&PlayerHook>,
    mut q_enemies: Query<(&Transform, &mut Enemy)>,
    q_colliders: Query<&Parent, (With<Collider>, Without<Enemy>, Without<Player>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
    let mut player = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };
    if player.state == PlayerState::Dashing {
        return;
    }
    // We already hooked an enemy, don't check for any further collisions.
    if player.hook_target_pos != Vec2::ZERO {
        return;
    }

    for ev in ev_collision_events.read() {
        let (source, target) = match ev {
            CollisionEvent::Started(source, target, _) => (source, target),
            CollisionEvent::Stopped(_, _, _) => continue,
        };

        let source_parent = match q_colliders.get(*source) {
            Ok(p) => p.get(),
            Err(_) => continue,
        };
        let target_parent = match q_colliders.get(*target) {
            Ok(p) => p.get(),
            Err(_) => continue,
        };

        let (enemy_transform, mut enemy) = if let Ok(r) = q_enemies.get_mut(source_parent) {
            r
        } else if let Ok(r) = q_enemies.get_mut(target_parent) {
            r
        } else {
            continue;
        };

        let _ = if let Ok(r) = q_hooks.get(source_parent) {
            r
        } else if let Ok(r) = q_hooks.get(target_parent) {
            r
        } else {
            continue;
        };

        player.state = PlayerState::Sliding;
        player.hook_target_pos = enemy_transform.translation.truncate();
        enemy.stunned = true;
    }
}

pub struct PlayerCollisionPlugin;

impl Plugin for PlayerCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                enemy_collisions,
                enemy_projectile_collisions,
                hook_enemy_collision,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
