pub mod super_sonic;

use bevy::prelude::*;

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((super_sonic::EffectSuperSonicPlugin,));
    }
}
