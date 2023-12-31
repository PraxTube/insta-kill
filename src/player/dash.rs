use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    utils::{quat_from_vec2, COLLISION_GROUPS_NONE},
    GameState,
};

use super::{
    input::{MouseWorldCoords, PlayerInput},
    spawn::{PlayerCollider, PlayerDashCollider},
    state::{PlayerChangedState, PlayerState},
    Player, DASH_MULTIPLIER, MOVE_SPEED,
};

fn trigger_dash(player_input: Res<PlayerInput>, mut q_player: Query<&mut Player>) {
    let mut player = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state == PlayerState::Dashing {
        return;
    }

    if player_input.dash {
        player.state = PlayerState::Dashing;
    }
}

fn move_player(
    mouse_coords: Res<MouseWorldCoords>,
    mut q_player: Query<(&Transform, &mut Velocity), With<Player>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    let (transform, mut velocity) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_player_changed_state.read() {
        if ev.new_state != PlayerState::Dashing {
            continue;
        }

        let dir = (mouse_coords.0 - transform.translation.truncate()).normalize_or_zero();
        velocity.linvel = dir * DASH_MULTIPLIER * MOVE_SPEED;
    }
}

fn rotate_player(
    mouse_coords: Res<MouseWorldCoords>,
    mut q_player: Query<&mut Transform, With<Player>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    let mut transform = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_player_changed_state.read() {
        if ev.new_state != PlayerState::Dashing {
            continue;
        }

        transform.rotation = quat_from_vec2(mouse_coords.0 - transform.translation.truncate());
    }
}

fn toggle_player_dash_collider(
    mut q_player_dash_collider: Query<&mut CollisionGroups, With<PlayerDashCollider>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    let mut collision_groups = match q_player_dash_collider.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_player_changed_state.read() {
        if ev.new_state == PlayerState::Dashing {
            *collision_groups = CollisionGroups::default();
        } else if ev.old_state == PlayerState::Dashing {
            *collision_groups = COLLISION_GROUPS_NONE;
        }
    }
}

fn toggle_player_collider(
    mut q_player_collider: Query<&mut CollisionGroups, With<PlayerCollider>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    let mut collision_groups = match q_player_collider.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_player_changed_state.read() {
        if ev.new_state == PlayerState::Dashing {
            *collision_groups = COLLISION_GROUPS_NONE;
        } else if ev.old_state == PlayerState::Dashing {
            *collision_groups = CollisionGroups::default();
        }
    }
}

fn reset_dash_rotation(
    mut q_player: Query<&mut Transform, With<Player>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    let mut transform = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_player_changed_state.read() {
        if ev.old_state != PlayerState::Dashing {
            continue;
        }

        transform.rotation = Quat::IDENTITY;
    }
}

pub struct PlayerDashPlugin;

impl Plugin for PlayerDashPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                trigger_dash,
                move_player,
                rotate_player,
                toggle_player_dash_collider,
                toggle_player_collider,
                reset_dash_rotation,
            )
                .chain()
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
