use std::f32::consts::TAU;

use rand::{thread_rng, Rng};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{player::Player, world::camera::YSort, GameAssets, GameState};

use super::Enemy;

const OFFSET: f32 = 350.0;

#[derive(Resource)]
struct EnemySpawnCooldown {
    timer: Timer,
}

#[derive(Event)]
pub struct DespawnEnemy {
    pub pos: Vec2,
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
            TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, -5.0, 0.0))),
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

fn despawn_enemies(
    mut commands: Commands,
    q_enemies: Query<(Entity, &Transform, &Enemy)>,
    mut ev_despawn_enemy: EventWriter<DespawnEnemy>,
) {
    for (entity, transform, enemy) in &q_enemies {
        if enemy.disabled {
            ev_despawn_enemy.send(DespawnEnemy {
                pos: transform.translation.truncate(),
            });
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

fn adjust_sprite_flip(
    q_player: Query<&Transform, With<Player>>,
    mut q_enemies: Query<(&Transform, &mut TextureAtlasSprite), With<Enemy>>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    for (transform, mut sprite) in &mut q_enemies {
        sprite.flip_x = player_pos.x > transform.translation.x;
    }
}

pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemySpawnCooldown {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        })
        .add_event::<DespawnEnemy>()
        .add_systems(
            Update,
            (spawn_enemies, tick_enemy_spawn_cooldown, adjust_sprite_flip)
                .run_if(in_state(GameState::Gaming)),
        )
        .add_systems(Update, (despawn_enemies,))
        .add_systems(OnEnter(GameState::Restart), disable_enemies);
    }
}
