use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{utils::quat_from_vec2, world::camera::YSort, GameAssets, GameState};

const PROJECTILE_SPEED: f32 = 800.0;

#[derive(Component, Default)]
pub struct ReflectionProjectile {
    disabled: bool,
}

#[derive(Event)]
pub struct SpawnReflectionProjectile {
    pub pos: Vec2,
    pub dir: Vec2,
}

fn spawn_reflection_projectiles(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_reflection_projectile: EventReader<SpawnReflectionProjectile>,
) {
    for ev in ev_spawn_reflection_projectile.read() {
        let transform = Transform::from_translation(ev.pos.extend(0.0))
            .with_rotation(quat_from_vec2(ev.dir))
            .with_scale(Vec3::splat(2.0));

        let mut animator = AnimationPlayer2D::default();
        animator.play(assets.player_reflection_projectile_animations[0].clone());

        let collider = commands
            .spawn((
                Sensor,
                Collider::ball(10.0),
                CollisionGroups::default(),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    25.0, 0.0, 0.0,
                ))),
            ))
            .id();

        commands
            .spawn((
                ReflectionProjectile::default(),
                RigidBody::Dynamic,
                animator,
                YSort(1.0),
                SpriteSheetBundle {
                    transform,
                    texture_atlas: assets.player_reflection_projectile.clone(),
                    ..default()
                },
            ))
            .push_children(&[collider]);
    }
}

fn despawn_reflection_projectiles(
    mut commands: Commands,
    q_projectiles: Query<(Entity, &ReflectionProjectile)>,
) {
    for (entity, projectile) in &q_projectiles {
        if projectile.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn disable_reflection_projectiles(mut q_projectiles: Query<&mut ReflectionProjectile>) {
    for mut projectile in &mut q_projectiles {
        projectile.disabled = true;
    }
}

fn move_projectiles(
    time: Res<Time>,
    mut q_projectiles: Query<&mut Transform, With<ReflectionProjectile>>,
) {
    for mut transform in &mut q_projectiles {
        let dir = transform.local_x();
        transform.translation += dir * PROJECTILE_SPEED * time.delta_seconds();
    }
}

pub struct PlayerReflectionProjectilePlugin;

impl Plugin for PlayerReflectionProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_reflection_projectiles,).run_if(in_state(GameState::Gaming)),
        )
        .add_event::<SpawnReflectionProjectile>()
        .add_systems(
            OnEnter(GameState::Restart),
            (disable_reflection_projectiles,),
        )
        .add_systems(Update, (despawn_reflection_projectiles, move_projectiles));
    }
}
