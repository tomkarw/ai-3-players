use crate::board::BoardState;
use crate::players::{Human, Player};

pub(crate) struct Game {
    board_state: BoardState,
    players: Vec<Box<dyn Player>>,
}

impl Game {
    pub(crate) fn new(player_types: Vec<String>, minimax_depth: usize) -> Result<Self, String> {
        if player_types.len() != 3 {
            return Err("Wrong number of arguments".to_owned());
        }
        let mut players = Vec::with_capacity(3);
        for player in player_types.iter() {
            let player_str = player.as_str();
            match player_str {
                "human" => players.push(Human {}),
                x => return Err(format!("'{}' is not valid player type", x)),
            }
        }
        Ok(Game {
            board_state: BoardState::new(9, 9),
            players: vec![Box::new(Human {}), Box::new(Human {}), Box::new(Human {})],
        })
    }

    pub(crate) fn start(&mut self) -> usize {
        self.board_state.setup_three_players();
        loop {
            let mut players_stuck = true;
            for (player_num, player) in self.players.iter().enumerate() {
                let player_num = player_num + 1;
                player.print_board(&self.board_state);
                if self.board_state.has_valid_move(player_num as usize) {
                    players_stuck = false;
                    let (row, col) = player.get_move(&self.board_state, player_num);
                    self.board_state.place(row, col, player_num as usize);
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
