use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    effect::super_sonic::SpawnSuperSonic,
    utils::{quat_from_vec2, COLLISION_GROUPS_NONE},
    world::camera::YSort,
    GameAssets, GameState,
};

use super::{
    input::{MouseWorldCoords, PlayerInput},
    spawn::{PlayerCollider, PlayerDashCollider},
    state::{PlayerChangedState, PlayerState},
    Player, DASH_MULTIPLIER, MOVE_SPEED,
};

const DASH_LANDING_OFFSET: Vec3 = Vec3::new(0.0, -50.0, 0.0);

#[derive(Component)]
pub struct DashLanding;
#[derive(Component)]
pub struct DashLandingCollider;

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
    mut q_player: Query<(&mut Transform, &mut Velocity), With<Player>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
    mut ev_spawn_super_sonic: EventWriter<SpawnSuperSonic>,
) {
    let (mut transform, mut velocity) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_player_changed_state.read() {
        if ev.new_state != PlayerState::Dashing {
            continue;
        }

        let (dir, slide_multiplier) = if ev.old_state == PlayerState::Sliding {
            let dir = velocity.linvel.normalize_or_zero();
            ev_spawn_super_sonic.send(SpawnSuperSonic {
                pos: transform.translation.truncate(),
                dir,
                scale_factor: 3.0,
            });
            ev_spawn_super_sonic.send(SpawnSuperSonic {
                pos: transform.translation.truncate() + dir * 150.0,
                dir,
                scale_factor: 3.0,
            });
            ev_spawn_super_sonic.send(SpawnSuperSonic {
                pos: transform.translation.truncate() + dir * 300.0,
                dir,
                scale_factor: 3.0,
            });
            (dir, 2.0)
        } else {
            let dir = (mouse_coords.0 - transform.translation.truncate()).normalize_or_zero();
            (dir, 1.0)
        };

        transform.rotation = quat_from_vec2(dir);
        velocity.linvel = dir * DASH_MULTIPLIER * slide_multiplier * MOVE_SPEED;
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

fn spawn_dash_landings(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<&Transform, With<Player>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    for ev in ev_player_changed_state.read() {
        if ev.old_state != PlayerState::Dashing {
            continue;
        }

        let mut animator = AnimationPlayer2D::default();
        animator.play(assets.super_sonic_animations[0].clone());
        animator.set_speed(1.5);

        let transform = Transform::from_translation(player_pos + DASH_LANDING_OFFSET)
            .with_rotation(Quat::from_rotation_z(PI / 2.0))
            .with_scale(Vec3::splat(4.0));

        let collider = commands
            .spawn((
                DashLandingCollider,
                Sensor,
                Collider::ball(30.0),
                CollisionGroups::default(),
                TransformBundle::from_transform(Transform::from_translation(
                    transform
                        .rotation
                        .mul_vec3(DASH_LANDING_OFFSET / transform.scale.x),
                )),
            ))
            .id();

        commands
            .spawn((
                RigidBody::Dynamic,
                DashLanding,
                animator,
                YSort(1.0),
                SpriteSheetBundle {
                    transform,
                    texture_atlas: assets.super_sonic.clone(),
                    ..default()
                },
            ))
            .push_children(&[collider]);
    }
}

fn despawn_dash_landings(
    mut commands: Commands,
    q_dash_landings: Query<(Entity, &AnimationPlayer2D), With<DashLanding>>,
) {
    for (entity, animator) in &q_dash_landings {
        if animator.is_finished() {
            commands.entity(entity).despawn_recursive();
        }
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
                toggle_player_dash_collider,
                toggle_player_collider,
                reset_dash_rotation,
                spawn_dash_landings,
                despawn_dash_landings,
            )
                .chain()
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
