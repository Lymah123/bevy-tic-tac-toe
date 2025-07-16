use bevy::prelude::*;
use crate::types::Player;

#[derive(Component)]
pub struct BoardPosition {
    pub row: usize,
    pub col: usize,
}

#[derive(Component)]
pub struct CellMark(pub Player);

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct GameOverMessage;
