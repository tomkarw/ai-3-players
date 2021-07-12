mod board;
mod game;
mod heuristics;
mod players;

use game::Game;
use std::env;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.iter().len() != 5 {
        return Err("Wrong number of arguments".to_owned());
    }
    let minimax_depth = if let Ok(depth) = args.get(4).unwrap().parse() {
        depth
    } else {
        return Err("Unable to parse minimax depth argument".to_owned());
    };

    let mut game = Game::new(args.into_iter().skip(1).take(3).collect(), minimax_depth)?;
    let winner = game.start();
    println!("winner is {}", winner);
    Ok(())
}
