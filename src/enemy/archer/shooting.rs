use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{
    enemy::EnemyProjectile, player::Player, utils::quat_from_vec3, world::camera::YSort,
    GameAssets, GameState,
};

use super::{update_animations, ArcherState, EnemyArcher, SHOOT_RANGE};

const PROJECTILE_SPEED: f32 = 650.0;
const OFFSET: f32 = 10.0;

#[derive(Component)]
struct Projectile;

fn trigger_shooting(
    q_player: Query<&Transform, With<Player>>,
    mut q_archers: Query<(&Transform, &mut EnemyArcher), Without<Player>>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    for (archer_transform, mut archer) in &mut q_archers {
        if archer.state == ArcherState::Shooting || archer.state == ArcherState::Stunned {
            continue;
        }
        if !archer.moving_cooldown.finished() {
            continue;
        }

        if archer_transform.translation.distance_squared(player_pos) <= SHOOT_RANGE.powi(2) {
            archer.state = ArcherState::Shooting;
        }
    }
}

fn spawn_projectiles(
    mut commands: Commands,
    assets: Res<GameAssets>,
    q_player: Query<&Transform, With<Player>>,
    mut q_archers: Query<(&Transform, &AnimationPlayer2D, &mut EnemyArcher), Without<Player>>,
) {
    let player_pos = match q_player.get_single() {
        Ok(r) => r.translation,
        Err(_) => return,
    };

    for (archer_transform, animator, mut archer) in &mut q_archers {
        if archer.state != ArcherState::Shooting || !animator.is_finished() {
            continue;
        }

        let rot = quat_from_vec3(player_pos - archer_transform.translation);
        let transform = Transform::from_translation(
            archer_transform.translation + rot.mul_vec3(Vec3::X) * OFFSET,
        )
        .with_rotation(rot)
        .with_scale(Vec3::splat(2.0));

        let collider = commands
            .spawn((
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                Collider::cuboid(25.0, 6.0),
                CollisionGroups::default(),
                TransformBundle::from_transform(Transform::from_translation(Vec3::new(
                    0.0, 0.0, 0.0,
                ))),
            ))
            .id();

        let mut animator = AnimationPlayer2D::default();
        animator.play(assets.archer_projectile_animations[0].clone());

        commands
            .spawn((
                EnemyProjectile::default(),
                Projectile,
                YSort(1.0),
                animator,
                SpriteSheetBundle {
                    transform,
                    texture_atlas: assets.archer_projectile.clone(),
                    ..default()
                },
            ))
            .push_children(&[collider]);

        archer.moving_cooldown.reset();
        archer.state = ArcherState::Idling;
    }
}

fn move_projectiles(time: Res<Time>, mut q_projectiles: Query<&mut Transform, With<Projectile>>) {
    for mut transform in &mut q_projectiles {
        let dir = transform.local_x();
        transform.translation += dir * PROJECTILE_SPEED * time.delta_seconds();
    }
}

pub struct EnemyArcherShootingPlugin;

impl Plugin for EnemyArcherShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_projectiles.before(trigger_shooting),
                trigger_shooting.before(update_animations),
                move_projectiles,
            )
                .run_if(in_state(GameState::Gaming)),
        );
    }
}
