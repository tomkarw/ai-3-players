mod board;
mod game;
mod players;

use game::Game;

fn main() {
    let mut game = Game::new(vec![0, 0, 0], 0);
    let winner = game.start();
    println!("winner is {}", winner);
}
