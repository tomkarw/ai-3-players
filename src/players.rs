use crate::board::{BoardState, Move};
use std::io;
use std::io::Write;

pub(crate) trait Player {
    fn get_move(&self, board: &BoardState, turn: usize) -> (usize, usize);
    fn print_board(&self, board: &BoardState);
    fn print_no_moves(&self);
}

pub(crate) struct Human {}

impl Player for Human {
    fn get_move(&self, board: &BoardState, player: usize) -> (usize, usize) {
        let mut input = String::with_capacity(4);
        print!("Provide coords as '<int>, <int>': ");
        // loop till valid move
        loop {
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let (row, col) = if let Some((row, col)) = input.split_once(",") {
                match (row.trim().parse(), col.trim_end_matches("\n").trim().parse()) {
                    (Ok(row), Ok(col)) => (row, col),
                    (_, _) => {
                        print!("Can't parse input. Try again: ");
                        continue;
                    }
                }
            } else {
                print!("Expected two arguments separated by comma. Try again: ");
                continue;
            };
            match board.validate_placing(row, col, player) {
                Move::Valid {..} => return (row, col),
                Move::Invalid(reason) => {
                    print!("{} Try again: ", reason);
                },
            }
        };
    }

    fn print_board(&self, board: &BoardState) {
        print!("{}", board);
        // println!("{}'s turn.", "nobody");
    }

    fn print_no_moves(&self) {
        // println!("");
    }
}
