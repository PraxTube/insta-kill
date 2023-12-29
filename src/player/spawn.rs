use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::world::camera::YSort;
use crate::{GameAssets, GameState};

use super::{Player, PLAYER_SPAWN_POS};

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.player_animations[0].clone()).repeat();

    let entity = commands
        .spawn((
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
        .id();

    let collider = commands
        .spawn((
            Collider::ball(4.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, -5.0, 0.0))),
        ))
        .id();

    commands
        .entity(entity)
        .insert(Player::new(collider))
        .push_children(&[collider]);
}

pub struct PlayerSpawnPlugin;

impl Plugin for PlayerSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}
