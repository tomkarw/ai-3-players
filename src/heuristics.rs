use crate::board::BoardState;

pub(crate) fn minimax<F>(
    board: &mut BoardState,
    depth: usize,
    maximizing_player: usize,
    current_player: usize,
    num_players: usize,
    turns_passed: usize,
    heuristic: &F,
    mut alpha: i32,
    mut beta: i32,
) -> i32
where
    F: Fn(&BoardState, usize) -> i32,
{
    // max depth reached or game has ended
    if depth == 0 || turns_passed == num_players {
        return heuristic(&board, current_player);
    }

    let mut best_eval = i32::MIN;
    let valid_moves = board.valid_moves(current_player);
    let next_player = (current_player + 1) % num_players;

    // handle case when player has no valid moves (skip him and continue evaluation)
    if valid_moves.is_empty() {
        return minimax(
            board,
            depth - 1,
            maximizing_player,
            next_player,
            num_players,
            turns_passed + 1,
            heuristic,
            alpha,
            beta,
        );
    }

    for (row, col) in valid_moves {
        let flipped = board.place(row, col, current_player);
        let evaluation = minimax(
            board,
            depth - 1,
            maximizing_player,
            next_player,
            num_players,
            0,
            heuristic,
            alpha,
            beta,
        );

        for ((row, col), player) in flipped {
            board.set(row, col, player);
        }

        // update best possible evaluation for maximizing player
        if current_player == maximizing_player {
            if evaluation > best_eval {
                best_eval = evaluation;
            }
        } else if evaluation < best_eval {
            best_eval = evaluation;
        }

        // alpha-betta pruning
        if current_player == maximizing_player {
            if evaluation > alpha {
                alpha = evaluation;
                if beta <= alpha {
                    break;
                }
            }
        } else if evaluation < beta {
            beta = evaluation;
            if beta <= alpha {
                break;
            }
        }
    }

    0
}

pub(crate) fn greedy_heuristic(board: &BoardState, player: usize) -> i32 {
    board.board.iter().filter(|&&x| x == player).count() as i32
}

const WEIGHTED_SUM_TABLE: [i32; 81] = [
    60, -30, 25, 25, 25, 25, 25, -30, 60, -30, -40, -25, -25, -25, -25, -25, -40, -30, 25, -25, 1,
    1, 1, 1, 1, -25, 25, 25, -25, 1, 1, 1, 1, 1, -25, 25, 25, -25, 1, 1, 1, 1, 1, -25, 25, 25, -25,
    1, 1, 1, 1, 1, -25, 25, 25, -25, 1, 1, 1, 1, 1, -25, 25, -30, -40, -25, -25, -25, -25, -25,
    -40, -30, 60, -30, 25, 25, 25, 25, 25, -30, 60,
];

pub(crate) fn weighted_sum_heuristic(board: &BoardState, player: usize) -> i32 {
    board
        .board
        .iter()
        .enumerate()
        .filter(|&(_, &p)| p == player)
        .fold(0, |sum, (i, _)| sum + WEIGHTED_SUM_TABLE[i])
}
