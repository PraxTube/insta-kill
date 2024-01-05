use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct KillCounter {
    kills: u32,
}

impl KillCounter {
    pub fn increase(&mut self) {
        self.kills += 1;
    }

    pub fn kills(&self) -> u32 {
        self.kills
    }
}

pub struct PlayerKillCounterPlugin;

impl Plugin for PlayerKillCounterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KillCounter>();
    }
}
