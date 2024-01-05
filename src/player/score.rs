use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct PlayerScore {
    score: u32,
}

impl PlayerScore {
    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn add(&mut self, addition: u32) {
        self.score += addition;
    }
}

pub struct PlayerScorePlugin;

impl Plugin for PlayerScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerScore>();
    }
}
