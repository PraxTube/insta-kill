use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    utils::{quat_from_vec2, FixedRotation},
    world::camera::YSort,
    GameAssets, GameState,
};

use super::{
    input::{MouseWorldCoords, PlayerInput},
    Player,
};

const OFFSET: Vec3 = Vec3::new(0.0, -10.0, 0.0);
const CHAIN_COOLDOWN: f32 = 0.5;
const STRIKE_COOLDOWN: f32 = 0.2;
const STRIKE_CHAIN_COUNT: usize = 3;

#[derive(Resource, Default)]
struct StrikeCooldown {
    absolute_cooldown: Timer,
    chain_cooldown: Timer,
    strike_index: usize,
}

#[derive(Component)]
pub struct Strike;
#[derive(Component)]
pub struct StrikeCollider {
    timer: Timer,
}

#[derive(Event)]
pub struct SpawnStrike {
    pub rot: Quat,
    strike_index: usize,
}

impl Default for StrikeCollider {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

fn spawn_strikes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<Entity, With<Player>>,
    mut ev_spawn_strike: EventReader<SpawnStrike>,
) {
    let player_entity = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for ev in ev_spawn_strike.read() {
        let mut animator = AnimationPlayer2D::default();
        animator.play(assets.player_strike_animations[0].clone());

        let flip_y = ev.strike_index % 2 == 1;
        let scale = if ev.strike_index == STRIKE_CHAIN_COUNT - 1 {
            Vec3::splat(2.0)
        } else {
            Vec3::splat(1.5)
        };

        let collider = commands
            .spawn((
                StrikeCollider::default(),
                Sensor,
                Collider::capsule(Vec2::new(0.0, 15.0), Vec2::new(0.0, -15.0), 18.0),
                CollisionGroups::default(),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    25.0, 0.0, 0.0,
                ))),
            ))
            .id();

        let strike = commands
            .spawn((
                FixedRotation {
                    offset: OFFSET,
                    rot: ev.rot,
                },
                Strike,
                YSort(1.0),
                animator,
                SpriteSheetBundle {
                    transform: Transform::from_scale(scale),
                    texture_atlas: assets.player_strike.clone(),
                    sprite: TextureAtlasSprite {
                        flip_y,
                        ..default()
                    },
                    ..default()
                },
            ))
            .push_children(&[collider])
            .id();

        commands.entity(player_entity).push_children(&[strike]);
    }
}

fn despawn_strikes(
    mut commands: Commands,
    q_strikes: Query<(Entity, &AnimationPlayer2D), With<Strike>>,
) {
    for (entity, animation_player) in &q_strikes {
        if animation_player.is_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn trigger_strike(
    player_input: Res<PlayerInput>,
    mouse_coords: Res<MouseWorldCoords>,
    mut strike_cooldown: ResMut<StrikeCooldown>,
    q_player: Query<&Transform, With<Player>>,
    mut ev_spawn_strike: EventWriter<SpawnStrike>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    if !strike_cooldown.absolute_cooldown.finished() {
        return;
    }

    let rot = quat_from_vec2(mouse_coords.0 - player_transform.translation.truncate());

    if !player_input.attack {
        return;
    }

    ev_spawn_strike.send(SpawnStrike {
        rot,
        strike_index: strike_cooldown.strike_index,
    });
    strike_cooldown.strike_index += 1;

    strike_cooldown
        .chain_cooldown
        .set_duration(Duration::from_secs_f32(CHAIN_COOLDOWN));
    strike_cooldown.chain_cooldown.reset();

    // Reached the last strike in the striking chain
    if strike_cooldown.strike_index == STRIKE_CHAIN_COUNT {
        strike_cooldown.strike_index = 0;

        strike_cooldown
            .absolute_cooldown
            .set_duration(Duration::from_secs_f32(STRIKE_COOLDOWN));
        strike_cooldown.absolute_cooldown.reset();
    }
}

fn reset_chain(mut strike_cooldown: ResMut<StrikeCooldown>) {
    if strike_cooldown.chain_cooldown.finished() {
        strike_cooldown.strike_index = 0;
    }
}

fn tick_strike_cooldown(time: Res<Time>, mut strike_cooldown: ResMut<StrikeCooldown>) {
    strike_cooldown.chain_cooldown.tick(time.delta());
    strike_cooldown.absolute_cooldown.tick(time.delta());
}

fn tick_strike_collider_timers(
    mut commands: Commands,
    time: Res<Time>,
    mut q_strike_colliders: Query<(Entity, &mut StrikeCollider)>,
) {
    for (entity, mut strike_collider) in &mut q_strike_colliders {
        strike_collider.timer.tick(time.delta());
        if strike_collider.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct PlayerStrikePlugin;

impl Plugin for PlayerStrikePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_strikes,
                despawn_strikes,
                trigger_strike,
                reset_chain,
                tick_strike_cooldown,
                tick_strike_collider_timers,
            )
                .run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<StrikeCooldown>()
        .add_event::<SpawnStrike>();
    }
}
