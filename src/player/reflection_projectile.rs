use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{utils::quat_from_vec2, world::camera::YSort, GameAssets, GameState};

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
        let transform =
            Transform::from_translation(ev.pos.extend(0.0)).with_rotation(quat_from_vec2(ev.dir));

        let mut animator = AnimationPlayer2D::default();
        animator.play(assets.player_reflection_projectile_animations[0].clone());

        commands.spawn((
            ReflectionProjectile::default(),
            animator,
            YSort(1.0),
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.player_reflection_projectile.clone(),
                ..default()
            },
        ));
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
        .add_systems(Update, (despawn_reflection_projectiles,));
    }
}
