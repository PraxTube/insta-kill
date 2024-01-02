pub mod dash;
pub mod hook;
pub mod input;
pub mod reflection_projectile;
pub mod spawn;
pub mod speed_timer;
pub mod state;
pub mod strike;

mod collision;
mod movement;

use bevy::prelude::*;

use state::PlayerState;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::new(100.0, 100.0, 0.0);

const MOVE_SPEED: f32 = 400.0;
const SLIDE_SPEED: f32 = 1200.0;
const DASH_MULTIPLIER: f32 = 2.0;
const HOOK_TIME: f32 = 0.55;
const HOOK_SLIDE_DISTANCE: f32 = 90.0;
const PLAYER_HITBOX_OFFSET: Vec3 = Vec3::new(0.0, -10.0, 0.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            movement::PlayerMovementPlugin,
            spawn::PlayerSpawnPlugin,
            state::PlayerStatePlugin,
            strike::PlayerStrikePlugin,
            collision::PlayerCollisionPlugin,
            speed_timer::SpeedTimerPlugin,
            dash::PlayerDashPlugin,
            hook::PlayerHookPlugin,
            reflection_projectile::PlayerReflectionProjectilePlugin,
        ));
    }
}

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub current_direction: Vec2,
    pub hook_target_pos: Vec2,
    pub collider_entity: Entity,
    pub disabled: bool,
}

impl Player {
    fn new(collider_entity: Entity) -> Self {
        Self {
            state: PlayerState::default(),
            current_direction: Vec2::ZERO,
            hook_target_pos: Vec2::ZERO,
            collider_entity,
            disabled: false,
        }
    }
}
