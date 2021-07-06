mod board;
use board::BoardState;
use std::io;

fn main() {
    let mut board = BoardState::new(9, 9);
    board.setup_three_players();
    println!("{}", board);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error getting stdin");
    board.place(2, 4, 1);
    print!("{}", termion::clear::All);
    println!("{}", board);
}
