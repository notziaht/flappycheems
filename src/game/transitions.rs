use crate::game::{GameMode, GameState};
use bracket_lib::prelude::*;

const UI_INSTRUCTIONS_Y: i32 = 0;
const UI_SCORE_Y: i32 = 1;

pub fn play_game(state: &mut GameState, ctx: &mut BTerm) {
    render_game_background(ctx);

    if state.timer_should_tick(ctx.frame_time_ms) {
        state.player_mut().update_physics();
    }

    handle_player_input(state, ctx);
    render_game_elements(state, ctx);
    render_ui(state, ctx);

    update_game_logic(state);
    check_game_over_conditions(state);
}

fn render_game_background(ctx: &mut BTerm) {
    ctx.cls_bg(NAVY);
}

fn handle_player_input(state: &mut GameState, ctx: &mut BTerm) {
    if let Some(VirtualKeyCode::Space) = ctx.key {
        state.player_mut().flap();
    }
}

fn render_game_elements(state: &mut GameState, ctx: &mut BTerm) {
    state.player().render(ctx);
    state.obstacle().render(ctx, state.player().x());
}

fn render_ui(state: &GameState, ctx: &mut BTerm) {
    ctx.print(0, UI_INSTRUCTIONS_Y, "Press SPACE to flap.");
    ctx.print(0, UI_SCORE_Y, format!("Score: {}", state.score()));
}

fn update_game_logic(state: &mut GameState) {
    if state.player().x() > state.obstacle().x() {
        state.increment_score();
        state.spawn_new_obstacle();
    }
}

fn check_game_over_conditions(state: &mut GameState) {
    let player_out_of_bounds = state.player().is_out_of_bounds();
    let player_hit_obstacle = state.obstacle().has_collision_with(state.player());

    if player_out_of_bounds || player_hit_obstacle {
        state.set_mode(GameMode::GameOver);
    }
}
