use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::utils::DebugMode;

fn toggle_rapier_debug(mut debug_context: ResMut<DebugRenderContext>, debug_mode: Res<DebugMode>) {
    if debug_context.enabled != debug_mode.active {
        debug_context.enabled = debug_mode.active;
    }
}

pub struct WorldDebugPlugin;

impl Plugin for WorldDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (toggle_rapier_debug,));
    }
}
