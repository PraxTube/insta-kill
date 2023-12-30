mod game_over_ui;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(game_over_ui::GameOverUiPlugin);
    }
}
