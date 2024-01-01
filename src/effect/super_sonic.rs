use bevy::prelude::*;
use bevy_trickfilm::prelude::*;

use crate::{utils::quat_from_vec2, world::camera::YSort, GameAssets, GameState};

#[derive(Component)]
struct SuperSonic;
#[derive(Event)]
pub struct SpawnSuperSonic {
    pub pos: Vec2,
    pub dir: Vec2,
    pub scale_factor: f32,
}

fn spawn_super_sonics(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut ev_spawn_super_sonic: EventReader<SpawnSuperSonic>,
) {
    for ev in ev_spawn_super_sonic.read() {
        let mut animator = AnimationPlayer2D::default();
        animator.play(assets.super_sonic_animations[0].clone());

        let transform = Transform::from_translation(ev.pos.extend(0.0))
            .with_rotation(quat_from_vec2(ev.dir))
            .with_scale(Vec3::splat(ev.scale_factor));

        commands.spawn((
            SuperSonic,
            animator,
            YSort(1.0),
            SpriteSheetBundle {
                transform,
                texture_atlas: assets.super_sonic.clone(),
                ..default()
            },
        ));
    }
}

fn despawn_super_sonics(
    mut commands: Commands,
    q_super_sonics: Query<(Entity, &AnimationPlayer2D), With<SuperSonic>>,
) {
    for (entity, animator) in &q_super_sonics {
        if animator.is_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct EffectSuperSonicPlugin;

impl Plugin for EffectSuperSonicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (spawn_super_sonics, despawn_super_sonics).run_if(in_state(GameState::Gaming)),
        )
        .add_event::<SpawnSuperSonic>();
    }
}
