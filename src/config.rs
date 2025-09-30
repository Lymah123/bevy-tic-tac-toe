use bevy::prelude::Color;

// Board Layout
pub const LINE_THICKNESS: f32 = 5.0;
pub const BOARD_SIZE: f32 = 300.0;
pub const CELL_SIZE: f32 = BOARD_SIZE / 3.0;

// Colors
pub const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
pub const LINE_COLOR: Color = Color::rgb(0.2, 0.2, 0.2);
pub const X_COLOR: Color = Color::rgb(0.8, 0.2, 0.2);
pub const O_COLOR: Color = Color::rgb(0.2, 0.2, 0.8);

// Positioning
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
#[allow(dead_code)]
pub const BOARD_OFFSET_X: f32 = 0.0;
#[allow(dead_code)]
pub const BOARD_OFFSET_Y: f32 = 50.0;

// Marker styling
#[allow(dead_code)]
pub const MARKER_THICKNESS: f32 = 8.0;
pub const MARKER_SIZE_RATIO: f32 = 0.7;

// Timing
#[allow(dead_code)]
pub const AI_MOVE_DELAY: f32 = 1.0; // Seconds before AI moves
#[allow(dead_code)]
pub const ANIMATION_DURATION: f32 = 0.3;
#[allow(dead_code)]
pub const GAME_OVER_DISPLAY_TIME: f32 = 3.0;

// AI difficulty settings
#[allow(dead_code)]
pub const EASY_AI_RANDOM_CHANCE: f32 = 0.8; // 80% random moves
#[allow(dead_code)]
pub const MEDIUM_AI_DEPTH: i32 = 3;
#[allow(dead_code)]
pub const HARD_AI_DEPTH: i32 = 9; // Full depth

// Text styling
pub const FONT_SIZE_TITLE: f32 = 32.0;
#[allow(dead_code)]
pub const FONT_SIZE_STATUS: f32 = 24.0;

// Hover effects
#[allow(dead_code)]
pub const HOVER_ALPHA: f32 = 0.7;
#[allow(dead_code)]
pub const CLICK_SCALE: f32 = 0.95;

// Grid spacing
#[allow(dead_code)]
pub const GRID_PADDING: f32 = 20.0;

// Game states
#[allow(dead_code)]
pub const WIN_HIGHLIGHT_COLOR: Color = Color::rgb(0.9, 0.9, 0.2);

// Additional constants for AI vs AI mode
#[allow(dead_code)]
pub const AI_VS_AI_MOVE_DELAY: f32 = 1.5; // Slower moves for visibility
#[allow(dead_code)]
pub const AI_THINKING_INDICATOR_DELAY: f32 = 0.5; // Show "thinking" indicator
