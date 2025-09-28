use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Player {
    #[default]
    X,
    O,
}

impl Player {
    // Returns the character representation of the player('X' or 'O').
    pub fn to_char(self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }

    // Returns the other player.
    #[allow(dead_code)]
    pub fn next_player(self) -> Player {
        self.opposite()
    }

    // Returns the opposite player. Useful for AI minimax logic.
    pub fn opposite(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

// Represents the state of the single cell on the Tic-Tac-toe board
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
#[allow(dead_code)]
pub enum CellState {
    Empty,
    Occupied(Player),
}

pub type Board = [[Option<Player>; 3]; 3];

pub fn check_winner(board: &Board) -> Option<Player> {
    // Check rows
    for row in board {
        if let Some(player) = row[0] {
            if row[1] == Some(player) && row[2] == Some(player) {
                return Some(player);
            }
        }
    }

    // Check columns
    for col in 0..3 {
        if let Some(player) = board[0][col] {
            if board[1][col] == Some(player) && board[2][col] == Some(player) {
                return Some(player);
            }
        }
    }

    // Check diagonals
    if let Some(player) = board[1][1] {
        if (board[0][0] == Some(player) && board[2][2] == Some(player))
            || (board[0][2] == Some(player) && board[2][0] == Some(player))
        {
            return Some(player);
        }
    }

    None
}

// Represent the possible outcomes of the game
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
#[allow(dead_code)]
pub enum GameResult {
    Win(Player),
    Draw,
    InProgress,
}

// Represents the difficulty level for the AI opponent.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Default)]
pub enum Difficulty {
    #[allow(dead_code)]
    Easy,
    #[default]
    Medium,
    Hard,
}

// Represents the current mode of the game (e.g., Human vs Human, Human vs AI, AI vs AI).
#[derive(Debug, PartialEq, Clone, Copy, Eq, Default)]
pub enum GameMode {
    HumanVsHuman,
    #[default]
    HumanVsAI,
    #[allow(dead_code)]
    AIVsAI,
}

// Helper function to check if the board is full (for draw detection)
pub fn is_board_full(board: &Board) -> bool {
    board
        .iter()
        .all(|row| row.iter().all(|cell| cell.is_some()))
}

// Get the current game result
#[allow(dead_code)]
pub fn get_game_result(board: &Board) -> GameResult {
    if let Some(winner) = check_winner(board) {
        GameResult::Win(winner)
    } else if is_board_full(board) {
        GameResult::Draw
    } else {
        GameResult::InProgress
    }
}
