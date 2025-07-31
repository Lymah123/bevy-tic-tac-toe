use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::BoardPosition;
use crate::config::CELL_SIZE;
use crate::events::PlayerMoveEvent;
use crate::resources::BoardState;
use crate::types::CellState;

pub fn handle_mouse_clicks(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    q_board_cells: Query<(&BoardPosition, &Transform)>,
    board_state: Res<BoardState>,
    mut player_move_events: EventWriter<PlayerMoveEvent>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        let Some(window) = q_windows.iter().next() else {
            return;
        };

        let Some(screen_pos) = window.cursor_position() else {
            return;
        };

        let (camera, camera_transform) = q_camera.single();

        let Some(world_pos) = camera
            .viewport_to_world(camera_transform, screen_pos)
            .map(|ray| ray.origin.truncate())
        else {
            return;
        };

        if board_state.game_over {
            return;
        }

        // Check all cells for hits
        for (board_pos, transform) in q_board_cells.iter() {
            let cell_half_size = CELL_SIZE / 2.0;
            let cell_min_x = transform.translation.x - cell_half_size;
            let cell_max_x = transform.translation.x + cell_half_size;
            let cell_min_y = transform.translation.y - cell_half_size;
            let cell_max_y = transform.translation.y + cell_half_size;

            if world_pos.x >= cell_min_x
                && world_pos.x <= cell_max_x
                && world_pos.y >= cell_min_y
                && world_pos.y <= cell_max_y
            {
                if let CellState::Occupied(_) = board_state.board[board_pos.row][board_pos.col] {
                    return;
                }

                player_move_events.send(PlayerMoveEvent {
                    row: board_pos.row,
                    col: board_pos.col,
                    player: board_state.current_player,
                });
                return;
            }
        }
    }
}
