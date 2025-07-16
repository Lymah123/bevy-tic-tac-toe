use bevy::prelude::*;

use crate::config::{BOARD_SIZE, CELL_SIZE, LINE_THICKNESS, BACKGROUND_COLOR, LINE_COLOR};
use crate::components::BoardPosition;
use crate::resources::BoardState;
use crate::types::{CellState, Player};

pub fn setup_game(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
  commands.spawn(Camera2dBundle::default());

  clear_color.0 = BACKGROUND_COLOR;

  commands.spawn(SpriteBundle {
    sprite: Sprite {
      color: LINE_COLOR,
      custom_size: Some(Vec2::new(LINE_THICKNESS, BOARD_SIZE)),
      ..default()
    },
    transform: Transform::from_xyz(-CELL_SIZE / 2.0, 0.0, 0.0),
    ..default()
  });
  commands.spawn(SpriteBundle {
    sprite: Sprite {
      color: LINE_COLOR,
      custom_size: Some(Vec2::new(LINE_THICKNESS, BOARD_SIZE)),
      ..default()
    },
    transform: Transform::from_xyz(CELL_SIZE / 2.0, 0.0, 0.0),
    ..default()
  });

  // Horizontal lines

  commands.spawn(SpriteBundle {
    sprite: Sprite {
      color: LINE_COLOR,
      custom_size: Some(Vec2::new(BOARD_SIZE, LINE_THICKNESS)),
      ..default()
    },
    transform: Transform::from_xyz(0.0, -CELL_SIZE / 2.0, 0.0),
    ..default()
  });

  commands.spawn(SpriteBundle {
    sprite: Sprite {
      color: LINE_COLOR,
      custom_size: Some(Vec2::new(BOARD_SIZE, LINE_THICKNESS)),
      ..default()
    },
    transform: Transform::from_xyz(0.0, CELL_SIZE / 2.0, 0.0),
    ..default()
  });

  for row in 0..3 {
    for col in 0..3 {
      let x = (col as f32 - 1.0) * CELL_SIZE;
      let y = (row as f32 - 1.0) * CELL_SIZE;

      commands.spawn((
        SpriteBundle {
          sprite: Sprite {
            color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(CELL_SIZE - LINE_THICKNESS, CELL_SIZE - LINE_THICKNESS)),
            ..default()
          },
          transform: Transform::from_xyz(x, y, 0.0),
          ..default()
        },
        BoardPosition { row, col },
      ));
    }
  }
}

