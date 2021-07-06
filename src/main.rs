mod board;
use board::BoardState;
use termion::color;

fn main() {
    let board = BoardState::new(9, 9);
    println!("{}", board);
}
