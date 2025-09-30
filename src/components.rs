use crate::types::Player;
use bevy::prelude::*;

#[derive(Component)]
#[allow(dead_code)]
pub struct GameBoard;

#[derive(Component)]
#[allow(dead_code)]
pub struct Cell(pub usize, pub usize);

#[derive(Component)]
pub struct BoardPosition {
    pub row: usize,
    pub col: usize,
}

#[derive(Component)]
pub struct CellMark(#[allow(dead_code)] pub Player);

#[derive(Component)]
#[allow(dead_code)]
pub struct GameUI;

#[derive(Component)]
#[allow(dead_code)]
pub struct StatusText;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
#[allow(dead_code)]
pub struct BackButton;

#[derive(Component)]
#[allow(dead_code)]
pub struct DifficultyButton(pub crate::types::Difficulty);

#[derive(Component)]
#[allow(dead_code)]
pub struct GameModeButton(pub crate::types::GameMode);

#[derive(Component)]
pub struct GameOverMessage;

#[derive(Component)]
#[allow(dead_code)]
pub struct AIPlayer {
    pub difficulty: crate::types::Difficulty,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct PlayerTurnIndicator;

#[derive(Component)]
#[allow(dead_code)]
pub struct GameTimer;

#[derive(Component)]
#[allow(dead_code)]
pub struct MainMenuButton;
