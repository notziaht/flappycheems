//! Player character implementation.
//!
//! This module handles all aspects of the player character, including:
//! - Physics simulation (gravity, velocity, movement)
//! - Animation management
//! - Rendering and visual effects
//! - Collision detection and boundary checking

use crate::config::{
    CHEEMS_ANIMATION_FRAMES, CHEEMS_ANIMATION_FRAME_COUNT, PHYSICS_FLAP_STRENGTH, PHYSICS_GRAVITY,
    PHYSICS_MAX_VELOCITY, PLAYER_MOVEMENT_SPEED, SCREEN_HEIGHT,
};
use bracket_lib::prelude::*;

/// Represents a 2D position in the game world.
///
/// Uses integer X coordinates for discrete horizontal movement
/// and floating-point Y coordinates for smooth vertical physics.
#[derive(Debug, Clone)]
pub struct Position {
    /// Horizontal position in game units.
    pub x: i32,
    /// Vertical position with sub-pixel precision.
    pub y: f32,
}

impl Position {
    /// Creates a new position with the specified coordinates.
    ///
    /// # Arguments
    ///
    /// * `x` - Horizontal position in game units
    /// * `y` - Vertical position with sub-pixel precision
    pub fn new(x: i32, y: f32) -> Self {
        Self { x, y }
    }
}

/// Represents vertical velocity with built-in physics constraints.
///
/// Automatically clamps velocity values to prevent unrealistic movement speeds
/// and provides methods for applying physics effects like gravity.
#[derive(Debug, Clone)]
pub struct Velocity {
    /// Current velocity value, positive = downward, negative = upward.
    value: f32,
}

impl Velocity {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn get(&self) -> f32 {
        self.value
    }

    pub fn set(&mut self, value: f32) {
        self.value = value.clamp(-PHYSICS_MAX_VELOCITY * 2.0, PHYSICS_MAX_VELOCITY);
    }

    pub fn apply_gravity(&mut self) {
        if self.value < PHYSICS_MAX_VELOCITY {
            self.value += PHYSICS_GRAVITY;
        }
    }
}

impl Default for Velocity {
    fn default() -> Self {
        Self::new(0.0)
    }
}

/// Manages character animation frame cycling.
///
/// Handles automatic frame advancement and sprite index mapping
/// for smooth character animation during gameplay.
#[derive(Debug, Clone)]
pub struct AnimationFrame {
    /// Current frame index in the animation sequence.
    current: usize,
}

impl AnimationFrame {
    pub fn new() -> Self {
        Self { current: 0 }
    }

    pub fn advance(&mut self) {
        self.current = (self.current + 1) % CHEEMS_ANIMATION_FRAME_COUNT;
    }

    pub fn get_sprite_index(&self) -> u16 {
        CHEEMS_ANIMATION_FRAMES[self.current]
    }
}

impl Default for AnimationFrame {
    fn default() -> Self {
        Self::new()
    }
}

/// The player character with complete state management.
///
/// Combines position, physics, and animation into a cohesive entity
/// that can be updated, rendered, and queried for game logic.
#[derive(Debug, Clone)]
pub struct Player {
    /// Current world position.
    position: Position,
    /// Current movement velocity.
    velocity: Velocity,
    /// Animation state.
    animation: AnimationFrame,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            position: Position::new(x, y as f32),
            velocity: Velocity::default(),
            animation: AnimationFrame::default(),
        }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn x(&self) -> i32 {
        self.position.x
    }

    pub fn y(&self) -> f32 {
        self.position.y
    }

    pub fn velocity(&self) -> f32 {
        self.velocity.get()
    }

    pub fn is_out_of_bounds(&self) -> bool {
        self.position.y as i32 >= SCREEN_HEIGHT || self.position.y < 0.0
    }

    pub fn update_physics(&mut self) {
        self.velocity.apply_gravity();
        self.position.y += self.velocity.get();
        self.position.x += PLAYER_MOVEMENT_SPEED;

        if self.position.y < 0.0 {
            self.position.y = 0.0;
            self.velocity.set(0.0);
        }

        self.animation.advance();
    }

    pub fn flap(&mut self) {
        self.velocity.set(PHYSICS_FLAP_STRENGTH);
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF::new(0.0, self.position.y),
            1,
            Degrees::new(0.0),
            PointF::new(2.0, 2.0),
            WHITE,
            NAVY,
            self.animation.get_sprite_index(),
        );
        ctx.set_active_console(0);
    }
}
