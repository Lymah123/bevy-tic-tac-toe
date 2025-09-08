use crate::types::{CellState, Player, Board};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardMove {
    pub row: usize,
    pub col: usize,
}

pub fn check_winner(board: &Board) -> Option<Player> {
    // Check rows
    for row in 0..3 {
        if let (CellState::Occupied(p1), CellState::Occupied(p2), CellState::Occupied(p3)) =
            (board[row][0], board[row][1], board[row][2])
        {
            if p1 == p2 && p2 == p3 {
                return Some(p1);
            }
        }
    }

    // Check columns
    for col in 0..3 {
        if let (CellState::Occupied(p1), CellState::Occupied(p2), CellState::Occupied(p3)) =
            (board[0][col], board[1][col], board[2][col])
        {
            if p1 == p2 && p2 == p3 {
                return Some(p1);
            }
        }
    }

    // Check main diagonal
    if let (CellState::Occupied(p1), CellState::Occupied(p2), CellState::Occupied(p3)) =
        (board[0][0], board[1][1], board[2][2])
    {
        if p1 == p2 && p2 == p3 {
            return Some(p1);
        }
    }

    // Check anti-diagonal
    if let (CellState::Occupied(p1), CellState::Occupied(p2), CellState::Occupied(p3)) =
        (board[0][2], board[1][1], board[2][0])
    {
        if p1 == p2 && p2 == p3 {
            return Some(p1);
        }
    }

    None
}

pub fn is_board_full(board: &Board) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if matches!(board[row][col], CellState::Empty) {
                return false;
            }
        }
    }
    true
}

fn minimax(
    board: &Board,
    depth: i32,
    mut alpha: i32,
    mut beta: i32,
    maximizing_player: bool,
) -> i32 {
    // Check for terminal states
    if let Some(winner) = check_winner(board) {
        return match winner {
            Player::O => 10 - depth, // AI wins (prefer quicker wins)
            Player::X => depth - 10, // Human wins (prefer later losses)
        };
    }

    if is_board_full(board) {
        return 0; // Draw
    }

    if maximizing_player {
        // AI (Player::O) is maximizing
        let mut max_eval = i32::MIN;
        for row in 0..3 {
            for col in 0..3 {
                if let CellState::Empty = board[row][col] {
                    let mut new_board = *board;
                    new_board[row][col] = CellState::Occupied(Player::O);
                    let eval = minimax(&new_board, depth + 1, alpha, beta, false);
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
        for row in 0..3 {
            for col in 0..3 {
                if let CellState::Empty = board[row][col] {
                    let mut new_board = *board;
                    new_board[row][col] = CellState::Occupied(Player::X);
                    let eval = minimax(&new_board, depth + 1, alpha, beta, true);
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

    let mut best_score = i32::MIN;
    let mut best_move = None;

    for row in 0..3 {
        for col in 0..3 {
            if let CellState::Empty = board[row][col] {
                let mut new_board = *board;
                new_board[row][col] = CellState::Occupied(player);

                // AI (Player::O) is maximizing, so after AI moves, human minimizes
                let score = minimax(&new_board, 0, i32::MIN, i32::MAX, false);

                println!("   Move ({}, {}) -> Score: {}", row, col, score);

                if score > best_score {
                    best_score = score;
                    best_move = Some((row, col));
                    println!(
                        "   ✅ New best move: ({}, {}) with score {}",
                        row, col, score
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

pub fn find_empty_cells(board: &Board) -> Vec<BoardMove> {
    let mut empty_cells = Vec::new();
    for row in 0..3 {
        for col in 0..3 {
            if let CellState::Empty = board[row][col] {
                empty_cells.push(BoardMove { row, col });
            }
        }
    }
    empty_cells
}
pub fn get_game_result(board: &Board) -> Option<Player> {
    check_winner(board)
}

impl BoardMove {
    pub fn new(row: usize, col: usize) -> Option<Self> {
        if row < 3 && col < 3 {
            Some(BoardMove { row, col })
        } else {
            None
        }
    }
}
