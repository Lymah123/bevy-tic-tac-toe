use bevy::prelude::*;

use crate::config::{FONT_SIZE_TITLE, BACKGROUND_COLOR, LINE_COLOR, X_COLOR, O_COLOR};
use crate::events::GameOverEvent;
use crate::resources::{BoardState, GameStats};
use crate::types::{Player, GameResult, CellState};
use crate::components::{CellMark, RestartButton, GameOverMessage};

pub fn display_game_over_ui(
  mut commands: Commands,
  mut game_over_events: EventReader<GameOverEvent>,
  asset_server: Res<AssetServer>,
  mut game_stats: ResMut<GameStats>,
  mut board_state: ResMut<BoardState>,
) {
  for event in game_over_events.read() {
    board_state.game_over = true;

    let message_text = match event {
      GameOverEvent::Win(player) => {
        match player {
          Player::X => game_stats.x_wins += 1,
          Player::O => game_stats.o_wins += 1,
        }
        format!("Player {} wins!", player.to_char())
      },
      GameOverEvent::Draw => {
        game_stats.draws += 1;
        "It's a draw!".to_string()
      },
    };
    game_stats.total_games += 1;

    info!("Game Over: {}", message_text);
    info!("Current Stats: X Wins: {}, O Wins: {}, Draws: {}, Total: {}", game_stats.x_wins, game_stats.o_wins, game_stats.draws, game_stats.total_games);

    let text_color = match event {
      GameOverEvent::Win(Player::X) => X_COLOR,
      GameOverEvent::Win(Player::O) => O_COLOR,
      GameOverEvent::Draw => LINE_COLOR,
    };

    // Spawn game over message
    commands.spawn((
      TextBundle {
        text: Text::from_section(message_text, TextStyle {
          font: asset_server.load("fonts/FiraSans-Bold.ttf"),
          font_size: FONT_SIZE_TITLE,
          color: text_color,
        }),
        style: Style {
          position_type: PositionType::Absolute,
          top: Val::Px(50.0),
          left: Val::Px(0.0),
          right: Val::Px(0.0),
          justify_content: JustifyContent::Center,
          ..default()
        },
        ..default()
      },
      GameOverMessage,
    ));

    // Spawn restart button
    commands.spawn((
      ButtonBundle {
        style: Style {
          width: Val::Px(150.0),
          height: Val::Px(50.0),
          position_type: PositionType::Absolute,
          top: Val::Px(120.0),
          left: Val::Px(50.0),
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          ..default()
        },
        background_color: Color::rgb(0.2, 0.7, 0.2).into(),
        ..default()
      },
      RestartButton,
    )).with_children(|parent| {
      parent.spawn(TextBundle::from_section(
        "Press R to Restart!",
        TextStyle {
          font: asset_server.load("fonts/FiraSans-Bold.ttf"),
          font_size: 16.0,
          color: Color::WHITE,
        },
      ));
    });
  }
}

pub fn handle_restart_button(
  keys: Res<ButtonInput<KeyCode>>,
  mut board_state: ResMut<BoardState>,
  mut commands: Commands,
  game_over_messages: Query<Entity, With<GameOverMessage>>,
  cell_marks: Query<Entity, With<CellMark>>,
  restart_buttons: Query<Entity, With<RestartButton>>,
) {
  // Test if ANY key is being detected
  for key in keys.get_just_pressed() {
    println!("🎮 Key detected: {:?}", key);
  }

  if keys.just_pressed(KeyCode::KeyR) {
    println!("🔄 R KEY PRESSED IN UI.RS - RESTART DETECTED!");
    info!("🔄 R key pressed - attempting restart...");

    // Reset board state
    board_state.board = [[CellState::Empty; 3]; 3];
    board_state.current_player = Player::X;
    board_state.game_over = false;

    println!("🎮 BOARD STATE RESET - Game Over: {}", board_state.game_over);
    info!("🎮 Board state reset - Current player: {:?}, Game over: {}",
          board_state.current_player, board_state.game_over);

    // Remove all game over messages
    let message_count = game_over_messages.iter().count();
    for entity in game_over_messages.iter() {
      commands.entity(entity).despawn();
    }
    info!("🗑️ Removed {} game over messages", message_count);

    // Remove all X and O marks from the board
    let mark_count = cell_marks.iter().count();
    for entity in cell_marks.iter() {
      commands.entity(entity).despawn();
    }
    info!("🗑️ Removed {} cell marks", mark_count);

    // Remove restart buttons
    let button_count = restart_buttons.iter().count();
    for entity in restart_buttons.iter() {
      commands.entity(entity).despawn();
    }
    info!("🗑️ Removed {} restart buttons", button_count);

    println!("✅ RESTART COMPLETE!");
    info!("✅ Game restarted! Current player is now: {:?}", board_state.current_player);
  }
}
