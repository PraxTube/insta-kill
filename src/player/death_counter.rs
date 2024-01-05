use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DeathCounter {
    kills: u32,
}

impl DeathCounter {
    pub fn increase(&mut self) {
        self.kills += 1;
    }

    pub fn kills(&self) -> u32 {
        self.kills
    }
}

pub struct PlayerDeathCounterPlugin;

impl Plugin for PlayerDeathCounterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeathCounter>();
    }
}
