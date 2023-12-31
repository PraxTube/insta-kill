use rand::Rng;
use rand_xoshiro::rand_core::SeedableRng;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use super::{seed::Seed, GameRng, BACKGROUND_ZINDEX_ABS, CHUNK_SIZE};
use crate::{player::Player, GameAssets, GameState};

const CAMERA_SIZE_X: f32 = 800.0;
const CAMERA_SIZE_Y: f32 = 550.0;
const IIDS: [&str; 6] = [
    "4561cae1-8990-11ee-bdb7-27b92e7f0bd1",
    "4c5c13d0-8990-11ee-bb97-5335be5f091d",
    "30c12d00-8990-11ee-8c0e-1f466f38a0b0",
    "09bdb020-8990-11ee-8c0e-83df39a96f91",
    "39c4ea40-8990-11ee-8c0e-f5477a2dc37e",
    "54eaef30-8990-11ee-bb97-69638b6a5187",
];

#[derive(Component)]
pub struct Chunk {
    x: i32,
    y: i32,
}

fn map_indices_to_world_coords(x_index: i32, y_index: i32) -> Vec3 {
    Vec3::new(
        x_index as f32 * CHUNK_SIZE,
        y_index as f32 * CHUNK_SIZE,
        -BACKGROUND_ZINDEX_ABS,
    )
}

fn world_coords_to_map_indices(position: Vec3) -> (i32, i32) {
    let x_index = (position.x / CHUNK_SIZE) as i32 + if position.x < 0.0 { -1 } else { 0 };
    let y_index = (position.y / CHUNK_SIZE) as i32 + if position.y < 0.0 { -1 } else { 0 };
    (x_index, y_index)
}

fn map_indices_to_index(x_index: i32, y_index: i32, seed: u32) -> usize {
    let m = x_index.abs() as u64;
    let n = y_index.abs() as u64;

    let seed: u64 = seed as u64 + m + n;
    let mut rng = GameRng::seed_from_u64(seed);
    rng.gen_range(0..IIDS.len())
}

fn level_set_from_map_indices(x_index: i32, y_index: i32, seed: u32) -> LevelSet {
    let index = map_indices_to_index(x_index, y_index, seed);
    if index >= IIDS.len() {
        return LevelSet::from_iids([IIDS[0]]);
    }
    LevelSet::from_iids([IIDS[index]])
}

fn adjust_chunks(
    mut commands: Commands,
    assets: Res<GameAssets>,
    seed: Res<Seed>,
    q_player: Query<&Transform, With<Player>>,
    q_chunks: Query<(Entity, &Chunk)>,
) {
    let player_pos = q_player.single().translation;

    let max_index =
        world_coords_to_map_indices(player_pos + Vec3::new(CAMERA_SIZE_X, CAMERA_SIZE_Y, 0.0));
    let min_index =
        world_coords_to_map_indices(player_pos + Vec3::new(-CAMERA_SIZE_X, -CAMERA_SIZE_Y, 0.0));

    let mut existing_indices: Vec<(i32, i32)> = Vec::new();
    for (entity, chunk) in &q_chunks {
        if chunk.x >= min_index.0
            && chunk.x <= max_index.0
            && chunk.y >= min_index.1
            && chunk.y <= max_index.1
        {
            existing_indices.push((chunk.x, chunk.y));
        } else {
            commands.entity(entity).despawn_recursive();
        }
    }

    for i in min_index.0..max_index.0 + 1 {
        for j in min_index.1..max_index.1 + 1 {
            if existing_indices.contains(&(i, j)) {
                continue;
            }

            commands.spawn((
                Chunk { x: i, y: j },
                LdtkWorldBundle {
                    transform: Transform::from_translation(map_indices_to_world_coords(i, j)),
                    ldtk_handle: assets.level.clone(),
                    level_set: LevelSet::from_iids(level_set_from_map_indices(i, j, seed.0)),
                    ..Default::default()
                },
            ));
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseZeroTranslation,
                ..default()
            })
            .add_systems(Update, (adjust_chunks).run_if(in_state(GameState::Gaming)));
    }
}
