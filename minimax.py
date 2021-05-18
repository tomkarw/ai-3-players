from typing import Union

from board import Board, EMPTY


def minimax(
    board: Board,
    depth: int,
    maximizing_player: int,
    current_player: int,
    num_players: int,
    turns_passed: int,
    heuristic,
    alpha: Union[int, None],
    beta: Union[int, None],
):
    # max depth reached or game ended
    if depth == 0 or turns_passed == num_players:
        return heuristic(board, current_player)

    max_eval = None
    valid_moves = board.valid_moves(current_player)
    next_player = (current_player + 1) % num_players

    # handle case when player has no valid moves (skip him, but continue minimax)
    if not valid_moves:
        return minimax(board, depth - 1, maximizing_player, next_player, num_players, turns_passed + 1, heuristic,
                       alpha, beta)

    for move_row, move_col in valid_moves:
        board.place(move_row, move_col, current_player)
        evaluation = minimax(board, depth - 1, maximizing_player, next_player, num_players, 0, heuristic, alpha, beta)
        board.board[move_row][move_col] = EMPTY
        f = max if current_player == maximizing_player else min
        max_eval = evaluation if max_eval is None else f(max_eval, evaluation)

        # alpha-beta pruning
        if current_player == maximizing_player:
            alpha = max(alpha, evaluation) if alpha is not None else evaluation
            if beta is not None and beta <= alpha:
                break
        else:
            beta = min(beta, evaluation) if beta is not None else evaluation
            if alpha is not None and beta <= alpha:
                break

    return max_eval
