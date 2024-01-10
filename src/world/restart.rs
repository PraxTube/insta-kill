use bevy::prelude::*;

use crate::{
    player::{
        input::PlayerInput, kill_counter::KillCounter, score::PlayerScore, speed_timer::SpeedTimer,
    },
    ui::game_over::GameOverState,
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
    mut death_counter: ResMut<KillCounter>,
    mut player_score: ResMut<PlayerScore>,
) {
    *speed_timer = SpeedTimer::default();
    *death_counter = KillCounter::default();
    *player_score = PlayerScore::default();
}

fn initiate_restart_from_game_over(
    mut next_state: ResMut<NextState<GameState>>,
    player_input: Res<PlayerInput>,
) {
    if player_input.escape {
        next_state.set(GameState::Restart);
    }
}

fn initiate_restart_from_leaderboard(
    mut next_state: ResMut<NextState<GameState>>,
    player_input: Res<PlayerInput>,
) {
    if player_input.restart || player_input.escape {
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
                (initiate_restart_from_game_over,).run_if(
                    in_state(GameState::GameOver).and_then(in_state(GameOverState::GameOver)),
                ),
            )
            .add_systems(
                Update,
                (initiate_restart_from_leaderboard,).run_if(
                    in_state(GameState::GameOver).and_then(in_state(GameOverState::Leaderboard)),
                ),
            )
            .add_systems(Update, (restart,).run_if(in_state(GameState::Restart)));
    }
}
