use bevy::prelude::*;
use crate::ai_logic::get_best_move;
use crate::events::PlayerMoveEvent;
use crate::resources::{AIDelay, BoardState, CurrentAIDifficulty};
use crate::types::{Difficulty, Player};

pub fn ai_make_move(
    time: Res<Time>,
    mut ai_delay: ResMut<AIDelay>,
    board_state: Res<BoardState>,
    ai_difficulty: Res<CurrentAIDifficulty>,
    mut player_move_events: EventWriter<PlayerMoveEvent>,
) {
    // Early exit conditions
    if board_state.game_over {
        ai_delay.timer.reset();
        return;
    }

    if board_state.current_player != Player::O {
        ai_delay.timer.reset();
        return;
    }

    // Update the timer
    ai_delay.timer.tick(time.delta());

    // Only make a move when the timer finishes (just_finished = only once)
    if ai_delay.timer.just_finished() {
        println!("🤖 AI making move...");

        // Convert difficulty to depth (for future use)
        let _depth = match ai_difficulty.0 {
            Difficulty::Easy => 1,
            Difficulty::Medium => 3,
            Difficulty::Hard => 5,
        };

        // Get best move
        if let Some(best_move) = get_best_move(&board_state.board, Player::O) {
            println!("🎯 AI chooses: ({}, {})", best_move.0, best_move.1);

            player_move_events.send(PlayerMoveEvent {
                row: best_move.0,
                col: best_move.1,
                player: Player::O,
            });
        } else {
            println!("❌ AI couldn't find a move! Looking for any empty cell...");

            // Fallback to any empty cell
            for row in 0..3 {
                for col in 0..3 {
                    if let crate::types::CellState::Empty = board_state.board[row][col] {
                        println!("🔄 AI fallback move: ({}, {})", row, col);

                        player_move_events.send(PlayerMoveEvent {
                            row,
                            col,
                            player: Player::O,
                        });
                        return;
                    }
                }
            }
            println!("💀 No empty cells found!");
        }

        // Reset timer after making a move (or trying to)
        ai_delay.timer.reset();
    }
}
