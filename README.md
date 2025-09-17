# Flappy Cheems

A simple implementation of the classic Flappy Bird game featuring Cheems, built on Rust and the bracket-lib game engine.


## Building and Running

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Build

```bash
# Debug build
cargo build

# Optimized release build
cargo build --release
```

### Run

```bash
# Run in debug mode
cargo run

# Run optimized version
cargo run --release
```

### Development

```bash
# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt

# Generate documentation
cargo doc --open
```

## Gameplay

- **Space**: Flap to gain altitude
- **P**: Play/restart game (in menus)
- **Q**: Quit game

Navigate Cheems through the obstacles by pressing space to flap. Each obstacle you pass increases your score and slightly increases the difficulty.

## Project Structure

```
src/
├── config.rs          # Game constants and configuration
├── error.rs           # Error types and handling
├── lib.rs             # Library entry point and main game loop
├── main.rs            # Binary entry point
├── environment/       # Game environment (obstacles, collisions)
│   ├── mod.rs
│   └── obstacle.rs
├── game/              # Game state and logic
│   ├── mod.rs
│   ├── game_mode.rs   # Game state enumeration
│   ├── menu.rs        # Menu rendering and input
│   ├── state.rs       # Main game state management
│   └── transitions.rs # Game loop and physics
└── player/            # Player character
    ├── mod.rs
    └── player_entity.rs # Player physics and rendering
```