mod board;
mod game;
mod heuristics;
mod players;

use game::Game;
use std::env;

fn main() -> Result<(), String> {
    let mut game = Game::new(env::args().skip(1).collect(), 0)?;
    let winner = game.start();
    println!("winner is {}", winner);
    Ok(())
}
