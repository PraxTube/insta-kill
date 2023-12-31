use bevy::prelude::*;

use crate::player::Player;

use super::{Enemy, EnemyBat, MOVE_SPEED};

fn move_enemies(
    time: Res<Time>,
    q_player: Query<&Transform, With<Player>>,
    mut q_enemies: Query<(&mut Transform, &Enemy), (With<EnemyBat>, Without<Player>)>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    for (mut transform, enemy) in &mut q_enemies {
        if enemy.disabled || enemy.stunned {
            continue;
        }

        let dir = (player_pos - transform.translation)
            .truncate()
            .normalize_or_zero()
            .extend(0.0);
        transform.translation += dir * MOVE_SPEED * time.delta_seconds();
    }
}

pub struct EnemyBatMovementPlugin;

impl Plugin for EnemyBatMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_enemies,));
    }
}
