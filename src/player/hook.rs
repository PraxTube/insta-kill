use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{utils::quat_from_vec2, world::camera::YSort, GameAssets, GameState};

use super::{
    input::{MouseWorldCoords, PlayerInput},
    state::{PlayerChangedState, PlayerState},
    Player,
};

const ROT_OFFSET: Vec3 = Vec3::new(160.0, 0.0, 0.0);
const OFFSET: Vec3 = Vec3::new(0.0, -10.0, 0.0);
const COLLIDER_SPEED: f32 = 1000.0;

#[derive(Component, Default)]
pub struct PlayerHook;
#[derive(Component)]
pub struct PlayerHookCollider;

fn trigger_hook(player_input: Res<PlayerInput>, mut q_player: Query<&mut Player>) {
    let mut player = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.state == PlayerState::Hooking || player.state == PlayerState::Dashing {
        return;
    }

    if player_input.hook {
        player.state = PlayerState::Hooking;
    }
}

fn spawn_hooks(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mouse_coords: Res<MouseWorldCoords>,
    mut q_player: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<Player>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    let (player_entity, transform, mut sprite) = match q_player.get_single_mut() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_player_changed_state.read() {
        if ev.new_state != PlayerState::Hooking {
            continue;
        }

        let mut animator = AnimationPlayer2D::default();
        animator.play(assets.player_hook_animations[0].clone());

        let dir = (mouse_coords.0 - transform.translation.truncate()).normalize_or_zero();
        let rot = quat_from_vec2(dir);
        let transform =
            Transform::from_translation(rot.mul_vec3(ROT_OFFSET) + OFFSET).with_rotation(rot);

        sprite.flip_x = dir.x < 0.0;

        let collider = commands
            .spawn((
                PlayerHookCollider,
                Sensor,
                Collider::ball(15.0),
                CollisionGroups::default(),
                TransformBundle::from_transform(Transform::from_translation(-ROT_OFFSET)),
            ))
            .id();

        let hook = commands
            .spawn((
                PlayerHook,
                animator,
                YSort(-1.0),
                SpriteSheetBundle {
                    transform,
                    texture_atlas: assets.player_hook.clone(),
                    ..default()
                },
            ))
            .push_children(&[collider])
            .id();

        commands.entity(player_entity).push_children(&[hook]);
    }
}

fn despawn_hooks(
    mut commands: Commands,
    q_hooks: Query<Entity, With<PlayerHook>>,
    mut ev_player_changed_state: EventReader<PlayerChangedState>,
) {
    for ev in ev_player_changed_state.read() {
        if ev.old_state != PlayerState::Hooking {
            continue;
        }

        for entity in &q_hooks {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn reverse_animations(
    mut commands: Commands,
    mut q_hooks: Query<&mut AnimationPlayer2D, With<PlayerHook>>,
    q_hook_colliders: Query<Entity, With<PlayerHookCollider>>,
) {
    for mut animator in &mut q_hooks {
        if animator.is_finished() && !animator.is_reverse() {
            animator.reverse();
            for entity in &q_hook_colliders {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn move_hook_colliders(
    time: Res<Time>,
    q_hooks: Query<&AnimationPlayer2D>,
    mut q_hook_colliders: Query<(&Parent, &mut Transform), With<PlayerHookCollider>>,
) {
    for (parent, mut transform) in &mut q_hook_colliders {
        if let Ok(animator) = q_hooks.get(parent.get()) {
            let sign = if animator.is_reverse() { -1.0 } else { 1.0 };
            transform.translation += sign * Vec3::X * COLLIDER_SPEED * time.delta_seconds();
        }
    }
}

pub struct PlayerHookPlugin;

impl Plugin for PlayerHookPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                trigger_hook,
                spawn_hooks,
                despawn_hooks,
                reverse_animations,
                move_hook_colliders,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
