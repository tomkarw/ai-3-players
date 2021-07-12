use std::io;
use std::io::Write;

use crate::board::{BoardState, Move};
use crate::heuristics::minimax;

const NUM_PLAYERS: usize = 3;

pub(crate) trait Player {
    fn get_move(&self, board: &BoardState, turn: usize) -> (usize, usize);
    fn print_board(&self, board: &BoardState);
    fn print_no_moves(&self);
}

pub(crate) struct Human {}

impl Human {
    pub(crate) fn new() -> Self {
        Human {}
    }
}

impl Player for Human {
    fn get_move(&self, board: &BoardState, player: usize) -> (usize, usize) {
        let mut input = String::with_capacity(4);
        print!("Provide coords as '<int>, <int>': ");
        // loop till valid move is provided
        loop {
            io::stdout().flush().unwrap();
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let (row, col) = if let Some((row, col)) = input.split_once(",") {
                match (
                    row.trim().parse(),
                    col.trim_end_matches('\n').trim().parse(),
                ) {
                    // we're only interested if everything is parsed correctly
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
                Move::Valid { .. } => return (row, col),
                Move::Invalid(reason) => {
                    print!("{} Try again: ", reason);
                }
            }
        }
    }

    fn print_board(&self, board: &BoardState) {
        print!("{}", board);
        // println!("{}'s turn.", "nobody");
    }

    fn print_no_moves(&self) {
        println!("No available moves. Skipping turn.");
    }
}

pub(crate) struct AiPlayer<F>
where
    F: Fn(&BoardState, usize) -> i32,
{
    minimax_depth: usize,
    heuristic: F,
}

impl<F> AiPlayer<F>
where
    F: Fn(&BoardState, usize) -> i32,
{
    pub(crate) fn new(minimax_depth: usize, heuristic: F) -> Self {
        AiPlayer {
            minimax_depth,
            heuristic,
        }
    }
}

impl<F> Player for AiPlayer<F>
where
    F: Fn(&BoardState, usize) -> i32,
{
    fn get_move(&self, board: &BoardState, player: usize) -> (usize, usize) {
        // we clone board to keep keep BoardState in get_move API immutable
        // we do it once, minimax alg will work on this mutable copy
        let mut board = board.clone();
        let valid_moves = board.valid_moves(player);

        let mut best_eval = i32::MIN;
        let mut best_move = *valid_moves.get(0).unwrap();
        let next_player = (player + 1) % NUM_PLAYERS;

        if self.minimax_depth == 0 {
            return best_move;
        }

        for (row, col) in valid_moves {
            board.place(row, col, player);
            let evaluation = minimax(
                &mut board,
                self.minimax_depth - 1,
                player,
                next_player,
                NUM_PLAYERS,
                0,
                &self.heuristic,
                i32::MIN,
                i32::MAX,
            );
            if evaluation > best_eval {
                best_eval = evaluation;
                best_move = (row, col);
            }
        }

        best_move
    }

    fn print_board(&self, board: &BoardState) {
        print!("{}", board);
    }

    fn print_no_moves(&self) {
        println!("No available moves. Skipping turn.");
    }
}
