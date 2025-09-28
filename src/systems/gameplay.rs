use bevy::prelude::*;

use crate::components::{BoardPosition, CellMark};
use crate::config::{CELL_SIZE, MARKER_SIZE_RATIO, O_COLOR, X_COLOR};
use crate::events::{GameOverEvent, PlayerMoveEvent};
use crate::resources::BoardState;
use crate::types::Player;

pub fn apply_player_move(
    mut commands: Commands,
    mut board_state: ResMut<BoardState>,
    mut player_move_events: EventReader<PlayerMoveEvent>,
    q_board_cells: Query<(Entity, &BoardPosition, &Transform)>,
) {
    let event_count = player_move_events.len();
    if event_count > 0 {
        info!("📨 Received {} PlayerMoveEvent(s)", event_count);
    }

    for event in player_move_events.iter() {
        let (row, col) = event.position;
        let player = board_state.current_player;

        info!("🎯 Processing move: {} at ({},{})", player.to_char(), row, col);

        // Check if game is over or cell is occupied
        if board_state.game_over {
            info!("🚫 Game is over, ignoring move");
            continue;
        }

        if board_state.board[row][col].is_some() {
            info!("🚫 Cell ({}, {}) already occupied", row, col);
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

        info!("📍 Found cell entity for ({},{})", row, col);

        board_state.board[row][col] = Some(player);

        // Calculate marker properties
        let mark_font_size = CELL_SIZE * MARKER_SIZE_RATIO;
        let mark_color = match player {
            Player::X => X_COLOR,
            Player::O => O_COLOR,
        };

        info!("Font size: {:.2}, CELL_SIZE: {:.2}, MARKER_SIZE_RATIO: {:.2}", mark_font_size, CELL_SIZE, MARKER_SIZE_RATIO);
        info!("Color for {}: {:?}", player.to_char(), mark_color);

        // Spawn the marker entity
        let marker_entity = commands
            .spawn((
              SpriteBundle {
                sprite: Sprite {
                    color: match player {
                      Player::X => Color::rgb(1.0, 0.0, 0.0),
                      Player::O => Color::rgb(0.0, 0.1, 0.0),
                    },
                    custom_size: Some(Vec2::new(80.0, 80.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 0.0, 100.0),
                ..default()
              },
              CellMark(player),
            ))
            .id();
        // Make the marker a child of the cell
        commands.entity(cell_entity).add_child(marker_entity);

        info!("✅ {} marker spawned at ({}, {}) with entity {:?} using color {:?} and font size {:.1}",
              player.to_char(), row, col, marker_entity, mark_color, mark_font_size);

        // Switch turns ONLY after a successful move
        board_state.current_player = board_state.current_player.opposite();
        info!("🔄 Turn switched to: {}", board_state.current_player.to_char());

        println!("Current board state:");
        for row in 0..3 {
            for col in 0..3 {
                match board_state.board[row][col] {
                    None => print!(" . "),
                    Some(Player::X) => print!(" X "),
                    Some(Player::O) => print!(" O "),
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
            game_over_events.send(GameOverEvent { winner: Some(winner) });
        }
        None => {
            // Check if board is full (draw)
            if crate::types::is_board_full(&board_state.board) {
                info!("🤝 DRAW!");
                board_state.game_over = true;
                board_state.winner = None;
                game_over_events.send(GameOverEvent { winner: None });
            }
            // If no winner and board not full, game continues
        }
    }
}
