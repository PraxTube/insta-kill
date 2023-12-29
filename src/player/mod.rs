pub mod input;
pub mod state;

mod movement;
mod spawn;
mod strike;

use bevy::prelude::*;

use state::PlayerState;

pub const PLAYER_SPAWN_POS: Vec3 = Vec3::new(100.0, 100.0, 0.0);

const MOVE_SPEED: f32 = 400.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            input::InputPlugin,
            movement::PlayerMovementPlugin,
            spawn::PlayerSpawnPlugin,
            state::PlayerStatePlugin,
            strike::PlayerStrikePlugin,
        ));
    }
}

#[derive(Component)]
pub struct Player {
    pub state: PlayerState,
    pub current_direction: Vec2,
    pub collider_entity: Entity,
}

impl Player {
    fn new(collider_entity: Entity) -> Self {
        Self {
            state: PlayerState::default(),
            current_direction: Vec2::ZERO,
            collider_entity,
        }
    }
}
