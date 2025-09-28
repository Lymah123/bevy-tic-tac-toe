use crate::types::{check_winner, Board, Player};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub struct BoardMove {
    pub row: usize,
    pub col: usize,
}

fn minimax(
    board: &mut Board,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
) -> i32 {
    // Check for terminal states
    if let Some(winner) = check_winner(board) {
        return match winner {
            Player::O => 10 - depth,
            Player::X => depth - 10,
        };
    }

    if crate::types::is_board_full(board) {
        return 0;
    }

    if maximizing_player {
        // AI (Player::O) is maximizing
        let mut max_eval = i32::MIN;
        for row_idx in 0..3 {
            for col_idx in 0..3 {
                if board[row_idx][col_idx].is_none() {
                    board[row_idx][col_idx] = Some(Player::O);
                    let eval = minimax(board, depth + 1, alpha, beta, false);
                    board[row_idx][col_idx] = None;
                    max_eval = max_eval.max(eval);
                    alpha = alpha.max(eval);
                    if beta <= alpha {
                        break; // Alpha-beta pruning
                    }
                }
            }
            if beta <= alpha {
                break; // Alpha-beta pruning for outer loop
            }
        }
        max_eval
    } else {
        // Human (Player::X) is minimizing
        let mut min_eval = i32::MAX;
        for row_idx in 0..3 {
            for col_idx in 0..3 {
                if board[row_idx][col_idx].is_none() {
                    board[row_idx][col_idx] = Some(Player::X);
                    let eval = minimax(board, depth + 1, alpha, beta, true);
                    board[row_idx][col_idx] = None;
                    min_eval = min_eval.min(eval);
                    beta = beta.min(eval);
                    if beta <= alpha {
                        break; // Alpha-beta pruning
                    }
                }
            }
            if beta <= alpha {
                break; // Alpha-beta pruning for outer loop
            }
        }
        min_eval
    }
}

pub fn get_best_move(board: &Board, player: Player) -> Option<(usize, usize)> {
    println!("🤖 AI analyzing board for player {:?}:", player);

    let mut best_score = if player == Player::O {
        i32::MIN
    } else {
        i32::MAX
    };
    let mut best_move = None;
    let mut board_copy = *board;

    for row_idx in 0..3 {
        for col_idx in 0..3 {
            if board_copy[row_idx][col_idx].is_none() {
                board_copy[row_idx][col_idx] = Some(player);

                let score = if player == Player::O {
                    // AI (Player::O) is maximizing
                    minimax(&mut board_copy, 0, i32::MIN, i32::MAX, false)
                } else {
                    // Human/AI (Player::X) is minimizing
                    minimax(&mut board_copy, 0, i32::MIN, i32::MAX, true)
                };

                board_copy[row_idx][col_idx] = None;

                println!("   Move ({}, {}) -> Score: {}", row_idx, col_idx, score);

                let is_better = if player == Player::O {
                    score > best_score
                } else {
                    score < best_score
                };

                if is_better {
                    best_score = score;
                    best_move = Some((row_idx, col_idx));
                    println!(
                        "   ✅ New best move: ({}, {}) with score {}",
                        row_idx, col_idx, score
                    );
                }
            }
        }
    }

    println!(
        "🎯 Final decision: {:?} with score {}",
        best_move, best_score
    );
    best_move
}

#[allow(dead_code)]
pub fn find_empty_cells(board: &Board) -> Vec<BoardMove> {
    let mut empty_cells = Vec::new();
    for (row_idx, row) in board.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if cell.is_none() {
                empty_cells.push(BoardMove {
                    row: row_idx,
                    col: col_idx,
                });
            }
        }
    }
    empty_cells
}

#[allow(dead_code)]
pub fn get_game_result(board: &Board) -> Option<Player> {
    check_winner(board)
}

#[allow(dead_code)]
impl BoardMove {
    pub fn new(row: usize, col: usize) -> Option<Self> {
        if row < 3 && col < 3 {
            Some(BoardMove { row, col })
        } else {
            None
        }
    }
}
