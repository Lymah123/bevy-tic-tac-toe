use bevy::prelude::*;
use crate::resources::BoardState;

pub fn debug_everything_system(
  keys: Res<ButtonInput<KeyCode>>,
  mouse: Res<ButtonInput<MouseButton>>,
  board_state: Res<BoardState>,
) {
  // Test keyboard
  for key in keys.get_just_pressed() {
    println!("DEBUG KEY: {:?}", key);
  }

  // Test mouse
  if mouse.just_pressed(MouseButton::Left) {
    println!("DEBUG MOUSE: Left click!");
  }

  // Test board state
  if board_state.is_changed() {
    println!("DEBUG BOARD: Game over: {}, Player: {:?}", board_state.game_over, board_state.current_player);
  }
}
