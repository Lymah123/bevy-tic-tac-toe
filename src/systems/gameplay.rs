use bevy::prelude::*;

use crate::components::{BoardPosition, CellMark};
use crate::config::{CELL_SIZE, MARKER_SIZE_RATIO, O_COLOR, X_COLOR};
use crate::events::{GameOverEvent, PlayerMoveEvent};
use crate::resources::BoardState;
use crate::types::{CellState, Player};

pub fn apply_player_move(
    mut commands: Commands,
    mut board_state: ResMut<BoardState>,
    mut player_move_events: EventReader<PlayerMoveEvent>,
    asset_server: Res<AssetServer>,
    q_board_cells: Query<(Entity, &BoardPosition, &Transform)>,
) {
    for event in player_move_events.read() {
        let row = event.row;
        let col = event.col;
        let player = event.player;

        // Check if game is over or cell is occupied
        if board_state.game_over {
            continue;
        }

        if matches!(board_state.board[row][col], CellState::Occupied(_)) {
            info!("🚫 Cell ({}, {}) already occupied", row, col);
            continue;
        }

        // Validate it's the correct player's turn
        if player != board_state.current_player {
            info!(
                "❌ Wrong turn! Expected {:?}, got {:?}",
                board_state.current_player, player
            );
            continue;
        }

        // Find the correct cell entity
        let mut found_cell_entity = None;

        for (entity, pos, _transform) in q_board_cells.iter() {
            if pos.row == row && pos.col == col {
                found_cell_entity = Some(entity);
                break;
            }
        }

        let Some(cell_entity) = found_cell_entity else {
            error!("❗ Could not find cell at ({}, {})", row, col);
            continue;
        };

        board_state.board[row][col] = CellState::Occupied(player);

        // Calculate marker properties
        let mark_font_size = CELL_SIZE * MARKER_SIZE_RATIO;
        let mark_color = match player {
            Player::X => X_COLOR,
            Player::O => O_COLOR,
        };

        let marker_transform = Transform::from_xyz(0.0, 0.0, 1.0);

        // Spawn the marker as a CHILD of the cell (no BoardPosition!)
        let marker_entity = commands
            .spawn((
                Text2dBundle {
                    text: Text::from_section(
                        player.to_char().to_string(),
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: mark_font_size,
                            color: mark_color,
                        },
                    ),
                    transform: marker_transform,
                    ..default()
                },
                CellMark(player),
            ))
            .id();

        // Make the marker a child of the cell
        commands.entity(cell_entity).add_child(marker_entity);

        info!("✅ {} placed at ({}, {})", player.to_char(), row, col);

        // 🔑 CRITICAL: Switch turns ONLY after a successful move
        board_state.current_player = board_state.current_player.opposite();
        info!("🔄 Turn: {}", board_state.current_player.to_char());

        println!("Current board state:");
        for row in 0..3 {
            for col in 0..3 {
                match board_state.board[row][col] {
                    CellState::Empty => print!(" . "),
                    CellState::Occupied(Player::X) => print!(" X "),
                    CellState::Occupied(Player::O) => print!(" O "),
                }
            }
            println!();
        }
        println!("Current player should be: {:?}", board_state.current_player);
    }
}

pub fn check_game_state(
    mut board_state: ResMut<BoardState>,
    mut game_over_events: EventWriter<GameOverEvent>,
) {
    if board_state.game_over {
        return;
    }

    let result = crate::ai_logic::get_game_result(&board_state.board);

    match result {
        Some(winner) => {
            info!("🏆 {} WINS!", winner.to_char());
            board_state.game_over = true;
            board_state.winner = Some(winner);
            game_over_events.send(GameOverEvent::Win(winner));
        }
        None => {
            // Check if board is full (draw)
            if crate::ai_logic::is_board_full(&board_state.board) {
                info!("🤝 DRAW!");
                board_state.game_over = true;
                board_state.winner = None;
                game_over_events.send(GameOverEvent::Draw);
            }
            // If no winner and board not full, game continues
        }
    }
}
