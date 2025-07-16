use bevy::prelude::*;

mod config;
mod events;
mod resources;
mod types;
mod components;
mod ai_logic;

use resources::{BoardState, CurrentGameMode, CurrentAIDifficulty, GameStats};
use types::{Player, GameMode, Difficulty};
use events::{PlayerMoveEvent, GameOverEvent};

mod systems;
use systems::setup::setup_game;
use systems::input::handle_mouse_clicks;
use systems::gameplay::{apply_player_move, check_game_state};
use systems::ai::ai_make_move;
use systems::ui::{display_game_over_ui, handle_restart_button};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tic-Tac-Toe".into(),
                resolution: (config::WINDOW_WIDTH, config::WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(config::BACKGROUND_COLOR))
        .add_event::<PlayerMoveEvent>()
        .add_event::<GameOverEvent>()
        .insert_resource(BoardState::default())
        .insert_resource(CurrentGameMode(GameMode::HumanVsAI))
        .insert_resource(CurrentAIDifficulty(Difficulty::Hard))
        .insert_resource(GameStats::default())
        .add_systems(Startup, setup_game)
        .add_systems(
            Update,
            (
                handle_mouse_clicks.run_if(
                    |board_state: Res<BoardState>, game_mode: Res<CurrentGameMode>| {
                        !board_state.game_over
                            && (game_mode.0 == GameMode::HumanVsHuman
                                || (game_mode.0 == GameMode::HumanVsAI
                                    && board_state.current_player == Player::X))
                    },
                ),
                ai_make_move.run_if(
                    |board_state: Res<BoardState>, game_mode: Res<CurrentGameMode>| {
                        !board_state.game_over
                            && game_mode.0 == GameMode::HumanVsAI
                            && board_state.current_player == Player::O
                    },
                ),
            )
                .chain(),
        )
        .add_systems(Update, apply_player_move.after(handle_mouse_clicks).after(ai_make_move))
        .add_systems(Update, check_game_state.after(apply_player_move))
        .add_systems(Update, display_game_over_ui.after(check_game_state))
        .add_systems(Update, handle_restart_button)
        .run();
}
