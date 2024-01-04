use bevy::prelude::*;

use crate::GameState;

/// Combo progress lost each second in percent.
const COMBO_DECAY: f32 = 20.0;

#[derive(Resource)]
pub struct ComboMeter {
    /// Float between 0.0 and 100.0, indicating the combo progress in percent.
    progress: f32,
}

impl Default for ComboMeter {
    fn default() -> Self {
        Self { progress: 100.0 }
    }
}

impl ComboMeter {
    fn decay(&mut self, decay: f32) {
        self.progress = (self.progress - decay.abs()).max(0.0);
    }

    pub fn progress(&self) -> f32 {
        self.progress
    }

    pub fn increase(&mut self, addition: f32) {
        self.progress = (self.progress + addition.abs()).min(100.0);
    }
}

fn decay_combo_progress(time: Res<Time>, mut combo_meter: ResMut<ComboMeter>) {
    combo_meter.decay(COMBO_DECAY * time.delta_seconds());
}

pub struct PlayerComboPlugin;

impl Plugin for PlayerComboPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (decay_combo_progress,).run_if(in_state(GameState::Gaming)),
        )
        .init_resource::<ComboMeter>();
    }
}
