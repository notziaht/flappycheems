use flappycheems::run_game;
use std::process;

fn main() {
    if let Err(e) = run_game() {
        eprintln!("Game error: {}", e);
        process::exit(1);
    }
}
