pub mod camera;
pub mod camera_shake;
pub mod world_debug;

mod map;
mod restart;

pub use camera::MainCamera;
pub use camera_shake::CameraShake;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const BACKGROUND_ZINDEX_ABS: f32 = 1000.0;
const CHUNK_SIZE: f32 = 32.0 * 32.0;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            camera::CameraPlugin,
            camera_shake::CameraShakePlugin,
            world_debug::WorldDebugPlugin,
            map::MapPlugin,
            restart::RestartPlugin,
        ))
        .add_systems(Startup, configure_physics);
    }
}

fn configure_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::ZERO;
}
