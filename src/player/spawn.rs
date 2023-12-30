use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::utils::{FixedRotation, COLLISION_GROUPS_NONE};
use crate::world::camera::YSort;
use crate::{GameAssets, GameState};

use super::{Player, PLAYER_SPAWN_POS};

const SHADOW_OFFSET: Vec3 = Vec3::new(0.0, -23.0, 0.0);

#[derive(Component)]
pub struct PlayerCollider;
#[derive(Component)]
pub struct PlayerDashCollider;
#[derive(Component)]
pub struct PlayerDashColliderContainer;

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.player_animations[0].clone()).repeat();

    let collider = commands
        .spawn((
            PlayerCollider,
            Collider::ball(5.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -10.0, 0.0,
            ))),
        ))
        .id();

    let dash_collider = commands
        .spawn((
            PlayerDashCollider,
            Sensor,
            Collider::ball(25.0),
            COLLISION_GROUPS_NONE,
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        ))
        .id();

    let dash_collider_container = commands
        .spawn((PlayerDashColliderContainer, TransformBundle::default()))
        .push_children(&[dash_collider])
        .id();

    let shadow = commands
        .spawn((
            FixedRotation {
                offset: SHADOW_OFFSET,
                ..default()
            },
            YSort(-1.0),
            SpriteBundle {
                texture: assets.player_shadow.clone(),
                transform: Transform::from_translation(SHADOW_OFFSET),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Player::new(collider),
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Velocity::zero(),
            Ccd::enabled(),
            YSort(0.0),
            animator,
            SpriteSheetBundle {
                transform: Transform::from_translation(PLAYER_SPAWN_POS)
                    .with_scale(Vec3::splat(2.0)),
                texture_atlas: assets.player.clone(),
                ..default()
            },
        ))
        .push_children(&[shadow, collider, dash_collider_container]);
}

fn despawn_player(mut commands: Commands, q_player: Query<Entity, With<Player>>) {
    let entity = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    commands.entity(entity).despawn_recursive();
}

fn trigger_game_over(mut next_state: ResMut<NextState<GameState>>, q_player: Query<&Player>) {
    let player = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if player.disabled {
        next_state.set(GameState::GameOver);
    }
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player)
            .add_systems(
                Update,
                (trigger_game_over,).run_if(in_state(GameState::Gaming)),
            )
            .add_systems(OnEnter(GameState::GameOver), despawn_player);
    }
}
