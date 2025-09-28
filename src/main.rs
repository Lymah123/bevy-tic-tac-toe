use bevy::prelude::*;

mod ai_logic;
mod components;
mod config;
mod events;
mod resources;
mod types;

#[cfg(test)]
mod test;

use events::{GameOverEvent, PlayerMoveEvent};
use resources::{AIDelay, BoardState, CurrentAIDifficulty, CurrentGameMode, GameStats};
use types::{Difficulty, GameMode};

mod systems;
use systems::ai::ai_make_move;
use systems::gameplay::{apply_player_move, check_game_state};
use systems::input::handle_mouse_clicks;
use systems::setup::setup_game;
use systems::ui::{display_game_over_ui, handle_restart_button};

fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Info).expect("error initializing log");
    }

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tic-Tac-Toe".into(),
                canvas: Some("#bevy".to_owned()),
                resolution: (config::WINDOW_WIDTH, config::WINDOW_HEIGHT).into(),
                resizable: false,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
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
        .add_startup_system(setup_game)
        .add_systems((
            // Just add the systems directly - Bevy handles parameters automatically
            handle_mouse_clicks,
            ai_make_move,
            apply_player_move,
            check_game_state,
            display_game_over_ui,
            handle_restart_button,
        ))
        .run();
}
