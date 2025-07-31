use bevy::prelude::*;

use crate::components::BoardPosition;
use crate::config::{BACKGROUND_COLOR, BOARD_SIZE, CELL_SIZE, LINE_COLOR, LINE_THICKNESS};

pub fn setup_game(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    // Camera
    commands.spawn(Camera2dBundle::default());
    clear_color.0 = BACKGROUND_COLOR;

    // Grid lines (NO BoardPosition components!)
    // Vertical lines
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

    println!("🏗️ Creating ONLY 9 cells with BoardPosition...");

    // ONLY cells get BoardPosition - NOTHING ELSE!
    for row in 0..3 {
        for col in 0..3 {
            let x = (col as f32 - 1.0) * CELL_SIZE;
            let y = (1.0 - row as f32) * CELL_SIZE;

            println!("📍 Cell ({},{}) at ({:.1}, {:.1})", row, col, x, y);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 1.0, 0.0, 0.1), // Green tint for debugging
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..default()
                },
                BoardPosition { row, col },
            ));
        }
    }

    println!("✅ Created exactly 9 cells");
}
