use bevy::prelude::*;

use crate::components::{BoardPosition, CellMark};
use crate::config::{CELL_SIZE, MARKER_SIZE_RATIO, O_COLOR, X_COLOR};
use crate::events::{GameOverEvent, PlayerMoveEvent};
use crate::resources::BoardState;
use crate::types::{CellState, GameResult, Player};

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
        let mut mark_transform = Transform::default();

        for (entity, pos, transform) in q_board_cells.iter() {
            if pos.row == row && pos.col == col {
                found_cell_entity = Some(entity);
                mark_transform = *transform;
                mark_transform.translation.z = 1.0; // Place marker above cell
                break;
            }
        }

        let Some(cell_entity) = found_cell_entity else {
            error!("❗ Could not find cell at ({}, {})", row, col);
            continue;
        };

        // Update board state
        board_state.board[row][col] = CellState::Occupied(player);

        // Calculate marker properties
        let mark_font_size = CELL_SIZE * MARKER_SIZE_RATIO;
        let mark_color = match player {
            Player::X => X_COLOR,
            Player::O => O_COLOR,
        };

        // Create marker transform - RELATIVE to parent (cell)
        let marker_transform = Transform::from_xyz(0.0, 0.0, 1.0); // Center of cell, above it

        // Spawn the marker as a CHILD of the cell (no BoardPosition!)
        let marker_entity = commands.spawn((
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
            // NOTE: NO BoardPosition component here!
        )).id();

        // Make the marker a child of the cell
        commands.entity(cell_entity).add_child(marker_entity);

        // ✅ Clean log message
        info!("✅ {} placed at ({}, {})", player.to_char(), row, col);

        // 🔑 CRITICAL: Switch turns ONLY after a successful move
        board_state.current_player = board_state.current_player.opposite();
        info!("🔄 Turn: {}", board_state.current_player.to_char());
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
        GameResult::Win(winner) => {
            info!("🏆 {} WINS!", winner.to_char());
            board_state.game_over = true;
            board_state.winner = Some(winner);
            game_over_events.send(GameOverEvent::Win(winner));
        }
        GameResult::Draw => {
            info!("🤝 DRAW!");
            board_state.game_over = true;
            board_state.winner = None;
            game_over_events.send(GameOverEvent::Draw);
        }
        GameResult::InProgress => {
            // Game continues
        }
    }
}
