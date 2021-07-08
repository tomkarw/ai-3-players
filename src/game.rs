use crate::board::BoardState;
use crate::players::{Human, Player};

pub(crate) struct Game {
    board_state: BoardState,
    players: Vec<Box<dyn Player>>,
}

impl Game {
    pub(crate) fn new(players: Vec<usize>, minimax_depth: usize) -> Self {
        Game {
            board_state: BoardState::new(9, 9),
            players: vec![Box::new(Human {}), Box::new(Human {}), Box::new(Human {})],
        }
    }

    pub(crate) fn start(&mut self) -> usize {
        self.board_state.setup_three_players();
        loop {
            let mut players_stuck = true;
            for (turn, player) in self.players.iter().enumerate() {
                let turn = turn + 1;
                player.print_board(&self.board_state);
                if self.board_state.has_valid_move(turn as usize) {
                    players_stuck = false;
                    let (row, col) = player.get_move(&self.board_state, turn);
                    self.board_state.place(row, col, turn as usize);
                } else {
                    player.print_no_moves()
                }
            }
            if players_stuck {
                return self.board_state.get_winner();
            }
        }
    }
}
