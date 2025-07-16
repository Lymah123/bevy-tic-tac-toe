use bevy::prelude::*;
use crate::types::Player;

// Event fired when a player makes a move
#[derive(Event, Debug)]
pub struct PlayerMoveEvent {
  pub row: usize,
  pub col: usize,
  pub player: Player,
}

// Event fired when the game ends
#[derive(Event, Debug)]
pub enum GameOverEvent {
  Win(Player),
  Draw,
}
