use bevy::prelude::*;

use crate::player::Player;

use super::{ArcherState, Enemy, EnemyArcher, MOVE_SPEED};

fn move_archers(
    time: Res<Time>,
    q_player: Query<&Transform, With<Player>>,
    mut q_enemies: Query<(&mut Transform, &Enemy, &EnemyArcher), Without<Player>>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    for (mut transform, enemy, archer) in &mut q_enemies {
        if enemy.disabled || enemy.stunned {
            continue;
        }
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
        app.add_systems(Update, (move_archers,));
    }
}
