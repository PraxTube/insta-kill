mod combo_bar;
mod death_counter;
mod game_over;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            game_over::GameOverPlugin,
            combo_bar::ComboBarPlugin,
            death_counter::DeathCounterPlugin,
        ));
    }
}
