use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{GameAssets, GameState};

use super::{Player, HOOK_TIME};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum PlayerState {
    #[default]
    Idling,
    Moving,
    Dashing,
    Hooking,
    Sliding,
}

#[derive(Event)]
pub struct PlayerChangedState {
    pub old_state: PlayerState,
    pub new_state: PlayerState,
}

fn player_changed_state(
    q_player: Query<&Player>,
    mut ev_changed_state: EventWriter<PlayerChangedState>,
    mut old_state: Local<PlayerState>,
) {
    let player = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state != *old_state {
        ev_changed_state.send(PlayerChangedState {
            old_state: *old_state,
            new_state: player.state,
        });
        *old_state = player.state;
    }
}

fn update_animations(
    assets: Res<GameAssets>,
    mut q_player: Query<(&Player, &mut AnimationPlayer2D)>,
) {
    let (player, mut animator) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    let animation = match player.state {
        PlayerState::Idling => assets.player_animations[0].clone(),
        PlayerState::Moving => assets.player_animations[1].clone(),
        PlayerState::Dashing => assets.player_animations[2].clone(),
        PlayerState::Hooking => assets.player_animations[3].clone(),
        PlayerState::Sliding => assets.player_animations[4].clone(),
    };

    animator.play(animation).repeat();
}

fn adjust_sprite_flip(mut q_player: Query<(&mut TextureAtlasSprite, &Player)>) {
    let (mut sprite, player) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state == PlayerState::Hooking || player.state == PlayerState::Sliding {
        return;
    }
    if player.state == PlayerState::Dashing {
        sprite.flip_x = false;
        return;
    }

    if player.current_direction.x == 0.0 {
        return;
    }
    sprite.flip_x = player.current_direction.x < 0.0;
}

fn leave_dash(mut q_player: Query<(&mut Player, &AnimationPlayer2D)>) {
    let (mut player, animator) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state != PlayerState::Dashing {
        return;
    }

    if animator.is_finished() {
        player.state = PlayerState::Idling;
    }
}

fn stop_hooking(mut q_player: Query<(&mut Player, &AnimationPlayer2D)>) {
    let (mut player, animator) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state != PlayerState::Hooking {
        return;
    }

    if animator.elapsed() >= HOOK_TIME {
        player.state = PlayerState::Idling;
    }
}

pub struct PlayerStatePlugin;

impl Plugin for PlayerStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            (
                player_changed_state,
                update_animations,
                leave_dash.after(update_animations),
                stop_hooking.after(update_animations),
                adjust_sprite_flip,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<PlayerChangedState>();
    }
}
