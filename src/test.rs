#[cfg(test)]
mod tests {
    use crate::ai_logic::*;
    use crate::types::*;

    #[test]
    fn test_check_winner_empty_board() {
        let board: Board = [[None; 3]; 3];
        assert_eq!(check_winner(&board), None);
    }

    #[test]
    fn test_check_winner_row() {
        let mut board: Board = [[None; 3]; 3];
        board[0][0] = Some(Player::X);
        board[0][1] = Some(Player::X);
        board[0][2] = Some(Player::X);
        assert_eq!(check_winner(&board), Some(Player::X));
    }

    #[test]
    fn test_check_winner_column() {
        let mut board: Board = [[None; 3]; 3];
        board[0][0] = Some(Player::O);
        board[1][0] = Some(Player::O);
        board[2][0] = Some(Player::O);
        assert_eq!(check_winner(&board), Some(Player::O));
    }

    #[test]
    fn test_check_winner_diagonal() {
        let mut board: Board = [[None; 3]; 3];
        board[0][0] = Some(Player::X);
        board[1][1] = Some(Player::X);
        board[2][2] = Some(Player::X);
        assert_eq!(check_winner(&board), Some(Player::X));
    }

    #[test]
    fn test_check_winner_anti_diagonal() {
        let mut board: Board = [[None; 3]; 3];
        board[0][2] = Some(Player::O);
        board[1][1] = Some(Player::O);
        board[2][0] = Some(Player::O);
        assert_eq!(check_winner(&board), Some(Player::O));
    }

    #[test]
    fn test_is_board_full_empty() {
        let board: Board = [[None; 3]; 3];
        assert!(!is_board_full(&board));
    }

    #[test]
    fn test_is_board_full_partial() {
        let mut board: Board = [[None; 3]; 3];
        board[0][0] = Some(Player::X);
        board[1][1] = Some(Player::O);
        assert!(!is_board_full(&board));
    }

    #[test]
    fn test_is_board_full_complete() {
        let mut board: Board = [[None; 3]; 3];
        for row in 0..3 {
            for col in 0..3 {
                board[row][col] = Some(if (row + col) % 2 == 0 {
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
        let mut board: Board = [[None; 3]; 3];
        board[0][0] = Some(Player::X);
        board[1][1] = Some(Player::O);

        let empty_cells = find_empty_cells(&board);
        assert_eq!(empty_cells.len(), 7);
    }

    #[test]
    fn test_get_best_move_simple() {
        let mut board: Board = [[None; 3]; 3];
        board[0][0] = Some(Player::X);
        board[0][1] = Some(Player::X);

        let ai_move = get_best_move(&board, Player::O);
        assert!(ai_move.is_some());
        // AI should block the winning move
        assert_eq!(ai_move, Some((0, 2)));
    }

    #[test]
    fn test_get_best_move_winning() {
        let mut board: Board = [[None; 3]; 3];
        board[0][0] = Some(Player::O);
        board[0][1] = Some(Player::O);

        let ai_move = get_best_move(&board, Player::O);
        assert!(ai_move.is_some());
        // AI should take the winning move
        assert_eq!(ai_move, Some((0, 2)));
    }
}
