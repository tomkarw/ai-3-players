use crate::board::BoardState;
use rayon::iter::{ParallelIterator, IntoParallelRefIterator};

pub(crate) fn minimax<F>(
    board: &BoardState,
    heuristic: F,
    player: usize,
    num_players: usize,
    minimax_depth: usize,
) -> (usize, usize)
where
    F: Fn(&BoardState, usize) -> i32 + std::marker::Sync,
{
    // we clone board to keep keep BoardState in get_move API immutable
    // we do it once, minimax alg will work on this mutable copy
    let valid_moves = board.valid_moves(player);

    let (best_row, best_col) = *valid_moves.get(0).unwrap();
    let next_player = (player + 1) % num_players;

    if minimax_depth == 0 {
        return (best_row, best_col);
    }

    let ((row, col), _) = valid_moves.par_iter().map(|(row, col)| {
        let mut board = board.clone();
        board.place(*row, *col, player);
        let evaluation = minimax_base(
            &board,
            minimax_depth - 1,
            player,
            next_player,
            num_players,
            0,
            &heuristic,
            i32::MIN,
            i32::MAX,
        );
        ((row, col), evaluation)
    }).reduce(|| ((&best_row, &best_col), i32::MIN), |(best_placement, best_eval), (current_placement, current_eval)| {
        if current_eval > best_eval {
            (current_placement, current_eval)
        } else {
            (best_placement, best_eval)
        }
    });

    (*row, *col)
}

fn minimax_base<F>(
    board: &BoardState,
    depth: usize,
    maximizing_player: usize,
    current_player: usize,
    num_players: usize,
    turns_passed: usize,
    heuristic: &F,
    alpha: i32,
    beta: i32,
) -> i32
where
    F: Fn(&BoardState, usize) -> i32 + std::marker::Sync,
{
    // max depth reached or game has ended
    if depth == 0 || turns_passed == num_players {
        return heuristic(board, current_player);
    }

    let valid_moves = board.valid_moves(current_player);
    let next_player = (current_player + 1) % num_players;

    // handle case when player has no valid moves (skip him and continue evaluation)
    if valid_moves.is_empty() {
        return minimax_base(
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

    valid_moves.par_iter().map(|(row, col)| {
        // let flipped = board.place(row, col, current_player);
        let mut board = board.clone();
        board.place(*row, *col, current_player);
        minimax_base(
            &board,
            depth - 1,
            maximizing_player,
            next_player,
            num_players,
            0,
            heuristic,
            alpha,
            beta,
        )

        // // alpha-betta pruning
        // if current_player == maximizing_player {
        //     if evaluation > alpha {
        //         alpha = evaluation;
        //         if beta <= alpha {
        //             break;
        //         }
        //     }
        // } else if evaluation < beta {
        //     beta = evaluation;
        //     if beta <= alpha {
        //         break;
        //     }
        // }
    }).reduce(|| i32::MIN, |best_eval: i32, evaluation: i32| {
        // update best possible evaluation for maximizing player
        if current_player == maximizing_player {
            if evaluation > best_eval {
                return evaluation;
            }
        } else if evaluation < best_eval {
            return evaluation;
        }
        best_eval
    });

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
