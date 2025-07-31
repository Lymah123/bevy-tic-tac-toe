use crate::types::{CellState, Difficulty, GameMode, Player};
use bevy::prelude::*;
use bevy::time::TimerMode;

#[derive(Resource)]
pub struct BoardState {
    pub board: [[CellState; 3]; 3],
    pub current_player: Player,
    pub game_over: bool,
    pub winner: Option<Player>,
}

#[derive(Resource)]
pub struct CurrentGameMode(pub GameMode);

#[derive(Resource)]
pub struct CurrentAIDifficulty(pub Difficulty);

#[derive(Resource)]
pub struct AIDelay {
    pub timer: Timer,
}

impl Default for AIDelay {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

// Default Implementation
impl Default for BoardState {
    fn default() -> Self {
        Self {
            board: [[CellState::Empty; 3]; 3],
            current_player: Player::X,
            game_over: false,
            winner: None,
        }
    }
}

// Game Statistics
#[derive(Resource, Default)]
pub struct GameStats {
    pub x_wins: u32,
    pub o_wins: u32,
    pub draws: u32,
    pub total_games: u32,
}
