use crate::types::Player;

#[derive(Debug, Clone)]
pub struct PlayerMoveEvent {
    pub position: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct GameOverEvent {
    pub winner: Option<Player>,
}
