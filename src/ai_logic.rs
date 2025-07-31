use crate::config::{EASY_AI_RANDOM_CHANCE, HARD_AI_DEPTH, MEDIUM_AI_DEPTH};
use crate::types::{CellState, GameResult, Player};
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BoardMove {
    pub row: usize,
    pub col: usize,
}

pub fn check_winner(board: &[[CellState; 3]; 3]) -> Option<Player> {
    for row in 0..3 {
        if let (CellState::Occupied(p1), CellState::Occupied(p2), CellState::Occupied(p3)) =
            (board[row][0], board[row][1], board[row][2])
        {
            if p1 == p2 && p2 == p3 {
                return Some(p1);
            }
        }
    }

    for col in 0..3 {
        if let (CellState::Occupied(p1), CellState::Occupied(p2), CellState::Occupied(p3)) =
            (board[0][col], board[1][col], board[2][col])
        {
            if p1 == p2 && p2 == p3 {
                return Some(p1);
            }
        }
    }

    if let (CellState::Occupied(p1), CellState::Occupied(p2), CellState::Occupied(p3)) =
        (board[0][0], board[1][1], board[2][2])
    {
        if p1 == p2 && p2 == p3 {
            return Some(p1);
        }
    }

    if let (CellState::Occupied(p1), CellState::Occupied(p2), CellState::Occupied(p3)) =
        (board[0][2], board[1][1], board[2][0])
    {
        if p1 == p2 && p2 == p3 {
            return Some(p1);
        }
    }

    None
}

pub fn is_board_full(board: &[[CellState; 3]; 3]) -> bool {
    for row in 0..3 {
        for col in 0..3 {
            if matches!(board[row][col], CellState::Empty) {
                return false;
            }
        }
    }
    true
}

pub fn get_game_result(board: &[[CellState; 3]; 3]) -> GameResult {
    if let Some(winner) = check_winner(board) {
        GameResult::Win(winner)
    } else if is_board_full(board) {
        GameResult::Draw
    } else {
        GameResult::InProgress
    }
}

fn evaluate_board(board: &[[CellState; 3]; 3], maximizing_player: Player) -> i32 {
    match check_winner(board) {
        Some(winner) if winner == maximizing_player => 10,
        Some(_) => -10,
        None if is_board_full(board) => 0,
        None => 0,
    }
}

fn minimax(
    board: &mut [[CellState; 3]; 3],
    depth: i32,
    is_maximizing: bool,
    alpha: i32,
    beta: i32,
    maximizing_player: Player,
) -> i32 {
    let score = evaluate_board(board, maximizing_player);

    if score == 10 || score == -10 || depth == 0 || is_board_full(board) {
        return score - depth;
    }

    let current_player = if is_maximizing {
        maximizing_player
    } else {
        maximizing_player.opposite()
    };

    let mut alpha = alpha;
    let mut beta = beta;

    if is_maximizing {
        let mut max_eval = i32::MIN;

        for mov in find_empty_cells(board) {
            board[mov.row][mov.col] = CellState::Occupied(current_player);
            let eval = minimax(board, depth - 1, false, alpha, beta, maximizing_player);
            board[mov.row][mov.col] = CellState::Empty;

            max_eval = max_eval.max(eval);
            alpha = alpha.max(eval);

            if beta <= alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = i32::MAX;

        for mov in find_empty_cells(board) {
            board[mov.row][mov.col] = CellState::Occupied(current_player);
            let eval = minimax(board, depth - 1, true, alpha, beta, maximizing_player);
            board[mov.row][mov.col] = CellState::Empty;

            min_eval = min_eval.min(eval);
            beta = beta.min(eval);

            if beta <= alpha {
                break;
            }
        }
        min_eval
    }
}

pub fn get_best_move(board: &[[CellState; 3]; 3], player: Player, depth: i32) -> Option<BoardMove> {
    let mut board_copy = *board;
    let mut best_move = None;
    let mut best_value = i32::MIN;

    for mov in find_empty_cells(board) {
        board_copy[mov.row][mov.col] = CellState::Occupied(player);
        let move_value = minimax(
            &mut board_copy,
            depth - 1,
            false,
            i32::MIN,
            i32::MAX,
            player,
        );
        board_copy[mov.row][mov.col] = CellState::Empty;

        if move_value > best_value {
            best_value = move_value;
            best_move = Some(mov);
        }
    }
    best_move
}

// Easy AI - mostly random moves with some randomness
pub fn get_easy_ai_move(board: &[[CellState; 3]; 3], _player: Player) -> Option<BoardMove> {
    let mut rng = thread_rng();
    if rng.r#gen::<f32>() < EASY_AI_RANDOM_CHANCE {
        get_random_ai_move(board)
    } else {
        get_best_move(board, _player, 1).or_else(|| get_random_ai_move(board))
    }
}

pub fn get_medium_ai_move(board: &[[CellState; 3]; 3], player: Player) -> Option<BoardMove> {
    get_best_move(board, player, MEDIUM_AI_DEPTH).or_else(|| get_random_ai_move(board))
}

pub fn get_hard_ai_move(board: &[[CellState; 3]; 3], player: Player) -> Option<BoardMove> {
    get_best_move(board, player, HARD_AI_DEPTH).or_else(|| get_random_ai_move(board))
}

pub fn get_ai_move(
    board: &[[CellState; 3]; 3],
    player: Player,
    difficulty: crate::types::Difficulty,
) -> Option<BoardMove> {
    use crate::types::Difficulty;

    match difficulty {
        Difficulty::Easy => get_easy_ai_move(board, player),
        Difficulty::Medium => get_medium_ai_move(board, player),
        Difficulty::Hard => get_hard_ai_move(board, player),
    }
}

pub fn find_empty_cells(board: &[[CellState; 3]; 3]) -> Vec<BoardMove> {
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

pub fn get_random_ai_move(board: &[[CellState; 3]; 3]) -> Option<BoardMove> {
    let empty_cells = find_empty_cells(board);

    empty_cells.choose(&mut thread_rng()).copied()
}

impl BoardMove {
    pub fn new(row: usize, col: usize) -> Option<Self> {
        if row < 3 && col < 3 {
            Some(BoardMove { row, col })
        } else {
            None
        }
    }

    pub fn is_valid(&self) -> bool {
        self.row < 3 && self.col < 3
    }
}

pub fn apply_move(board: &mut [[CellState; 3]; 3], mov: BoardMove, player: Player) -> bool {
    if matches!(board[mov.row][mov.col], CellState::Empty) {
        board[mov.row][mov.col] = CellState::Occupied(player);
        true
    } else {
        false
    }
}
