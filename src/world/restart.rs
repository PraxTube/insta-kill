use bevy::prelude::*;

use crate::{
    player::{
        combo::ComboMeter, input::PlayerInput, kill_counter::KillCounter, speed_timer::SpeedTimer,
    },
    GameState,
};

#[derive(Resource, Deref, DerefMut)]
struct RestartTimer(Timer);

impl Default for RestartTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.2, TimerMode::Repeating))
    }
}

fn reset_resources(
    mut speed_timer: ResMut<SpeedTimer>,
    mut combo_meter: ResMut<ComboMeter>,
    mut death_counter: ResMut<KillCounter>,
) {
    *speed_timer = SpeedTimer::default();
    *combo_meter = ComboMeter::default();
    *death_counter = KillCounter::default();
}

fn initiate_restart(mut next_state: ResMut<NextState<GameState>>, player_input: Res<PlayerInput>) {
    if player_input.restart {
        next_state.set(GameState::Restart);
    }
}

fn restart(
    time: Res<Time>,
    mut restart_timer: ResMut<RestartTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    restart_timer.tick(time.delta());
    if restart_timer.just_finished() {
        next_state.set(GameState::Gaming);
    }
}

pub struct RestartPlugin;

impl Plugin for RestartPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RestartTimer>()
            .add_systems(OnEnter(GameState::Restart), (reset_resources,))
            .add_systems(
                Update,
                (initiate_restart,).run_if(in_state(GameState::GameOver)),
            )
            .add_systems(Update, (restart,).run_if(in_state(GameState::Restart)));
    }
}
