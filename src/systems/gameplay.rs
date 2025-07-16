use bevy::prelude::*;

use crate::config::{CELL_SIZE, X_COLOR, O_COLOR, MARKER_SIZE_RATIO};
use crate::components::{BoardPosition, CellMark, GameOverMessage};
use crate::resources::BoardState;
use crate::events::{PlayerMoveEvent, GameOverEvent};
use crate::types::{CellState, Player, GameResult};

pub fn apply_player_move(
  mut commands: Commands,
  mut board_state: ResMut<BoardState>,
  mut player_move_events: EventReader<PlayerMoveEvent>,
  asset_server: Res<AssetServer>,
  q_board_cells: Query<(&BoardPosition, &Transform)>,
) {
  for event in player_move_events.read() {
    info!("🎮 Received PlayerMoveEvent: row={}, col={}, player={:?}",
          event.row, event.col, event.player);

    let row = event.row;
    let col = event.col;
    let player = event.player;

    // Check if game is over or cell is occupied
    if board_state.game_over {
      info!("⏹️ Game is over, ignoring move");
      continue;
    }

    if matches!(board_state.board[row][col], CellState::Occupied(_)) {
      info!("🚫 Cell ({}, {}) is already occupied", row, col);
      continue;
    }

    // Validate it's the correct player's turn
    if player != board_state.current_player {
      info!("❌ Wrong player! Expected {:?}, got {:?}", board_state.current_player, player);
      continue;
    }

    info!("✅ Move is valid, applying...");

    // Find the correct cell's transform
    let mut mark_transform = Transform::default();
    let mut found_cell = false;

    for (pos, transform) in q_board_cells.iter() {
      if pos.row == row && pos.col == col {
        mark_transform = *transform;
        mark_transform.translation.z = 1.0;
        found_cell = true;
        info!("📍 Found cell transform at position: {:?}", mark_transform.translation);
        break;
      }
    }

    if !found_cell {
      error!("❗ Could not find cell at ({}, {})", row, col);
      continue;
    }

    // Update board state
    board_state.board[row][col] = CellState::Occupied(player);
    info!("💾 Updated board state at ({}, {}) with {:?}", row, col, player);

    // Calculate marker properties
    let mark_font_size = CELL_SIZE * MARKER_SIZE_RATIO;
    let mark_color = match player {
      Player::X => X_COLOR,
      Player::O => O_COLOR,
    };

    info!("🎨 Creating mark: '{}', size: {}, color: {:?}",
          player.to_char(), mark_font_size, mark_color);

    // Spawn the marker
    let entity = commands.spawn((
      Text2dBundle {
        text: Text::from_section(
          player.to_char().to_string(),
          TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: mark_font_size,
            color: mark_color,
          },
        ),
        transform: mark_transform,
        ..default()
      },
      CellMark(player),
      BoardPosition { row, col },
    ));

    info!("✨ Spawned mark entity: {:?}", entity.id());

    // 🔑 CRITICAL: Switch turns ONLY after a successful move
    board_state.current_player = board_state.current_player.opposite();
    info!("🔄 Turn switched to {:?}", board_state.current_player);
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
      info!("🏆 Game won by {:?}", winner);
      board_state.game_over = true;
      board_state.winner = Some(winner);
      game_over_events.send(GameOverEvent::Win(winner));
    }
    GameResult::Draw => {
      info!("🤝 Game ended in a draw");
      board_state.game_over = true;
      board_state.winner = None;
      game_over_events.send(GameOverEvent::Draw);
    }
    GameResult::InProgress => {
      //  No turn switching here!
      // Turns are switched in apply_player_move after successful moves
    }
  }
}
