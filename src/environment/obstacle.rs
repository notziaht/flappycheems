use crate::config::{
    OBSTACLE_BASE_GAP_SIZE, OBSTACLE_GAP_MAX_Y, OBSTACLE_GAP_MIN_Y, OBSTACLE_MIN_GAP_SIZE,
    SCREEN_HEIGHT, SCREEN_WIDTH,
};
use crate::player::Player;
use bracket_lib::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct ObstacleGap {
    center_y: i32,
    half_size: i32,
}

impl ObstacleGap {
    pub fn new(center_y: i32, half_size: i32) -> Self {
        Self {
            center_y,
            half_size,
        }
    }

    pub fn top_boundary(&self) -> i32 {
        self.center_y - self.half_size
    }

    pub fn bottom_boundary(&self) -> i32 {
        self.center_y + self.half_size
    }

    pub fn contains_point(&self, y: i32) -> bool {
        y >= self.top_boundary() && y <= self.bottom_boundary()
    }
}

#[derive(Debug, Clone)]
pub struct Obstacle {
    x: i32,
    gap: ObstacleGap,
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        let mut rng = RandomNumberGenerator::new();
        let gap_center = rng.range(OBSTACLE_GAP_MIN_Y, OBSTACLE_GAP_MAX_Y);
        let gap_half_size = i32::max(OBSTACLE_MIN_GAP_SIZE, OBSTACLE_BASE_GAP_SIZE - score) / 2;

        Self {
            x,
            gap: ObstacleGap::new(gap_center, gap_half_size),
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn has_collision_with(&self, player: &Player) -> bool {
        if player.x() != self.x {
            return false;
        }

        let player_y = player.y() as i32;
        !self.gap.contains_point(player_y)
    }

    pub fn render(&self, ctx: &mut BTerm, player_x: i32) {
        let screen_x = self.x - player_x;

        if !(0..SCREEN_WIDTH).contains(&screen_x) {
            return;
        }

        self.render_ground(ctx);
        self.render_obstacle_pillars(ctx, screen_x);
    }

    fn render_ground(&self, ctx: &mut BTerm) {
        for x in 0..SCREEN_WIDTH {
            ctx.set(x, SCREEN_HEIGHT - 1, WHITE, WHITE, to_cp437('#'));
        }
    }

    fn render_obstacle_pillars(&self, ctx: &mut BTerm, screen_x: i32) {
        for y in 0..self.gap.top_boundary() {
            ctx.set(screen_x, y, RED, BLACK, 179);
        }

        for y in self.gap.bottom_boundary()..SCREEN_HEIGHT - 1 {
            ctx.set(screen_x, y, RED, BLACK, 179);
        }
    }
}
