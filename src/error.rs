//! Error handling for the Flappy Cheems game.
//!
//! This module provides comprehensive error types and utilities for handling
//! various failure scenarios that can occur during game execution.

use thiserror::Error;

/// Comprehensive error type for all game-related failures.
///
/// This enum covers all possible error conditions that can occur during
/// game initialization, execution, and shutdown, providing detailed context
/// for debugging and error reporting.
#[derive(Error, Debug)]
pub enum GameError {
    /// Errors originating from the bracket-lib game engine.
    #[error("Bracket-lib error: {0}")]
    BracketLib(#[from] Box<dyn std::error::Error + Send + Sync>),

    /// Invalid screen dimension parameters.
    #[error("Invalid screen dimensions: width={width}, height={height}")]
    InvalidScreenDimensions {
        /// Screen width that caused the error.
        width: i32,
        /// Screen height that caused the error.
        height: i32,
    },

    /// Player position is outside valid game boundaries.
    #[error("Player position out of bounds: x={x}, y={y}")]
    PlayerOutOfBounds {
        /// X coordinate of the invalid position.
        x: i32,
        /// Y coordinate of the invalid position.
        y: f32,
    },

    /// Game asset file could not be loaded.
    #[error("Asset loading failed: {path}")]
    AssetLoadingFailed {
        /// Path to the asset that failed to load.
        path: String,
    },
}

/// Convenient Result type alias for game operations.
///
/// This type alias simplifies error handling throughout the codebase
/// by providing a consistent return type for fallible operations.
pub type GameResult<T> = Result<T, GameError>;
