use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{enemy::Enemy, GameState};

use super::Player;

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

pub struct PlayerCollisionPlugin;

impl Plugin for PlayerCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (enemy_collisions,).run_if(in_state(GameState::Gaming)),
        );
    }
}
