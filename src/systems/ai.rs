use crate::ai_logic::get_best_move;
use crate::events::PlayerMoveEvent;
use crate::resources::{AIDelay, BoardState, CurrentAIDifficulty};
use crate::types::{Difficulty, Player};
use bevy::prelude::*;

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
        // Convert difficulty to depth
        let depth = match ai_difficulty.0 {
            Difficulty::Easy => 1,
            Difficulty::Medium => 3,
            Difficulty::Hard => 5,
        };

        // Get best move and handle the Option return type
        if let Some(best_move) = get_best_move(&board_state.board, Player::O, depth) {
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
}
