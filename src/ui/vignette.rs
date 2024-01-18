use bevy::prelude::*;

use crate::{GameAssets, GameState};

fn spawn_vignette(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn(ImageBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        image: UiImage {
            texture: assets.vignette.clone(),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    });
}

pub struct VignettePlugin;

impl Plugin for VignettePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::AssetLoading), spawn_vignette);
    }
}
