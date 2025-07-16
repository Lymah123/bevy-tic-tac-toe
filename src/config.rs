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
pub const BOARD_OFFSET_X: f32 = 0.0;
pub const BOARD_OFFSET_Y: f32 = 50.0;

// Marker styling
pub const MARKER_THICKNESS: f32 = 8.0;
pub const MARKER_SIZE_RATIO: f32 = 0.7;

// Timing
pub const AI_MOVE_DELAY: f32 = 1.0; // Seconds before AI moves
pub const ANIMATION_DURATION: f32 = 0.3;
pub const GAME_OVER_DISPLAY_TIME: f32 = 3.0;

// AI difficulty settings
pub const EASY_AI_RANDOM_CHANCE: f32 = 0.8; // 80% random moves
pub const MEDIUM_AI_DEPTH: i32 = 3;
pub const HARD_AI_DEPTH: i32 = 9; // Full depth

// Text styling
pub const FONT_SIZE_TITLE: f32 = 32.0;
pub const FONT_SIZE_STATUS: f32 = 24.0;
pub const FONT_SIZE_BUTTON: f32 = 18.0;

// Button dimensions
pub const BUTTON_WIDTH: f32 = 120.0;
pub const BUTTON_HEIGHT: f32 = 40.0;
pub const BUTTON_MARGIN: f32 = 10.0;

// Hover effects
pub const HOVER_ALPHA: f32 = 0.7;
pub const CLICK_SCALE: f32 = 0.95;

// Grid spacing
pub const GRID_PADDING: f32 = 20.0;

// Game states
pub const WIN_HIGHLIGHT_COLOR: Color = Color::rgb(0.9, 0.9, 0.2);
