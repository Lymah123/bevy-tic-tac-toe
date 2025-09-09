#[cfg(test)]
mod tests {
    use crate::ai_logic::*;
    use crate::types::*;

    #[test]
    fn test_empty_board_has_no_winner() {
        let board = [[CellState::Empty; 3]; 3];
        assert_eq!(check_winner(&board), None);
    }

    #[test]
    fn test_row_win() {
        let mut board = [[CellState::Empty; 3]; 3];
        board[0][0] = CellState::Occupied(Player::X);
        board[0][1] = CellState::Occupied(Player::X);
        board[0][2] = CellState::Occupied(Player::X);

        assert_eq!(check_winner(&board), Some(Player::X));
    }

    #[test]
    fn test_column_win() {
        let mut board = [[CellState::Empty; 3]; 3];
        board[0][0] = CellState::Occupied(Player::O);
        board[1][0] = CellState::Occupied(Player::O);
        board[2][0] = CellState::Occupied(Player::O);

        assert_eq!(check_winner(&board), Some(Player::O));
    }

    #[test]
    fn test_diagonal_win() {
        let mut board = [[CellState::Empty; 3]; 3];
        board[0][0] = CellState::Occupied(Player::X);
        board[1][1] = CellState::Occupied(Player::X);
        board[2][2] = CellState::Occupied(Player::X);

        assert_eq!(check_winner(&board), Some(Player::X));
    }

    #[test]
    fn test_anti_diagonal_win() {
        let mut board = [[CellState::Empty; 3]; 3];
        board[0][2] = CellState::Occupied(Player::O);
        board[1][1] = CellState::Occupied(Player::O);
        board[2][0] = CellState::Occupied(Player::O);

        assert_eq!(check_winner(&board), Some(Player::O));
    }

    #[test]
    fn test_empty_board_not_full() {
        let board = [[CellState::Empty; 3]; 3];
        assert!(!is_board_full(&board));
    }

    #[test]
    fn test_full_board() {
        let board = [
            [
                CellState::Occupied(Player::X),
                CellState::Occupied(Player::O),
                CellState::Occupied(Player::X),
            ],
            [
                CellState::Occupied(Player::O),
                CellState::Occupied(Player::X),
                CellState::Occupied(Player::O),
            ],
            [
                CellState::Occupied(Player::O),
                CellState::Occupied(Player::X),
                CellState::Occupied(Player::O),
            ],
        ];
        assert!(is_board_full(&board));
    }

    #[test]
    fn test_player_to_char() {
        assert_eq!(Player::X.to_char(), 'X');
        assert_eq!(Player::O.to_char(), 'O');
    }

    #[test]
    fn test_next_player() {
        assert_eq!(Player::X.next_player(), Player::O);
        assert_eq!(Player::O.next_player(), Player::X);
    }

    #[test]
    fn test_find_empty_cells() {
        let mut board = [[CellState::Empty; 3]; 3];
        board[0][0] = CellState::Occupied(Player::X);
        board[1][1] = CellState::Occupied(Player::O);

        let empty_cells = find_empty_cells(&board);
        assert_eq!(empty_cells.len(), 7);
    }

    #[test]
    fn test_ai_blocks_winning_move() {
        let mut board = [[CellState::Empty; 3]; 3];
        // Human has two X's in a row, AI should block
        board[0][0] = CellState::Occupied(Player::X);
        board[0][1] = CellState::Occupied(Player::X);

        let ai_move = get_best_move(&board, Player::O);
        assert_eq!(ai_move, Some((0, 2)));
    }

    #[test]
    fn test_ai_takes_winning_move() {
        let mut board = [[CellState::Empty; 3]; 3];
        board[1][0] = CellState::Occupied(Player::O);
        board[1][1] = CellState::Occupied(Player::O);

        let ai_move = get_best_move(&board, Player::O);
        assert_eq!(ai_move, Some((1, 2)));
    }
}
