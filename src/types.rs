#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum Player {
    X,
    O,
}

impl Player {
  // Returns the character representation of the player('X' or 'O').
  pub fn to_char(&self) -> char {
    match self {
      Player::X => 'X',
      Player::O => 'O',
      }
  }

  // Returns the other player.
  pub fn next_player(&self) -> Player {
    self.opposite()
  }

  // Returns the opposite player. Useful for AI minimax logic.
  pub fn opposite(&self) -> Player {
    match self {
      Player::X => Player::O,
      Player::O => Player::X,
    }
  }
}

// Represents the state of the single cell on the Tic-Tac-toe board
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum CellState {
  Empty,
  Occupied(Player),
}

// Represent the possible outcomes of the game
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum GameResult {
  Win(Player),
  Draw,
  InProgress,
}

// Represents the difficulty level for the AI opponent.
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum Difficulty {
  Easy,
  Medium,
  Hard,
}

// Represents the current mode of the game (e.g,.. Human vs Human, Human vs AI).
#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum GameMode {
  HumanVsHuman,
  HumanVsAI,
  // AIVsAI(I will add that later)
}

impl Default for Player {
  fn default() -> Self {
    Player::X
  }
}

impl Default for Difficulty {
  fn default() -> Self {
    Difficulty::Medium
  }
}

impl Default for GameMode {
  fn default() -> Self {
    GameMode::HumanVsAI
  }
}
