use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    player::{
        dash::DashLanding,
        reflection_projectile::{ReflectionProjectile, SpawnReflectionProjectile},
        spawn::PlayerDashColliderContainer,
        strike::Strike,
    },
    utils::FixedRotation,
};

use super::{Enemy, EnemyProjectile, DASH_SCORE_MULTIPLIYER, REFLECTION_PROJECTILE_SCORE_ADDITION};

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
    q_strikes: Query<(&FixedRotation, &Strike)>,
    mut q_enemy_projectiles: Query<(&Transform, &mut EnemyProjectile)>,
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

        let (strike_fixed_rotation, _) = if let Ok(r) = q_strikes.get(source_parent) {
            r
        } else if let Ok(r) = q_strikes.get(target_parent) {
            r
        } else {
            continue;
        };

        let dir = strike_fixed_rotation.rot.mul_vec3(Vec3::X).truncate();

        enemy_projectile.disabled = true;
        ev_spawn_reflection_projectile.send(SpawnReflectionProjectile {
            pos: projectile_transform.translation.truncate(),
            dir,
        })
    }
}

fn player_reflection_projectiles_collisions(
    mut q_projectiles: Query<&mut ReflectionProjectile>,
    mut q_enemies: Query<&mut Enemy>,
    q_colliders: Query<
        &Parent,
        (
            With<Collider>,
            Without<Enemy>,
            Without<ReflectionProjectile>,
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

        let mut projectile = if let Ok(r) = q_projectiles.get_mut(source_parent) {
            r
        } else if let Ok(r) = q_projectiles.get_mut(target_parent) {
            r
        } else {
            continue;
        };

        projectile.increase_counter();
        enemy.disabled = true;
        enemy.score += REFLECTION_PROJECTILE_SCORE_ADDITION * projectile.enemy_counter();
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
        enemy.score = (enemy.score as f32 * DASH_SCORE_MULTIPLIYER) as u32;
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
        enemy.score = (enemy.score as f32 * DASH_SCORE_MULTIPLIYER) as u32;
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
                player_reflection_projectiles_collisions,
                player_dash_collisions,
                player_dash_landing_collisions,
            ),
        );
    }
}
