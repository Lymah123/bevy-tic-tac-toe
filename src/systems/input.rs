use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::BoardPosition;
use crate::resources::BoardState;
use crate::events::PlayerMoveEvent;
use crate::config::{CELL_SIZE, LINE_THICKNESS};
use crate::types::CellState;

pub fn handle_mouse_clicks(
  mut commands: Commands,
  mouse_button_input: Res<ButtonInput<MouseButton>>,
  q_windows: Query<&Window, With<PrimaryWindow>>,
  q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
  q_board_cells: Query<(&BoardPosition, &Transform)>,
  board_state: Res<BoardState>,
  mut player_move_events: EventWriter<PlayerMoveEvent>,
) {
  if mouse_button_input.just_pressed(MouseButton::Left) {
    info!("🖱️ Mouse LEFT clicked!");

    let Some(window) = q_windows.iter().next() else {
      error!("Primary window not found!");
      return;
    };

    let Some(screen_pos) = window.cursor_position() else {
      info!("❌ No cursor position found");
      return;
    };

    info!("📍 Screen position: {:?}", screen_pos);

    let (camera, camera_transform) = q_camera.single();

    let Some(world_pos) = camera.viewport_to_world(camera_transform, screen_pos)
        .map(|ray| ray.origin.truncate()) else {
          info!("❌ Failed to convert to world position");
          return;
        };

    info!("🌍 World position: {:?}", world_pos);

    if board_state.game_over {
      info!("⏹️ Game is over! Press R to restart.");
      return;
    }

    info!("🎮 Current player: {:?}", board_state.current_player);
    info!("🎯 Checking {} board cells for click...", q_board_cells.iter().count());

    for (board_pos, transform) in q_board_cells.iter() {
      let cell_min_x = transform.translation.x - (CELL_SIZE - LINE_THICKNESS ) / 2.0;
      let cell_max_x = transform.translation.x + (CELL_SIZE - LINE_THICKNESS) / 2.0;
      let cell_min_y = transform.translation.y - (CELL_SIZE - LINE_THICKNESS) / 2.0;
      let cell_max_y = transform.translation.y + (CELL_SIZE - LINE_THICKNESS) / 2.0;

      if world_pos.x >= cell_min_x && world_pos.x <= cell_max_x &&
        world_pos.y >= cell_min_y && world_pos.y <= cell_max_y {

          info!("✅ Hit detected on cell ({}, {})", board_pos.row, board_pos.col);

          if let CellState::Occupied(_) = board_state.board[board_pos.row][board_pos.col] {
            info!("❌ Cell ({}, {}) is already occupied.", board_pos.row, board_pos.col);
            return;
          }

          info!("📤 Sending PlayerMoveEvent for cell ({}, {}) with player {:?}",
                board_pos.row, board_pos.col, board_state.current_player);

          player_move_events.send(PlayerMoveEvent {
            row: board_pos.row,
            col: board_pos.col,
            player: board_state.current_player,
          });
          return;
        }
    }

    info!("❌ Click was outside all cells");
  }
}
