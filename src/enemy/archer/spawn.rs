use std::f32::consts::TAU;

use rand::{thread_rng, Rng};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    enemy::spawn::SPAWN_OFFSET, player::Player, utils::quat_from_vec2, world::camera::YSort,
    GameAssets, GameState,
};

use super::{ArcherState, Enemy, EnemyArcher, SCORE};

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
    animator
        .play(assets.enemy_archer_animations[0].clone())
        .repeat();

    let pos = player_transform.translation
        + Quat::from_rotation_z(rng.gen_range(0.0..TAU)).mul_vec3(Vec3::X) * SPAWN_OFFSET;

    let collider = commands
        .spawn((
            Collider::capsule(Vec2::new(8.0, 0.0), Vec2::new(-8.0, 0.0), 7.0),
            ActiveEvents::COLLISION_EVENTS,
            CollisionGroups::default(),
            TransformBundle::from_transform(
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                    .with_rotation(quat_from_vec2(Vec2::Y)),
            ),
        ))
        .id();

    let shadow = commands
        .spawn((
            YSort(-1.0),
            SpriteBundle {
                texture: assets.enemy_archer_shadow.clone(),
                transform: Transform::from_translation(Vec3::new(0.0, -16.0, 0.0)),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Enemy {
                score: SCORE,
                ..default()
            },
            EnemyArcher::default(),
            YSort(0.0),
            animator,
            SpriteSheetBundle {
                texture_atlas: assets.enemy_archer.clone(),
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(2.0)),
                ..default()
            },
        ))
        .push_children(&[shadow, collider]);
}

fn tick_enemy_spawn_cooldown(
    time: Res<Time>,
    mut enemy_spawn_cooldown: ResMut<EnemySpawnCooldown>,
) {
    enemy_spawn_cooldown.timer.tick(time.delta());
}

fn trigger_stunned(mut q_archers: Query<(&Enemy, &mut EnemyArcher)>) {
    for (enemy, mut archer) in &mut q_archers {
        if enemy.disabled || enemy.stunned {
            archer.state = ArcherState::Stunned;
        }
    }
}

pub struct EnemyArcherSpawnPlugin;

impl Plugin for EnemyArcherSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnCooldown {
            timer: Timer::from_seconds(1.3, TimerMode::Repeating),
        })
        .add_systems(
            Update,
            (spawn_enemies, tick_enemy_spawn_cooldown, trigger_stunned)
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
