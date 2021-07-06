use crate::board::BoardState;

pub(crate) trait Player {
    fn get_move(&self, board: &BoardState, turn: usize) -> (usize, usize);
    fn print_board(&self, board: &BoardState);
    fn print_no_moves(&self);
}

pub(crate) struct Human {}

impl Player for Human {
    fn get_move(&self, board: &BoardState, turn: usize) -> (usize, usize) {
        todo!()
    }

    fn print_board(&self, board: &BoardState) {
        print!("{}", board);
        // println!("{}'s turn.", "nobody");
    }

    fn print_no_moves(&self) {
        // println!("");
    }
}
