use crate::game::GameState;
use bracket_lib::prelude::*;

const MENU_TITLE_Y: i32 = 5;
const MENU_OPTIONS_START_Y: i32 = 8;
const MENU_SCORE_Y: i32 = 6;

pub fn main_menu(state: &mut GameState, ctx: &mut BTerm) {
    render_menu_background(ctx);

    ctx.print_centered(MENU_TITLE_Y, "Welcome to Flappy Cheems.");
    ctx.print_centered(MENU_OPTIONS_START_Y, "▌P▐ Play game");
    ctx.print_centered(MENU_OPTIONS_START_Y + 1, "▌Q▐ Quit game");

    handle_menu_input(state, ctx);
}

pub fn game_over_menu(state: &mut GameState, ctx: &mut BTerm) {
    render_menu_background(ctx);

    ctx.print_centered(MENU_TITLE_Y, "Game Over");
    ctx.print_centered(
        MENU_SCORE_Y,
        format!("You earned {} points.", state.score()),
    );
    ctx.print_centered(MENU_OPTIONS_START_Y, "▌P▐ Play again");
    ctx.print_centered(MENU_OPTIONS_START_Y + 1, "▌Q▐ Quit game");

    handle_menu_input(state, ctx);
}

fn render_menu_background(ctx: &mut BTerm) {
    ctx.cls();
}

fn handle_menu_input(state: &mut GameState, ctx: &mut BTerm) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::P => state.reset_game(),
            VirtualKeyCode::Q => ctx.quitting = true,
            _ => {}
        }
    }
}
