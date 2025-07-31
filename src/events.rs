use crate::types::Player;
use bevy::prelude::*;

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
