use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{utils::quat_from_vec2, world::camera::YSort, GameAssets, GameState};

use super::{
    input::{MouseWorldCoords, PlayerInput},
    Player,
};

#[derive(Component)]
struct Strike;

#[derive(Event)]
pub struct SpawnStrike {
    pub transform: Transform,
}

fn spawn_strikes(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_strike: EventReader<SpawnStrike>,
) {
    for ev in ev_spawn_strike.read() {
        let mut animator = AnimationPlayer2D::default();
        animator.play(assets.player_strike_animations[0].clone());

        let collider = commands
            .spawn((
                Sensor,
                Collider::ball(4.0),
                CollisionGroups::default(),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0.0, -5.0, 0.0,
                ))),
            ))
            .id();

        commands
            .spawn((
                Strike,
                YSort(1.0),
                animator,
                SpriteSheetBundle {
                    texture_atlas: assets.player_strike.clone(),
                    transform: ev.transform.clone(),
                    ..default()
                },
            ))
            .push_children(&[collider]);
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
    q_player: Query<&Transform, With<Player>>,
    player_input: Res<PlayerInput>,
    mouse_coords: Res<MouseWorldCoords>,
    mut ev_spawn_strike: EventWriter<SpawnStrike>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    let rot = quat_from_vec2(mouse_coords.0 - player_transform.translation.truncate());

    if player_input.attack {
        let transform = Transform::from_translation(player_transform.translation)
            .with_rotation(rot)
            .with_scale(player_transform.scale);
        ev_spawn_strike.send(SpawnStrike { transform })
    }
}

fn move_strikes(
    q_player: Query<&Transform, With<Player>>,
    mut q_strikes: Query<&mut Transform, (With<Strike>, Without<Player>)>,
) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };

    for mut transform in &mut q_strikes {
        transform.translation = player_transform.translation;
    }
}

pub struct PlayerStrikePlugin;

impl Plugin for PlayerStrikePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_strikes, despawn_strikes, trigger_strike, move_strikes)
                .run_if(in_state(GameState::Gaming)),
        )
        .add_event::<SpawnStrike>();
    }
}
