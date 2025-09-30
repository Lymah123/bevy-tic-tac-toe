use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::BoardPosition;
use crate::config::CELL_SIZE;
use crate::events::PlayerMoveEvent;
use crate::resources::BoardState;

pub fn handle_mouse_clicks(
    mouse_button_input: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    q_board_cells: Query<(&BoardPosition, &Transform)>,
    board_state: Res<BoardState>,
    mut player_move_events: EventWriter<PlayerMoveEvent>,
) {
    // Check if clicks are detected at all
    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("🖱️ LEFT MOUSE CLICK DETECTED!");

        // Check if game is over
        if board_state.game_over {
            info!("🚫 Game is over, ignoring click");
            return;
        }

        info!(
            "✅ Game not over, current player: {:?}",
            board_state.current_player
        );

        // Get window
        let Ok(window) = q_windows.get_single() else {
            error!("❌ No primary window found!");
            return;
        };

        // Get cursor position
        let Some(cursor_position) = window.cursor_position() else {
            error!("❌ No cursor position found!");
            return;
        };

        info!("📍 Raw cursor position: {:?}", cursor_position);

        // Get camera
        let Ok((camera, camera_transform)) = q_camera.get_single() else {
            error!("❌ No Camera2d found!");
            return;
        };

        info!("📷 Camera found at: {:?}", camera_transform.translation());

        // Convert cursor to world coordinates
        let Some(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position)
        else {
            error!("❌ Failed to convert cursor to world coordinates!");
            return;
        };

        info!("🌍 World position: {:?}", world_position);

        // Check all board cells
        let cell_count = q_board_cells.iter().count();
        info!("🔢 Total board cells found: {}", cell_count);

        let mut found_cell = false;
        for (board_pos, transform) in q_board_cells.iter() {
            let cell_center = transform.translation.truncate();
            let distance = world_position.distance(cell_center);
            let half_cell = CELL_SIZE / 2.0;

            info!(
                "🏠 Cell ({},{}) at {:?}, distance: {:.2}, half_cell: {:.2}",
                board_pos.row, board_pos.col, cell_center, distance, half_cell
            );

            // Check if click is within cell bounds
            if world_position.x >= cell_center.x - half_cell
                && world_position.x <= cell_center.x + half_cell
                && world_position.y >= cell_center.y - half_cell
                && world_position.y <= cell_center.y + half_cell
            {
                info!("🎯 CLICKED ON CELL ({},{})!", board_pos.row, board_pos.col);

                // Check if cell is already occupied
                if board_state.board[board_pos.row][board_pos.col].is_some() {
                    info!(
                        "🚫 Cell ({},{}) already occupied",
                        board_pos.row, board_pos.col
                    );
                    return;
                }

                info!(
                    "✅ Sending PlayerMoveEvent for ({},{})",
                    board_pos.row, board_pos.col
                );
                player_move_events.send(PlayerMoveEvent {
                    position: (board_pos.row, board_pos.col),
                });

                found_cell = true;
                break;
            }
        }

        if !found_cell {
            info!("❌ Click not on any cell");
        }
    }
}
