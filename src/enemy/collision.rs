use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::player::{spawn::PlayerDashColliderContainer, strike::Strike};

use super::Enemy;

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

pub struct EnemyCollisionPlugin;

impl Plugin for EnemyCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_strike_collisions, player_dash_collisions));
    }
}
