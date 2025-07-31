use bevy::prelude::*;

mod ai_logic;
mod components;
mod config;
mod events;
mod resources;
mod types;

use events::{GameOverEvent, PlayerMoveEvent};
use resources::{AIDelay, BoardState, CurrentAIDifficulty, CurrentGameMode, GameStats};
use types::{Difficulty, GameMode, Player};

mod systems;
use systems::ai::ai_make_move;
use systems::gameplay::{apply_player_move, check_game_state};
use systems::input::handle_mouse_clicks;
use systems::setup::setup_game;
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
        .insert_resource(AIDelay::default())
        .add_systems(Startup, setup_game)
        .add_systems(
            Update,
            (
                // Input handling - only for human players
                handle_mouse_clicks.run_if(
                    |board_state: Res<BoardState>, game_mode: Res<CurrentGameMode>| {
                        !board_state.game_over
                            && (game_mode.0 == GameMode::HumanVsHuman
                                || (game_mode.0 == GameMode::HumanVsAI
                                    && board_state.current_player == Player::X))
                    },
                ),
                // AI move - only when it's AI's turn
                ai_make_move.run_if(
                    |board_state: Res<BoardState>, game_mode: Res<CurrentGameMode>| {
                        !board_state.game_over
                            && game_mode.0 == GameMode::HumanVsAI
                            && board_state.current_player == Player::O
                    },
                ),
            ),
        )
        .add_systems(
            Update,
            (
                // Core game logic - these should run in order
                apply_player_move,
                check_game_state,
            ).chain(),
        )
        .add_systems(
            Update,
            (
                // UI systems - can run independently
                display_game_over_ui,
                handle_restart_button,
            ),
        )
        .run();
}
