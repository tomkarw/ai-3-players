from copy import deepcopy

from board import Board


def minimax(board_state: Board, depth: int, maximizing_player: int, current_player: int, num_players: int, turns_passed: int, heuristic):
    # max depth reached or game ended
    if depth == 0 or turns_passed == num_players:
        return heuristic(board_state, current_player)

    # TODO(tkarwwoski): not sure about efficiency of copying whole board
    #   in Haskell we would use persistent data structures, in Rust we would copy semi-cheap Vec<u8>
    max_eval = None
    valid_moves = board_state.valid_moves(current_player)
    next_player = (current_player + 1) % num_players

    # handle case when player has no valid moves (skip him, but continue minimax)
    if not valid_moves:
        return minimax(deepcopy(board_state), depth - 1, maximizing_player, next_player, num_players, turns_passed + 1, heuristic)

    for move_row, move_col in valid_moves:
        new_board = deepcopy(board_state)
        new_board.place(move_row, move_col, current_player)
        evaluation = minimax(new_board, depth - 1, maximizing_player, next_player, num_players, 0, heuristic)
        f = max if current_player == maximizing_player else min
        max_eval = evaluation if max_eval is None else f(max_eval, evaluation)

    return max_eval
