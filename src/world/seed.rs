use bevy::prelude::*;
use chrono::Utc;

#[derive(Resource, Deref, DerefMut)]
pub struct Seed(pub u32);

pub struct GameSeedPlugin;

impl Plugin for GameSeedPlugin {
    fn build(&self, app: &mut App) {
        let seed = (Utc::now().timestamp_millis().abs() & 0xFFFF_FFFF) as u32;
        app.insert_resource(Seed(seed));
    }
}
