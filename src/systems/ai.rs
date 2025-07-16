use bevy::prelude::*;
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
