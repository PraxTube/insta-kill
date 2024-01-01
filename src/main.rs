#![allow(clippy::too_many_arguments, clippy::type_complexity)]

mod assets;
mod effect;
mod enemy;
mod player;
mod ui;
mod utils;
mod world;

pub use assets::GameAssets;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode};

use bevy_asset_loader::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_trickfilm::Animation2DPlugin;

const BACKGROUND_COLOR: Color = Color::rgb(0.75, 0.6, 0.5);

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Gaming,
    GameOver,
    Restart,
}

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin {
                enabled: false,
                ..default()
            },
            Animation2DPlugin,
        ))
        .insert_resource(Msaa::Off)
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Gaming),
        )
        .configure_loading_state(
            LoadingStateConfig::new(GameState::AssetLoading).load_collection::<GameAssets>(),
        )
        .add_plugins((
            world::WorldPlugin,
            ui::UiPlugin,
            effect::EffectPlugin,
            player::PlayerPlugin,
            enemy::EnemyPlugin,
            utils::UtilsPlugin,
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
}
