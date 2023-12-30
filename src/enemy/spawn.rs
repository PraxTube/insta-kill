use std::f32::consts::TAU;

use rand::{thread_rng, Rng};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::Player, world::camera::YSort, GameAssets, GameState};

use super::Enemy;

const OFFSET: f32 = 250.0;

#[derive(Resource)]
struct EnemySpawnCooldown {
    timer: Timer,
}

fn spawn_enemies(
    mut commands: Commands,
    assets: Res<GameAssets>,
    enemy_spawn_cooldown: Res<EnemySpawnCooldown>,
    q_player: Query<&Transform, With<Player>>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if !enemy_spawn_cooldown.timer.just_finished() {
        return;
    }

    let mut rng = thread_rng();

    let mut animator = AnimationPlayer2D::default();
    animator.play(assets.enemy_animations[0].clone()).repeat();

    let pos = player_transform.translation
        + Quat::from_rotation_z(rng.gen_range(0.0..TAU)).mul_vec3(Vec3::X) * OFFSET;

    let collider = commands
        .spawn((
            Collider::ball(8.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                0.0, -10.0, 0.0,
            ))),
        ))
        .id();

    let shadow = commands
        .spawn((
            YSort(-1.0),
            SpriteBundle {
                texture: assets.enemy_shadow.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -23.0, 0.0)),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Enemy::default(),
            YSort(0.0),
            animator,
            SpriteSheetBundle {
                texture_atlas: assets.enemy.clone(),
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(2.0)),
                ..default()
            },
        ))
        .push_children(&[shadow, collider]);
}

fn despawn_enemies(mut commands: Commands, q_enemies: Query<(Entity, &Enemy)>) {
    for (entity, enemy) in &q_enemies {
        if enemy.disabled {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn disable_enemies(mut q_enemies: Query<&mut Enemy>) {
    for mut enemy in &mut q_enemies {
        enemy.disabled = true;
    }
}

fn tick_enemy_spawn_cooldown(
    time: Res<Time>,
    mut enemy_spawn_cooldown: ResMut<EnemySpawnCooldown>,
) {
    enemy_spawn_cooldown.timer.tick(time.delta());
}

pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnCooldown {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (spawn_enemies, tick_enemy_spawn_cooldown).run_if(in_state(GameState::Gaming)),
        )
        .add_systems(Update, (despawn_enemies,))
        .add_systems(OnEnter(GameState::Restart), disable_enemies);
    }
}
