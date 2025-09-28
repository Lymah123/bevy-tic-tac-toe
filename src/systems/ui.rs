use bevy::prelude::*;

use crate::components::{CellMark, GameOverMessage, RestartButton};
use crate::config::{FONT_SIZE_TITLE, LINE_COLOR, O_COLOR, X_COLOR};
use crate::events::GameOverEvent;
use crate::resources::{BoardState, GameStats};
use crate::types::Player;

pub fn display_game_over_ui(
    mut commands: Commands,
    mut game_over_events: EventReader<GameOverEvent>,
    mut game_stats: ResMut<GameStats>,
    mut board_state: ResMut<BoardState>,
) {
    for event in game_over_events.iter() {
        board_state.game_over = true;

        let message_text = match event.winner {
            Some(player) => {
                match player {
                    Player::X => game_stats.x_wins += 1,
                    Player::O => game_stats.o_wins += 1,
                }
                format!("Player {} wins! (Press R to restart)", player.to_char())
            }
            None => {
                game_stats.draws += 1;
                "It's a draw! (Press R to restart)".to_string()
            }
        };
        game_stats.total_games += 1;

        info!("Game Over: {}", message_text);
        info!(
            "Current Stats: X Wins: {}, O Wins: {}, Draws: {}, Total: {}",
            game_stats.x_wins, game_stats.o_wins, game_stats.draws, game_stats.total_games
        );

        let text_color = match event.winner {
            Some(Player::X) => X_COLOR,
            Some(Player::O) => O_COLOR,
            None => LINE_COLOR,
        };

        // Spawn game over message with restart instructions
        commands.spawn((
            TextBundle {
                text: Text::from_section(
                    message_text,
                    TextStyle {
                        font: Handle::default(),
                        font_size: FONT_SIZE_TITLE,
                        color: text_color,
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    margin: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(50.0),
                        bottom: Val::Auto,
                    },
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            GameOverMessage,
        ));
    }
}

pub fn handle_restart_button(
    keys: Res<Input<KeyCode>>,
    mut board_state: ResMut<BoardState>,
    mut commands: Commands,
    game_over_messages: Query<Entity, With<GameOverMessage>>,
    cell_marks: Query<Entity, With<CellMark>>,
    restart_buttons: Query<Entity, With<RestartButton>>,
) {
    if keys.just_pressed(KeyCode::R) {
        info!("🔄 Restarting game...");

        // Reset board state
        board_state.board = [[None; 3]; 3];
        board_state.current_player = Player::X;
        board_state.game_over = false;
        board_state.winner = None;

        // Clean up UI elements
        for entity in game_over_messages.iter() {
            commands.entity(entity).despawn();
        }

        // Remove all X and O marks
        for entity in cell_marks.iter() {
            commands.entity(entity).despawn();
        }

        // Remove any remaining restart buttons
        for entity in restart_buttons.iter() {
            commands.entity(entity).despawn_recursive();
        }

        info!("✅ Game restarted - Player X's turn");
    }
}
