//! Game configuration constants and settings.
//!
//! This module centralizes all game constants, physics parameters, and configuration
//! settings to ensure consistency and easy tuning of game behavior.

use std::time::Duration;

// Display Settings
/// Width of the game screen in terminal characters.
pub const SCREEN_WIDTH: i32 = 50;

/// Height of the game screen in terminal characters.
pub const SCREEN_HEIGHT: i32 = 30;

/// Duration between physics updates in milliseconds.
pub const FRAME_DURATION_MS: f32 = 75.0;

/// Size of sprite tiles in pixels.
pub const TILE_SIZE: i32 = 32;

/// Size of console tiles for rendering.
pub const CONSOLE_TILE_SIZE: i32 = 12;

// Physics Constants
/// Gravitational acceleration applied to the player each frame.
pub const PHYSICS_GRAVITY: f32 = 0.2;

/// Maximum downward velocity the player can achieve.
pub const PHYSICS_MAX_VELOCITY: f32 = 2.0;

/// Upward velocity applied when the player flaps.
pub const PHYSICS_FLAP_STRENGTH: f32 = -1.0;

/// Horizontal movement speed of the player in units per frame.
pub const PLAYER_MOVEMENT_SPEED: i32 = 1;

// Obstacle Configuration
/// Minimum gap size between obstacle segments.
pub const OBSTACLE_MIN_GAP_SIZE: i32 = 2;

/// Base gap size that decreases with score for progressive difficulty.
pub const OBSTACLE_BASE_GAP_SIZE: i32 = 10;

/// Minimum Y coordinate for obstacle gap center.
pub const OBSTACLE_GAP_MIN_Y: i32 = 5;

/// Maximum Y coordinate for obstacle gap center.
pub const OBSTACLE_GAP_MAX_Y: i32 = 20;

// Animation Settings
/// Sprite indices for Cheems character animation frames.
pub const CHEEMS_ANIMATION_FRAMES: [u16; 6] = [64, 1, 2, 3, 2, 1];

/// Total number of animation frames for the Cheems character.
pub const CHEEMS_ANIMATION_FRAME_COUNT: usize = 6;

// Asset Paths
/// Path to the font sprite sheet.
pub const ASSET_PATH_FONT: &str = "../resources/flappycheems.png";

/// Path to the game sprite sheet.
pub const ASSET_PATH_SPRITES: &str = "../resources/flappycheems.png";

// Game Metadata
/// Display title for the game window.
pub const GAME_TITLE: &str = "Flappy Cheems Adventure";

/// Configuration structure for game settings.
///
/// Provides a centralized way to manage game configuration with
/// sensible defaults and type safety.
#[derive(Debug, Clone, Copy)]
pub struct GameConfig {
    /// Screen width in characters.
    pub screen_width: i32,
    /// Screen height in characters.
    pub screen_height: i32,
    /// Frame duration for physics updates.
    pub frame_duration: Duration,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            frame_duration: Duration::from_millis(FRAME_DURATION_MS as u64),
        }
    }
}
