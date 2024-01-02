use bevy::prelude::*;

use crate::player::Player;

use super::{ArcherState, EnemyArcher, MOVE_SPEED};

fn trigger_moving(mut q_archers: Query<&mut EnemyArcher>) {
    for mut archer in &mut q_archers {
        if !archer.moving_cooldown.finished() {
            continue;
        }

        if archer.state == ArcherState::Idling {
            archer.state = ArcherState::Moving;
        }
    }
}

fn move_archers(
    time: Res<Time>,
    q_player: Query<&Transform, With<Player>>,
    mut q_enemies: Query<(&mut Transform, &EnemyArcher), Without<Player>>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    for (mut transform, archer) in &mut q_enemies {
        if archer.state != ArcherState::Moving {
            continue;
        }

        let dir = (player_pos - transform.translation)
            .truncate()
            .normalize_or_zero()
            .extend(0.0);
        transform.translation += dir * MOVE_SPEED * time.delta_seconds();
    }
}

pub struct EnemyArcherMovementPlugin;

impl Plugin for EnemyArcherMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (trigger_moving, move_archers));
    }
}
