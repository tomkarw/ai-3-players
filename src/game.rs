use crate::board::BoardState;
use crate::heuristics::greedy_heuristic;
use crate::players::{AiPlayer, Human, Player};

pub(crate) struct Game {
    board_state: BoardState,
    players: Vec<Box<dyn Player>>,
}

impl Game {
    pub(crate) fn new(player_types: Vec<String>, minimax_depth: usize) -> Result<Self, String> {
        if player_types.len() != 3 {
            return Err("Wrong number of arguments".to_owned());
        }
        let mut players: Vec<Box<dyn Player>> = Vec::with_capacity(3);
        for player in player_types.iter() {
            let player_str = player.as_str();
            match player_str {
                "human" => players.push(Box::new(Human::new())),
                "greedy_ai" => players.push(Box::new(AiPlayer::new(minimax_depth, greedy_heuristic))),
                x => return Err(format!("'{}' is not valid player type", x)),
            }
        }
        Ok(Game {
            board_state: BoardState::new(9, 9),
            players,
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
