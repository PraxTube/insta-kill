mod combo_bar;
mod game_over;
mod kill_counter;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            game_over::GameOverPlugin,
            combo_bar::ComboBarPlugin,
            kill_counter::KillCounterPlugin,
        ));
    }
}
