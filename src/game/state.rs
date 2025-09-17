use crate::config::{FRAME_DURATION_MS, SCREEN_WIDTH};
use crate::environment::Obstacle;
use crate::game::menu::{game_over_menu, main_menu};
use crate::game::transitions::play_game;
use crate::game::GameMode;
use crate::player::Player;
use bracket_lib::prelude::*;

#[derive(Debug, Clone)]
pub struct Score {
    value: i32,
}

impl Score {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn get(&self) -> i32 {
        self.value
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }
}

impl Default for Score {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct GameTimer {
    accumulated_time: f32,
}

impl GameTimer {
    pub fn new() -> Self {
        Self {
            accumulated_time: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.accumulated_time += delta_time;
    }

    pub fn should_tick(&mut self) -> bool {
        if self.accumulated_time >= FRAME_DURATION_MS {
            self.accumulated_time = 0.0;
            true
        } else {
            false
        }
    }

    pub fn reset(&mut self) {
        self.accumulated_time = 0.0;
    }
}

impl Default for GameTimer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct GameState {
    player: Player,
    timer: GameTimer,
    obstacle: Obstacle,
    mode: GameMode,
    score: Score,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            player: Player::new(5, 25),
            timer: GameTimer::new(),
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            mode: GameMode::default(),
            score: Score::new(),
        }
    }

    pub fn player(&self) -> &Player {
        &self.player
    }

    pub fn player_mut(&mut self) -> &mut Player {
        &mut self.player
    }

    pub fn obstacle(&self) -> &Obstacle {
        &self.obstacle
    }

    pub fn obstacle_mut(&mut self) -> &mut Obstacle {
        &mut self.obstacle
    }

    pub fn mode(&self) -> GameMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: GameMode) {
        self.mode = mode;
    }

    pub fn score(&self) -> i32 {
        self.score.get()
    }

    pub fn increment_score(&mut self) {
        self.score.increment();
    }

    pub fn timer_should_tick(&mut self, delta_time: f32) -> bool {
        self.timer.update(delta_time);
        self.timer.should_tick()
    }

    pub fn reset_game(&mut self) {
        self.player = Player::new(5, SCREEN_WIDTH / 2);
        self.timer.reset();
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.mode = GameMode::Playing;
        self.score.reset();
    }

    pub fn spawn_new_obstacle(&mut self) {
        self.obstacle = Obstacle::new(self.player.x() + SCREEN_WIDTH, self.score.get());
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl bracket_lib::prelude::GameState for GameState {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => main_menu(self, ctx),
            GameMode::GameOver => game_over_menu(self, ctx),
            GameMode::Playing => play_game(self, ctx),
        }
    }
}
