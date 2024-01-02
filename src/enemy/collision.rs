use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::{
    dash::DashLanding, reflection_projectile::SpawnReflectionProjectile,
    spawn::PlayerDashColliderContainer, strike::Strike,
};

use super::{Enemy, EnemyProjectile};

fn player_strike_collisions(
    q_strikes: Query<&Strike>,
    mut q_enemies: Query<&mut Enemy>,
    q_colliders: Query<&Parent, (With<Collider>, Without<Enemy>, Without<Strike>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
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

        let mut enemy = if let Ok(r) = q_enemies.get_mut(source_parent) {
            r
        } else if let Ok(r) = q_enemies.get_mut(target_parent) {
            r
        } else {
            continue;
        };

        let _ = if let Ok(r) = q_strikes.get(source_parent) {
            r
        } else if let Ok(r) = q_strikes.get(target_parent) {
            r
        } else {
            continue;
        };

        enemy.disabled = true;
    }
}

fn projectile_strike_collisions(
    q_strikes: Query<(&Transform, &Strike)>,
    mut q_enemy_projectiles: Query<(&Transform, &mut EnemyProjectile), Without<Strike>>,
    q_colliders: Query<&Parent, (With<Collider>, Without<EnemyProjectile>, Without<Strike>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
    mut ev_spawn_reflection_projectile: EventWriter<SpawnReflectionProjectile>,
) {
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

        let (projectile_transform, mut enemy_projectile) =
            if let Ok(r) = q_enemy_projectiles.get_mut(source_parent) {
                r
            } else if let Ok(r) = q_enemy_projectiles.get_mut(target_parent) {
                r
            } else {
                continue;
            };

        let (strike_transform, _) = if let Ok(r) = q_strikes.get(source_parent) {
            r
        } else if let Ok(r) = q_strikes.get(target_parent) {
            r
        } else {
            continue;
        };

        enemy_projectile.disabled = true;
        ev_spawn_reflection_projectile.send(SpawnReflectionProjectile {
            pos: projectile_transform.translation.truncate(),
            dir: strike_transform.local_x().truncate(),
        })
    }
}

fn player_dash_collisions(
    q_dash_collider_containers: Query<&PlayerDashColliderContainer>,
    mut q_enemies: Query<&mut Enemy>,
    q_colliders: Query<
        &Parent,
        (
            With<Collider>,
            Without<Enemy>,
            Without<PlayerDashColliderContainer>,
        ),
    >,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
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

        let mut enemy = if let Ok(r) = q_enemies.get_mut(source_parent) {
            r
        } else if let Ok(r) = q_enemies.get_mut(target_parent) {
            r
        } else {
            continue;
        };

        let _ = if let Ok(r) = q_dash_collider_containers.get(source_parent) {
            r
        } else if let Ok(r) = q_dash_collider_containers.get(target_parent) {
            r
        } else {
            continue;
        };

        enemy.disabled = true;
    }
}

fn player_dash_landing_collisions(
    q_dash_landings: Query<&DashLanding>,
    mut q_enemies: Query<&mut Enemy>,
    q_colliders: Query<&Parent, (With<Collider>, Without<Enemy>, Without<DashLanding>)>,
    mut ev_collision_events: EventReader<CollisionEvent>,
) {
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

        let mut enemy = if let Ok(r) = q_enemies.get_mut(source_parent) {
            r
        } else if let Ok(r) = q_enemies.get_mut(target_parent) {
            r
        } else {
            continue;
        };

        let _ = if let Ok(r) = q_dash_landings.get(source_parent) {
            r
        } else if let Ok(r) = q_dash_landings.get(target_parent) {
            r
        } else {
            continue;
        };

        enemy.disabled = true;
    }
}

pub struct EnemyCollisionPlugin;

impl Plugin for EnemyCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_strike_collisions,
                projectile_strike_collisions,
                player_dash_collisions,
                player_dash_landing_collisions,
            ),
        );
    }
}
