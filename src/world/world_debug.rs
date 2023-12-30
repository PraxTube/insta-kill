use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_screen_diagnostics::{
    ScreenDiagnostics, ScreenDiagnosticsPlugin, ScreenEntityDiagnosticsPlugin,
    ScreenFrameDiagnosticsPlugin,
};

use crate::GameState;

#[derive(Resource, Default)]
pub struct DebugMode {
    pub active: bool,
}

#[derive(Resource, Default)]
struct Diagnostics {
    active: bool,
}

fn toggle_rapier_debug(mut debug_context: ResMut<DebugRenderContext>, debug_mode: Res<DebugMode>) {
    if debug_context.enabled != debug_mode.active {
        debug_context.enabled = debug_mode.active;
    }
}

fn toggle_diags(screen_diags: &mut ResMut<ScreenDiagnostics>) {
    screen_diags.modify("fps").toggle();
    screen_diags.modify("ms/frame").toggle();
    screen_diags.modify("entities").toggle();
}

fn toggle_off(mut screen_diags: ResMut<ScreenDiagnostics>) {
    toggle_diags(&mut screen_diags);
}

fn toggle_diagnostics(
    debug_mode: Res<DebugMode>,
    mut diags: ResMut<Diagnostics>,
    mut screen_diags: ResMut<ScreenDiagnostics>,
) {
    if debug_mode.active != diags.active {
        diags.active = debug_mode.active;
        toggle_diags(&mut screen_diags);
    }
}

pub struct WorldDebugPlugin;

impl Plugin for WorldDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ScreenDiagnosticsPlugin {
                timestep: 1.0,
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(5.0),
                    left: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            ScreenFrameDiagnosticsPlugin,
            ScreenEntityDiagnosticsPlugin,
        ))
        .init_resource::<DebugMode>()
        .init_resource::<Diagnostics>()
        .add_systems(OnExit(GameState::AssetLoading), toggle_off)
        .add_systems(
            Update,
            toggle_diagnostics.run_if(resource_changed::<DebugMode>()),
        )
        .add_systems(Update, (toggle_rapier_debug,));
    }
}
