use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::GameState;

use super::input::PlayerInput;
use super::{
    Player, PlayerState, HOOK_SLIDE_DISTANCE, MOVE_SPEED, PLAYER_HITBOX_OFFSET, SLIDE_SPEED,
};

fn player_movement(
    mut q_player: Query<(&mut Velocity, &mut Player)>,
    player_input: Res<PlayerInput>,
) {
    let (mut velocity, mut player) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state == PlayerState::Hooking {
        velocity.linvel = Vec2::ZERO;
        return;
    }
    if player.state != PlayerState::Moving && player.state != PlayerState::Idling {
        return;
    }

    let direction = player_input.move_direction;
    if direction == Vec2::default() {
        player.state = PlayerState::Idling;
        velocity.linvel = Vec2::ZERO;
        return;
    }

    player.state = PlayerState::Moving;
    player.current_direction = direction;
    velocity.linvel = direction * MOVE_SPEED;
}

fn slide_player(mut q_player: Query<(&Transform, &mut Velocity, &mut Player)>) {
    let (transform, mut velocity, mut player) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state != PlayerState::Sliding {
        return;
    }
    if (transform.translation + PLAYER_HITBOX_OFFSET)
        .truncate()
        .distance_squared(player.hook_target_pos)
        <= HOOK_SLIDE_DISTANCE.powi(2)
    {
        player.state = PlayerState::Idling;
        return;
    }

    let dir = (player.hook_target_pos - transform.translation.truncate()).normalize_or_zero();
    velocity.linvel = dir * SLIDE_SPEED;
}

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (player_movement, slide_player).run_if(in_state(GameState::Gaming)),
        );
    }
}
