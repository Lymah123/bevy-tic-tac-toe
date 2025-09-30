#[cfg(test)]
mod tests {
    use crate::ai_logic::*;
    use crate::types::*;

    // Helper function to create test boards
    fn create_test_board(moves: &[(usize, usize, Player)]) -> Board {
        let mut board = [[None; 3]; 3];
        for &(row, col, player) in moves {
            board[row][col] = Some(player);
        }
        board
    }

    #[test]
    fn test_check_winner_empty_board() {
        let board = create_test_board(&[]);
        assert_eq!(check_winner(&board), None);
    }

    #[test]
    fn test_check_winner_row() {
        let board = create_test_board(&[
            (0, 0, Player::X),
            (0, 1, Player::X),
            (0, 2, Player::X),
        ]);
        assert_eq!(check_winner(&board), Some(Player::X));
    }

    #[test]
    fn test_check_winner_column() {
        let board = create_test_board(&[
            (0, 0, Player::O),
            (1, 0, Player::O),
            (2, 0, Player::O),
        ]);
        assert_eq!(check_winner(&board), Some(Player::O));
    }

    #[test]
    fn test_check_winner_diagonal() {
        let board = create_test_board(&[
            (0, 0, Player::X),
            (1, 1, Player::X),
            (2, 2, Player::X),
        ]);
        assert_eq!(check_winner(&board), Some(Player::X));
    }

    #[test]
    fn test_check_winner_anti_diagonal() {
        let board = create_test_board(&[
            (0, 2, Player::O),
            (1, 1, Player::O),
            (2, 0, Player::O),
        ]);
        assert_eq!(check_winner(&board), Some(Player::O));
    }

    #[test]
    fn test_is_board_full_empty() {
        let board = create_test_board(&[]);
        assert!(!is_board_full(&board));
    }

    #[test]
    fn test_is_board_full_partial() {
        let board = create_test_board(&[
            (0, 0, Player::X),
            (1, 1, Player::O),
        ]);
        assert!(!is_board_full(&board));
    }

    #[test]
    fn test_is_board_full_complete() {
        let mut board: Board = [[None; 3]; 3];
        for (row, board_row) in board.iter_mut().enumerate() {
            for (col, cell) in board_row.iter_mut().enumerate() {
                *cell = Some(if (row + col) % 2 == 0 {
                    Player::X
                } else {
                    Player::O
                });
            }
        }
        assert!(is_board_full(&board));
    }

    #[test]
    fn test_find_empty_cells() {
        let board = create_test_board(&[
            (0, 0, Player::X),
            (1, 1, Player::O),
        ]);
        let empty_cells = find_empty_cells(&board);
        assert_eq!(empty_cells.len(), 7);
    }

    #[test]
    fn test_get_best_move_simple() {
        let board = create_test_board(&[
            (0, 0, Player::X),
            (0, 1, Player::X),
        ]);
        let ai_move = get_best_move(&board, Player::O);
        assert_eq!(ai_move, Some((0, 2)), "AI should block X's winning move");
    }

    #[test]
    fn test_get_best_move_winning() {
        let board = create_test_board(&[
            (0, 0, Player::O),
            (0, 1, Player::O),
        ]);
        let ai_move = get_best_move(&board, Player::O);
        assert_eq!(ai_move, Some((0, 2)), "AI should take the winning move");
    }
}
