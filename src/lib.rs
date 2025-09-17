//! # Flappy Cheems
//!
//! A professional implementation of the classic Flappy Bird game featuring Cheems,
//! built with Rust and the bracket-lib game engine.
//!
//! ## Features
//!
//! - Smooth character animation with physics-based movement
//! - Progressive difficulty scaling
//! - Clean, modular architecture with strong type safety
//! - Comprehensive error handling
//! - Memory-efficient rendering
//!
//! ## Architecture
//!
//! The game is structured with clear separation of concerns:
//!
//! - **Player**: Handles character physics, animation, and rendering
//! - **Environment**: Manages obstacles and collision detection
//! - **Game**: Coordinates game states, scoring, and transitions
//! - **Config**: Centralizes all game constants and configuration
//! - **Error**: Provides comprehensive error handling
//!
//! ## Usage
//!
//! ```no_run
//! use flappycheems::run_game;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     run_game()?;
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod environment;
pub mod error;
pub mod game;
pub mod player;

use crate::error::{GameError, GameResult};
use crate::game::GameState;
use bracket_lib::prelude::*;
use config::{
    ASSET_PATH_FONT, ASSET_PATH_SPRITES, CONSOLE_TILE_SIZE, GAME_TITLE, SCREEN_HEIGHT,
    SCREEN_WIDTH, TILE_SIZE,
};

/// Runs the Flappy Cheems game.
///
/// This function initializes the game context, creates the game state,
/// and starts the main game loop.
///
/// # Errors
///
/// Returns a `GameError` if:
/// - The game context cannot be created
/// - Asset files cannot be loaded
/// - The main game loop encounters an error
///
/// # Examples
///
/// ```no_run
/// use flappycheems::run_game;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     run_game()?;
///     Ok(())
/// }
/// ```
pub fn run_game() -> GameResult<()> {
    let context = create_game_context()?;
    let game_state = GameState::new();

    main_loop(context, game_state).map_err(GameError::BracketLib)?;
    Ok(())
}

/// Creates and configures the game context.
///
/// This function sets up the bracket-lib terminal with the appropriate
/// console layers, fonts, and display settings for the game.
///
/// # Errors
///
/// Returns a `GameError::BracketLib` if the terminal builder fails
/// to create the context, typically due to missing asset files.
fn create_game_context() -> GameResult<BTerm> {
    let context = BTermBuilder::new()
        .with_title(GAME_TITLE)
        .with_font(ASSET_PATH_FONT, TILE_SIZE, TILE_SIZE)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, ASSET_PATH_SPRITES)
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, ASSET_PATH_SPRITES)
        .with_tile_dimensions(CONSOLE_TILE_SIZE, CONSOLE_TILE_SIZE)
        .build()
        .map_err(GameError::BracketLib)?;

    Ok(context)
}
