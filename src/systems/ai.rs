use bevy::prelude::*;

use crate::resources::{AIDelay, BoardState, CurrentAIDifficulty};
use crate::events::PlayerMoveEvent;
use crate::types::{Player, Difficulty};
use crate::ai_logic::get_best_move;

pub fn ai_make_move(
  time: Res<Time>,
  mut ai_delay: ResMut<AIDelay>,
  board_state: Res<BoardState>,
  ai_difficulty: Res<CurrentAIDifficulty>,
  mut player_move_events: EventWriter<PlayerMoveEvent>,
) {
  if board_state.game_over || board_state.current_player != Player::O {
    ai_delay.timer.reset();
    return;
  }

  // Update the timer
  ai_delay.timer.tick(time.delta());

  // Only make a move when the timer finishes
  if ai_delay.timer.just_finished() {
    info!("AI timer finished! Making move...");
    info!("AI Player O, Difficulty {:?} is thinking...", ai_difficulty.0);

    // Convert difficulty to depth
    let depth = match ai_difficulty.0 {
      Difficulty::Easy => 1,
      Difficulty::Medium => 3,
      Difficulty::Hard => 5,
    };

    // Get best move and handle the Option return type
    if let Some(best_move) = get_best_move(&board_state.board, Player::O, depth) {
      info!("AI Player O chose ({}, {})", best_move.row, best_move.col);

      player_move_events.send(PlayerMoveEvent {
        row: best_move.row,
        col: best_move.col,
        player: Player::O,
      });
    } else {
      // Fallback to any empty cell if no good move found
      for row in 0..3 {
        for col in 0..3 {
          if let crate::types::CellState::Empty = board_state.board[row][col] {
            info!("AI Player O chose fallback move ({}, {})", row, col);
            player_move_events.send(PlayerMoveEvent {
              row,
              col,
              player: Player::O,
            });
            return;
          }
        }
      }
    }
  }
  
use crate::types::{Player, Difficulty};
use crate::resources::{BoardState, CurrentAIDifficulty, CurrentGameMode};
use crate::events::PlayerMoveEvent;
use crate::ai_logic::{get_ai_move};

pub fn ai_make_move(
  board_state: Res<BoardState>,
  current_ai_difficulty: Res<CurrentAIDifficulty>,
  mut player_move_events: EventWriter<PlayerMoveEvent>,
) {
  if board_state.game_over {
    return;
  }

  info!("AI Player {:?}, Difficulty {:?} is thinking...",
    board_state.current_player, current_ai_difficulty.0);

    if let Some(ai_move) = get_ai_move(
      &board_state.board,
      board_state.current_player,
      current_ai_difficulty.0,
    ) {
      player_move_events.send(PlayerMoveEvent {
        row: ai_move.row,
        col: ai_move.col,
        player: board_state.current_player,
      });
      info!("AI Player {:?} chose ({}, {})", board_state.current_player, ai_move.row, ai_move.col);
    } else {
      warn!("AI could not find a move, but game is not over. This indicates a potential logic issue or a very late draw detection.");
    }
}
